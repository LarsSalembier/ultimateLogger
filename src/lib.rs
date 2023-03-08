mod log_file;
pub mod log_level;

use chrono::offset;
use colored::ColoredString;
use log_file::LogFile;

pub struct Logger {
    name: String,
    min_level: log_level::LogLevel,
    log_file: Option<log_file::LogFile>,
    write_to_console: bool,
    write_to_file: bool,
}

impl Logger {
    pub fn new(name: String, min_level: log_level::LogLevel) -> Self {
        Self {
            name,
            min_level,
            log_file: None,
            write_to_console: true,
            write_to_file: false,
        }
    }

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

    pub fn info(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Info, message)
    }

    pub fn debug(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Debug, message)
    }

    pub fn trace(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Trace, message)
    }

    pub fn warning(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Warning, message)
    }

    pub fn error(&mut self, message: &str) -> bool {
        self.log(log_level::LogLevel::Error, message)
    }

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
