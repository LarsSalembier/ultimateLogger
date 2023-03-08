//! Ultimate Logger is a simple logger that can write to a file and/or the console.
//!
//! # Examples
//!
//! ## Write to the console
//!
//! ```
//! use logger::Logger;
//! use logger::log_level::LogLevel;
//!
//! let mut logger = Logger::new(String::from("example"), LogLevel::Trace);
//!
//! logger.info("This is an info message");
//! logger.debug("This is a debug message");
//! logger.trace("This is a trace message");
//! logger.warning("This is a warning message");
//! logger.error("This is an error message");
//! logger.critical("This is a critical message");
//! ```
//!
//! ## Write to a file
//!
//! ```
//! use logger::Logger;
//! use logger::log_level::LogLevel;
//!
//! let mut logger = Logger::new_to_file(String::from("example"), LogLevel::Trace, String::from("log.txt"), true);
//!
//! logger.info("This is an info message");
//! logger.debug("This is a debug message");
//! logger.trace("This is a trace message");
//! logger.warning("This is a warning message");
//! logger.error("This is an error message");
//! logger.critical("This is a critical message");
//! ```
//!
//! # Features
//!
//! - Write to a file
//! - Write to the console
//! - Write to both
//! - Set a minimum log level
//! - Colored output
//! - Timestamps
//! - Multiple loggers with different names are possible

mod log_file;
pub mod log_level;

use chrono::offset;
use colored::ColoredString;
use log_file::LogFile;

/// A logger that can write to a file and/or the console.
///
/// # Examples
///
/// ## Write to the console
///
/// ```
/// use logger::Logger;
/// use logger::log_level::LogLevel;
///
/// let mut logger = Logger::new(String::from("example"), LogLevel::Trace);
///
/// logger.info("This is an info message");
/// logger.debug("This is a debug message");
/// logger.trace("This is a trace message");
/// logger.warning("This is a warning message");
/// logger.error("This is an error message");
/// logger.critical("This is a critical message");
/// ```
///
/// ## Write to a file
///
/// ```
/// use logger::Logger;
/// use logger::log_level::LogLevel;
///
/// let mut logger = Logger::new_to_file(String::from("example"), LogLevel::Trace, String::from("log.txt"), true);
///
/// logger.info("This is an info message");
/// logger.debug("This is a debug message");
/// logger.trace("This is a trace message");
/// logger.warning("This is a warning message");
/// logger.error("This is an error message");
/// logger.critical("This is a critical message");
/// ```
pub struct Logger {
    name: String,
    min_level: log_level::LogLevel,
    log_file: Option<log_file::LogFile>,
    write_to_console: bool,
    write_to_file: bool,
}

impl Logger {
    /// Creates a new logger that writes to the console.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the logger.
    /// * `min_level` - The minimum log level.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    /// use logger::log_level::LogLevel;
    ///
    /// let mut logger = Logger::new(String::from("example"), LogLevel::Trace);
    /// ```
    ///
    /// This will create a logger that writes to the console and has the name "example" and the minimum log level "Trace".
    pub fn new(name: String, min_level: log_level::LogLevel) -> Self {
        Self {
            name,
            min_level,
            log_file: None,
            write_to_console: true,
            write_to_file: false,
        }
    }

    /// Creates a new logger that writes to a file.
    /// If `write_to_console_too` is set to `true`, the logger will also write to the console.
    /// If `write_to_console_too` is set to `false`, the logger will only write to the file.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the logger.
    /// * `min_level` - The minimum log level.
    /// * `filepath` - The path to the file. If the file doesn't exist, it will be created.
    /// * `write_to_console_too` - Whether the logger should write to the console too.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    /// use logger::log_level::LogLevel;
    ///
    /// let logger = Logger::new_to_file(String::from("example"), LogLevel::Trace, String::from("log.txt"), true);
    /// ```
    ///
    /// This will create a logger that writes to the file "log.txt" and has the name "example" and the minimum log level "Trace".
    /// It will also write to the console, because `write_to_console_too` is set to `true`.
    ///
    /// # Panics
    ///
    /// This function will panic if the file can't be created or opened.
    /// This can happen if:
    /// - The file is already open in another process.
    /// - The file is a directory.
    /// - The file is read-only.
    /// - The file doesn't exist and the parent directory is read-only.
    /// - The file doesn't exist and the parent directory doesn't exist.
    /// - The file doesn't exist and the parent directory is a file.
    /// - The file doesn't exist and the parent directory is a symlink.
    /// - ...
    pub fn new_to_file(
        name: String,
        min_level: log_level::LogLevel,
        filepath: String,
        write_to_console_too: bool,
    ) -> Self {
        let log_file = log_file::LogFile::new(&filepath);

        Self {
            name,
            min_level,
            log_file: Some(log_file),
            write_to_console: write_to_console_too,
            write_to_file: true,
        }
    }

    /// Creates a new logger that writes to the console. The minimum log level is set to "Info".
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the logger.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    /// ```
    ///
    /// This will create a logger that writes to the console, has the name "example" and the minimum log level "Info".
    pub fn new_default(name: String) -> Self {
        Self::new(name, log_level::LogLevel::Info)
    }

