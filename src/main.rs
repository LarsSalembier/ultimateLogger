use ultimate_logger::log_level;
use ultimate_logger::Logger;

fn main() {
    let mut logger = Logger::new(log_level::LogLevel::Info);

    logger.trace("This is a trace message");
    logger.debug("This is a debug message");
    logger.info("This is an info message");
    logger.warning("This is a warning message");
    logger.error("This is an error message");
    logger.critical("This is a critical message");

    logger.set_min_level(log_level::LogLevel::Warning);

    logger.trace("This is a trace message");
    logger.debug("This is a debug message");
    logger.info("This is an info message");
    logger.warning("This is a warning message");
    logger.error("This is an error message");
    logger.critical("This is a critical message");

    logger.set_write_to_console(false);

    logger.trace("This is a trace message");

    logger.set_filepath("log.txt".to_string());

    logger.trace("This is a trace message");
    logger.debug("This is a debug message");
    logger.info("This is an info message");
    logger.warning("This is a warning message");
    logger.error("This is an error message");
    logger.critical("This is a critical message");
}
