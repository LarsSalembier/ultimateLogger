use ultimate_logger::log_level::LogLevel;

fn main() {
    let mut logger = ultimate_logger::Logger::new_to_file(
        "First logger".to_string(),
        LogLevel::Warning,
        String::from("log.txt"),
        true,
    );

    logger.log(LogLevel::Trace, "This is a trace message");
    logger.log(LogLevel::Debug, "This is a debug message");
    logger.log(LogLevel::Info, "This is an info message");
    logger.log(LogLevel::Warning, "This is a warning message");
    logger.log(LogLevel::Error, "This is an error message");
    logger.log(LogLevel::Critical, "This is a critical message");

    let mut logger_2 = ultimate_logger::Logger::new_to_file(
        "Second logger".to_string(),
        LogLevel::Trace,
        String::from("log.txt"),
        true,
    );

    logger_2.log(LogLevel::Trace, "This is a trace message");
    logger_2.log(LogLevel::Debug, "This is a debug message");
    logger_2.log(LogLevel::Info, "This is an info message");
    logger_2.log(LogLevel::Warning, "This is a warning message");
    logger_2.log(LogLevel::Error, "This is an error message");
    logger_2.log(LogLevel::Critical, "This is a critical message");
}