    fn get_date_time() -> String {
        offset::Local::now().format("%F %T%.3f").to_string()
    }

    fn get_colored_level_name(level: log_level::LogLevel) -> ColoredString {
        level.color_string(level.to_string())
    }

    fn get_colored_message(level: log_level::LogLevel, message: &str) -> ColoredString {
        level.color_string(message)
    }

    fn log_to_file(log_file: &mut LogFile, level: log_level::LogLevel, message: &str, name: &str) {
        log_file.write(format!(
            "[{}] [{}] [{}] {}\n",
            Logger::get_date_time(),
            name,
            level.to_string(),
            message
        ));
    }

    fn log_to_console(level: log_level::LogLevel, message: &str, name: &str) {
        println!(
            "[{}] [{}] [{}] {}",
            Logger::get_date_time(),
            name,
            Logger::get_colored_level_name(level),
            Logger::get_colored_message(level, message)
        );
    }

    /// Logs a message with the specified log level.
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level of the message.
    /// * `message` - The message.
    ///
    /// # Examples
    ///
    /// ## Default log level
    ///
    /// ```
    /// use logger::Logger;
    /// use logger::log_level::LogLevel;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.log(LogLevel::Info, "This is an info message.");
    /// logger.log(LogLevel::Debug, "This is a debug message.");
    /// logger.log(LogLevel::Trace, "This is a trace message.");
    /// ```
    ///
    /// This will log the following messages:
    /// [2020-12-31 23:59:59.999] [example] [info] This is an info message.
    /// [2020-12-31 23:59:59.999] [example] [debug] This is a debug message.
    /// [2020-12-31 23:59:59.999] [example] [trace] This is a trace message.
    ///
    /// All messages will be logged, because the minimum log level is the default "Info".
    ///
    /// ## Custom log level
    /// ```
    /// use logger::Logger;
    /// use logger::log_level::LogLevel;
    ///
    /// let mut logger = Logger::new(String::from("example"), LogLevel::Debug);
    ///
    /// logger.log(LogLevel::Info, "This is an info message.");
    /// logger.log(LogLevel::Debug, "This is a debug message.");
    /// logger.log(LogLevel::Trace, "This is a trace message.");
    /// ```
    ///
    /// This will log the following messages:
    /// [2020-12-31 23:59:59.999] [example] [debug] This is a debug message.
    /// [2020-12-31 23:59:59.999] [example] [trace] This is a trace message.
    ///
    /// Only the messages with the log level "Debug" and "Trace" will be logged, because the minimum log level is "Debug".
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn log(&mut self, level: log_level::LogLevel, message: &str) -> bool {
        if level as u8 >= self.min_level as u8 {
            if self.write_to_file {
                if let Some(log_file) = &mut self.log_file {
                    Logger::log_to_file(log_file, level, message, &self.name);
                }
            }

            if self.write_to_console {
                Logger::log_to_console(level, message, &self.name);
            }

            return true;
        }

        false
    }

