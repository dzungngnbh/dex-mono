use log::LevelFilter;

pub fn init() {
    let log_level = match std::env::var("RUST_LOG") {
        Ok(level_str) => match level_str.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Debug,
        },
        Err(_) => LevelFilter::Debug,
    };

    logforth::builder()
        .dispatch(|d| {
            d.filter(log_level)
                .append(logforth::append::Stdout::default())
        })
        .apply();
}
