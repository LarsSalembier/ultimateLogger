use ultimate_logger::{log, LogLevel};

fn main() {
    log(LogLevel::Trace, "This is a trace message");
    log(LogLevel::Debug, "This is a debug message");
    log(LogLevel::Info, "This is an info message");
    log(LogLevel::Warning, "This is a warning message");
    log(LogLevel::Error, "This is an error message");
    log(LogLevel::Critical, "This is a critical message");
}
