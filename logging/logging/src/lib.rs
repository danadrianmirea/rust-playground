// src/lib.rs

/// A basic logger class
pub struct Logger {
    level: log::LevelFilter,
}

impl Logger {
    /// Create a new logger with a specific level
    pub fn new(level: log::LevelFilter) -> Self {
        Logger { level }
    }

    fn enabled(&self, level: log::LevelFilter) -> bool {
        self.level >= level
    }

    /// Log a message at the debug level
    pub fn debug(&self, message: &str) {
        if self.enabled(log::LevelFilter::Debug) {
            println!("DEBUG: {}", message);
        }
    }

    /// Log a message at the info level
    pub fn info(&self, message: &str) {
        if self.enabled(log::LevelFilter::Info) {
            println!("INFO: {}", message);
        }
    }

    /// Log a message at the warn level
    pub fn warn(&self, message: &str) {
        if self.enabled(log::LevelFilter::Warn) {
            println!("WARN: {}", message);
        }
    }

    /// Log a message at the error level
    pub fn error(&self, message: &str) {
        if self.enabled(log::LevelFilter::Error) {
            println!("ERROR: {}", message);
        }
    }
}

