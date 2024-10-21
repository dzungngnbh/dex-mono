use anyhow::Result;
use log::info;
use minitrace::collector::Config;
use minitrace::collector::ConsoleReporter;
use minitrace::prelude::*;
use minitrace::Event;
use shared::db::redis as rediss;
use std::io::Write;

mod oracle_task;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    minitrace::set_reporter(ConsoleReporter, Config::default());
    env_logger::init();

    dotenvy::dotenv().ok();
    let redis_client = rediss::get_redis_client()?;
    info!("Starting oracle service");

    // spawn tokio task with tthis loop
    let oracle_task = tokio::spawn(async move { oracle_task::run(&redis_client).await.unwrap() });

    // wait for oracle_task to finish
    oracle_task.await?;

    Ok(())
}
