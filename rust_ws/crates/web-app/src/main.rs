#![feature(lazy_cell)]

mod api;
mod backend;
mod errors;
mod layout;
mod lib;
mod models;
mod pages;
mod routes;

use anyhow::Result;
use axum::{Extension, Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use log::LevelFilter;
use logforth::append;
use logforth::layout::JsonLayout;
use std::str::FromStr;
use std::{env, net::SocketAddr, path::PathBuf};
use tower::builder::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::{
    backend::Backend,
    lib::env::Env,
    pages::four0four_index, // Contains session_context extractor
};

// Use mimalloc as the global allocator for better performance
use crate::pages::pulse;
use api::{auth, ws};
use mimalloc::MiMalloc;
use models::account;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Load TLS configuration from certificate and key files
async fn get_rust_tsl_config() -> Result<RustlsConfig> {
    Ok(RustlsConfig::from_pem_file(
        PathBuf::from("keys/cert.pem"),
        PathBuf::from("keys/key.pem"),
    )
    .await?)
}

/// Configure and set up application routes
fn setup_routes(env: &Env) -> Router {
    let mut app = Router::new()
        // API http
        .nest("/api/auth", auth::routes())
        .nest("/api/account", account::routes())
        // API websockets
        .route("/ws", get(ws::ws_handler))
        .route("/ws_traders", get(ws::traders_ws_handler))
        // Page routes
        .route("/c/:component_name", get(pages::components::components))
        .route("/pulse", get(pulse::Page::index))
        .route("/trade/:symbol", get(pages::trade::Page::index))
        // Component routes
        .route("/404", get(four0four_index))
        // /trade/{:market}
        // order_book, market_trades, current_price, 24h change, 24h volume
        // auth
        // .route("/login", get(login::index))
        // .route("/signup", get(signup::index))
        // .route("/signup", post(signup::signup))
        // Static file serving
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/dist", ServeDir::new("dist"));

    // Enable development-only routes
    if env.env == "dev" {
        info!("Environment DEV");
        info!("Prototypes are enabled at /prototypes");
        app = app.nest_service("/prototypes", ServeDir::new("prototypes"))
    }

    app
}

/// Configure middleware layers
fn setup_middleware(app: Router, backend: Backend) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(Extension(backend))
            .layer(CompressionLayer::new())
            .layer(TraceLayer::new_for_http())
            .layer(CookieManagerLayer::new()),
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    lib::log::init();

    // Load environment configuration
    let env = Env::get_env()?;
    let backend = Backend::new(&env);

    // Set up TLS configuration
    let config = get_rust_tsl_config().await?;

    // Initialize router with routes
    let app = setup_routes(&env);

    // Add middleware layers
    let app = setup_middleware(app, backend);

    // Configure server address and port
    let port = if env.env == "dev" { 3000 } else { 443 };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on http://{addr}");

    // Start the server
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
