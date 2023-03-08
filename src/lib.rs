use chrono::offset::Local;
use colored::{ColoredString, Colorize};

const NAME: &str = "console";

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    fn to_string(&self) -> &str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Critical => "critical",
        }
    }

    fn color_string(&self, str: &str) -> ColoredString {
        match self {
            LogLevel::Trace => str.underline(),
            LogLevel::Debug => str.dimmed(),
            LogLevel::Info => str.clear(),
            LogLevel::Warning => str.yellow(),
            LogLevel::Error => str.red(),
            LogLevel::Critical => str.red().bold(),
        }
    }
}

pub fn log(level: LogLevel, message: &str) {
    let message = level.color_string(message);
    let level_name = level.color_string(level.to_string());

    let date_time = Local::now().format("%F %T%.3f").to_string();

    println!("[{}] [{}] [{}] {}", date_time, NAME, level_name, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        log(LogLevel::Trace, "This is a trace message");
        log(LogLevel::Debug, "This is a debug message");
        log(LogLevel::Info, "This is an info message");
        log(LogLevel::Warning, "This is a warning message");
        log(LogLevel::Error, "This is an error message");
        log(LogLevel::Critical, "This is a critical message");
    }

    #[test]
    fn test_log_level_to_string() {
        assert_eq!(LogLevel::Trace.to_string(), "trace");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warning.to_string(), "warning");
        assert_eq!(LogLevel::Error.to_string(), "error");
        assert_eq!(LogLevel::Critical.to_string(), "critical");
    }

    #[test]
    fn test_log_level_color_string() {
        assert_eq!(LogLevel::Trace.color_string("trace"), "trace".underline());
        assert_eq!(LogLevel::Debug.color_string("debug"), "debug".dimmed());
        assert_eq!(LogLevel::Info.color_string("info"), "info".clear());
        assert_eq!(
            LogLevel::Warning.color_string("warning"),
            "warning".yellow()
        );
        assert_eq!(LogLevel::Error.color_string("error"), "error".red());
        assert_eq!(
            LogLevel::Critical.color_string("critical"),
            "critical".red().bold()
        );
    }
}
