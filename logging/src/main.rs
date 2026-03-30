// src/main.rs

use logging::Logger;

fn main() {
    // Initialize the logger
    let logger = Logger::new(log::LevelFilter::Debug);

    // Log messages at different levels
    logger.debug("This is a debug message");
    logger.info("This is an info message");
    logger.warn("This is a warning message");
    logger.error("This is an error message");
}