use anyhow::Result;
use minitrace::collector::Config;
use minitrace::collector::ConsoleReporter;
use minitrace::prelude::*;
use minitrace::Event;
use std::io::Write;

use shared::db::redis as rediss;

mod oracle_task;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    minitrace::set_reporter(ConsoleReporter, Config::default());
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            // Add a event to the current local span representing the log record
            Event::add_to_local_parent(record.level().as_str(), || {
                [("message".into(), record.args().to_string().into())]
            });

            // Output the log to stdout as usual
            writeln!(buf, "[{}] {}", record.level(), record.args())
        })
        // .filter_level(log::LevelFilter::Debug)
        .init();

    dotenvy::dotenv().ok();
    let redis_client = rediss::get_redis_client()?;

    // spawn tokio task with tthis loop
    let oracle_task = tokio::spawn(async move { oracle_task::run(&redis_client).await.unwrap() });

    // wait for oracle_task to finish
    oracle_task.await?;

    Ok(())
}