    /// Logs a message with the log level "Info".
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    /// This is a shortcut for [`log`](#method.log) with the log level "Info".
    /// See [`log`](#method.log) for more information.
    ///
    /// The message will only be logged if the minimum log level is "Info" (which is the default).
    ///
    /// # Arguments
    ///
    /// * `message` - The message.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.info("This is an info message.");
    /// ```
    ///
    /// This will log the following message:
    /// [2020-12-31 23:59:59.999] [example] [info] This is an info message.\
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn info(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Info, message)
    }

    /// Logs a message with the log level "Debug".
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    /// This is a shortcut for [`log`](#method.log) with the log level "Debug".
    /// See [`log`](#method.log) for more information.
    ///
    /// The message will only be logged if the minimum log level is "Info" or "Debug".
    ///
    /// # Arguments
    ///
    /// * `message` - The message.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.debug("This is a debug message.");
    /// ```
    ///
    /// This will log the following message:
    /// [2020-12-31 23:59:59.999] [example] [debug] This is a debug message.
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn debug(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Debug, message)
    }

    /// Logs a message with the log level "Trace".
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    /// This is a shortcut for [`log`](#method.log) with the log level "Trace".
    /// See [`log`](#method.log) for more information.
    ///
    /// The message will only be logged if the minimum log level is "Info", "Debug" or "Trace".
    ///
    /// # Arguments
    ///
    /// * `message` - The message.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.trace("This is a trace message.");
    /// ```
    ///
    /// This will log the following message:
    /// [2020-12-31 23:59:59.999] [example] [trace] This is a trace message.
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn trace(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Trace, message)
    }

    /// Logs a message with the log level "Warning".
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    /// This is a shortcut for [`log`](#method.log) with the log level "Warning".
    /// See [`log`](#method.log) for more information.
    ///
    /// The message will only be logged if the minimum log level is "Info", "Debug", "Trace" or "Warning".
    ///
    /// # Arguments
    ///
    /// * `message` - The message.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.warning("This is a warning message.");
    /// ```
    ///
    /// This will log the following message:
    /// [2020-12-31 23:59:59.999] [example] [warning] This is a warning message.
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn warning(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Warning, message)
    }

    /// Logs a message with the log level "Error".
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    /// This is a shortcut for [`log`](#method.log) with the log level "Error".
    /// See [`log`](#method.log) for more information.
    ///
    /// The message will only be logged if the minimum log level is "Info", "Debug", "Trace", "Warning" or "Error".
    ///
    /// # Arguments
    ///
    /// * `message` - The message.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.error("This is an error message.");
    /// ```
    ///
    /// This will log the following message:
    /// [2020-12-31 23:59:59.999] [example] [error] This is an error message.
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn error(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Error, message)
    }

    /// Logs a message with the log level "Critical".
    /// Returns `true` if the message was logged and `false` if the message wasn't logged because the log level was too low.
    /// This is a shortcut for [`log`](#method.log) with the log level "Critical".
    /// See [`log`](#method.log) for more information.
    /// The message will always be logged.
    /// This is the highest log level.
    ///
    /// # Arguments
    ///
    /// * `message` - The message.
    ///
    /// # Example
    ///
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new_default(String::from("example"));
    ///
    /// logger.critical("This is a critical message.");
    /// ```
    ///
    /// This will log the following message:
    /// [2020-12-31 23:59:59.999] [example] [critical] This is a critical message.
    ///
    /// # Panics
    ///
    /// This function will panic if we try to log to a file and we can't write to the file.
    pub fn critical(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Critical, message)
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use super::*;

    // Logger::new()

    #[test]
    fn new_logger_should_have_correct_min_level() {
        let logger = Logger::new(String::from("test"), log_level::LogLevel::Trace);

        assert_eq!(logger.min_level, log_level::LogLevel::Trace);
    }

    #[test]
    fn new_logger_should_write_to_console() {
        let logger = Logger::new(String::from("test"), log_level::LogLevel::Trace);

        assert_eq!(logger.write_to_console, true);
    }

    #[test]
    fn new_logger_should_not_write_to_file() {
        let logger = Logger::new(String::from("test"), log_level::LogLevel::Trace);

        assert_eq!(logger.write_to_file, false);
    }

    // Logger::new_default()

    #[test]
    fn new_default_logger_should_have_level_info() {
        let logger = Logger::new_default(String::from("test"));

        assert_eq!(logger.min_level, log_level::LogLevel::Info);
    }

    // Logger::new_to_file()

    #[test]
    fn new_logger_to_file_should_enable_write_to_file() {
        let logger = Logger::new_to_file(
            String::from("test"),
            log_level::LogLevel::Trace,
            String::from("test.log"),
            false,
        );

        assert_eq!(logger.write_to_file, true);
    }

    #[test]
    fn new_logger_to_file_which_also_writes_to_console_should_enable_write_to_console() {
        let logger = Logger::new_to_file(
            String::from("test"),
            log_level::LogLevel::Trace,
            String::from("test.log"),
            true,
        );

        assert_eq!(logger.write_to_console, true);
    }

    #[test]
    fn new_logger_to_file_which_does_not_write_to_console_should_disable_write_to_console() {
        let logger = Logger::new_to_file(
            String::from("test"),
            log_level::LogLevel::Trace,
            String::from("test.log"),
            false,
        );

        assert_eq!(logger.write_to_console, false);
    }

    #[test]
    fn new_logger_to_file_should_have_correct_min_level() {
        let logger = Logger::new_to_file(
            String::from("test"),
            log_level::LogLevel::Trace,
            String::from("test.log"),
            false,
        );

        assert_eq!(logger.min_level, log_level::LogLevel::Trace);
    }

    // Logger::get_colored_level_name()

    #[test]
    fn get_colored_level_name_should_return_correct_string_for_critical() {
        let level_name = Logger::get_colored_level_name(log_level::LogLevel::Critical);

        assert_eq!(level_name, "critical".red().bold());
    }

    // Logger::get_colored_message()

    #[test]
    fn get_colored_message_should_return_correct_string_for_critical() {
        let message = Logger::get_colored_message(log_level::LogLevel::Critical, "test");

        assert_eq!(message, "test".red().bold());
    }

    // Logger::log()

    #[test]
    fn log_should_return_false_if_level_is_below_min_level() {
        let mut logger = Logger::new(String::from("test"), log_level::LogLevel::Error);

        let result = logger.log(log_level::LogLevel::Debug, "test");

        assert_eq!(result, false);
    }

    #[test]
    fn log_should_return_true_if_level_is_above_min_level() {
        let mut logger = Logger::new(String::from("test"), log_level::LogLevel::Debug);

        let result = logger.log(log_level::LogLevel::Error, "test");

        assert_eq!(result, true);
    }

    #[test]
    fn log_should_return_true_if_level_is_equal_to_min_level() {
        let mut logger = Logger::new(String::from("test"), log_level::LogLevel::Error);

        let result = logger.log(log_level::LogLevel::Error, "test");

        assert_eq!(result, true);
    }
}
