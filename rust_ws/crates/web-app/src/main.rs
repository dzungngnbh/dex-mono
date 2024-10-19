#![feature(lazy_cell)]

mod account;
mod app;
mod auth;
mod backend;
mod errors;
mod lib;
mod routes;
mod ws;

use anyhow::Result;
use axum::extract::{FromRef, FromRequestParts};
use axum::response::IntoResponse;
use axum::{routing::get, Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower::builder::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::app::four0four_index;
// @dev: This contains session_context extractor, do not remove
use crate::auth::extractor;
use crate::backend::Backend;
use crate::lib::env::Env;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn get_rust_tsl_config() -> Result<RustlsConfig> {
    Ok(RustlsConfig::from_pem_file(
        PathBuf::from("keys/cert.pem"),
        PathBuf::from("keys/key.pem"),
    )
    .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let env = Env::get_env()?;
    let backend = Backend::new(&env);

    // TODO: MVP update this to use our own certs
    let config = get_rust_tsl_config().await?;
    let mut app = Router::new()
        // /api support both get components and post mutation
        // sometimes you need specific component for each page, you should consider put that component in the page
        // route instead of generalize it, easier to find and work with.
        // TODO: rate-limiting this route hard
        .nest("/api/auth", auth::routes())
        .nest("/api/account", account::routes())
        .route("/ws", get(ws::ws_handler))
        // ui route
        .route("/404", get(four0four_index))
        // /markets
        // /trade/{:market}
        // order_book, market_trades, current_price, 24h change, 24h volume
        // auth
        // .route("/login", get(login::index))
        // .route("/signup", get(signup::index))
        // .route("/signup", post(signup::signup))
        // global components
        .route("/c/:component_name", get(app::components::components))
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/dist", ServeDir::new("dist"));

    // dev only routes
    if env.env == "dev" {
        app = app.nest_service("/prototypes", ServeDir::new("prototypes"))
    }

    // add layers
    // use ServiceBuilder the layer will be hit from top to bottom https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    app = app.layer(
        ServiceBuilder::new()
            .layer(Extension(backend))
            .layer(CompressionLayer::new())
            .layer(TraceLayer::new_for_http())
            .layer(CookieManagerLayer::new()),
    );

    // fallback to 404 page

    // port
    let port = if env.env == "dev" { 3000 } else { 443 };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on {addr}");
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
