use lazy_static::lazy_static;
use slog::{o, Drain, Logger};
use slog_async;
use slog_json::Json;
use slog_stdlog::StdLog;
use std::sync::Mutex;
pub struct SimpleLogger {
    logger: Logger,
}

impl SimpleLogger {
    pub fn new() -> Self {

        let logger = slog::Logger::root(
            Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
            o!(),
        );

        Self { logger }
    }

    pub fn init(&self) {
        let _ = slog_stdlog::init().unwrap();
    }

    pub fn get_logger(&self) -> &Logger {
        &self.logger
    }
}

lazy_static! {
    pub static ref LOGGER: SimpleLogger = SimpleLogger::new();
}

pub fn init() {
    LOGGER.init();
}
