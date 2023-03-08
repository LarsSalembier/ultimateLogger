//! Log level enum and functions

use colored::{ColoredString, Colorize};

/// Log level enum
///
/// # Examples
///
/// ```
/// use ultimate_logger::log_level;
///
/// let log_level_info = log_level::LogLevel::Info;
/// let log_level_debug = log_level::LogLevel::Debug;
/// let log_level_trace = log_level::LogLevel::Trace;
/// let log_level_warning = log_level::LogLevel::Warning;
/// let log_level_error = log_level::LogLevel::Error;
/// let log_level_critical = log_level::LogLevel::Critical;
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    /// Returns the log level as a string
    ///
    /// # Examples
    ///
    /// ```
    /// use ultimate_logger::log_level;
    ///
    /// assert_eq!(log_level::LogLevel::Info.to_string(), "info");
    /// assert_eq!(log_level::LogLevel::Debug.to_string(), "debug");
    /// assert_eq!(log_level::LogLevel::Trace.to_string(), "trace");
    /// assert_eq!(log_level::LogLevel::Warning.to_string(), "warning");
    /// assert_eq!(log_level::LogLevel::Error.to_string(), "error");
    /// assert_eq!(log_level::LogLevel::Critical.to_string(), "critical");
    /// ```
    pub fn to_string(&self) -> &str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Critical => "critical",
        }
    }

    /// Returns the log level as a colored string
    ///
    /// # Examples
    ///
    /// ```
    /// use ultimate_logger::log_level;
    ///
    /// assert_eq!(LogLevel::Info.color_string("info"), "info".clear());
    /// assert_eq!(LogLevel::Debug.color_string("debug"), "debug".dimmed());
    /// assert_eq!(LogLevel::Trace.color_string("trace"), "trace".underline());
    /// assert_eq!(LogLevel::Warning.color_string("warning"), "warning".yellow());
    /// assert_eq!(LogLevel::Error.color_string("error"), "error".red());
    /// assert_eq!(LogLevel::Critical.color_string("critical"), "critical".red().bold());
    /// ```
    pub fn color_string(&self, str: &str) -> ColoredString {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_order() {
        assert!(LogLevel::Info < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Trace);
        assert!(LogLevel::Trace < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Critical);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(LogLevel::Trace.to_string(), "trace");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warning.to_string(), "warning");
        assert_eq!(LogLevel::Error.to_string(), "error");
        assert_eq!(LogLevel::Critical.to_string(), "critical");
    }

    #[test]
    fn test_color_string() {
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
