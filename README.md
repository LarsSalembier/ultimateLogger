# UltimateLogger

Ultimate Logger is a simple logger than can log to a file and/or `stdout`. It is designed to be simple to use and to be able to be used in any project. It uses colored output to make it easier to read when logging to `stdout`.

It has 6 levels of logging: `trace`, `debug`, `info`, `warn`, `error` and `critical`. It has a `log` function that can be used to log any level of logging. It also has shortcut `trace`, `debug`, `info`, `warn`, `error` and `critical` functions to log at those levels.

Multiple loggers with different names can be made. Each logger can have its own level of logging. The default logger has a level of `trace`. The default logger can be accessed with the `Logger::new_default()` function. Other loggers can be made with the `Logger::new()` function.

## Examples

### Write to the console

```
use ultimate_logger::Logger;
use ultimate_logger::log_level::LogLevel;

let mut logger = Logger::new(String::from("example"), LogLevel::Trace);

logger.trace("This is a trace message");
logger.debug("This is a debug message");
logger.info("This is an info message");
logger.warning("This is a warning message");
logger.error("This is an error message");
logger.critical("This is a critical message");
```

This will output the following to the console with appropriate colors:

```text
[2020-05-01 12:00:00.000] [example] [trace] This is a trace message
[2020-05-01 12:00:00.000] [example] [debug] This is a debug message
[2020-05-01 12:00:00.000] [example] [info] This is an info message
[2020-05-01 12:00:00.000] [example] [warning] This is a warning message
[2020-05-01 12:00:00.000] [example] [error] This is an error message
[2020-05-01 12:00:00.000] [example] [critical] This is a critical message
```

### Write to a file

```
use ultimate_logger::Logger;
use ultimate_logger::log_level::LogLevel;

let mut logger = Logger::new_to_file(String::from("example"), LogLevel::Trace, String::from("log.txt"), true);

logger.trace("This is a trace message");
logger.debug("This is a debug message");
logger.info("This is an info message");
logger.warning("This is a warning message");
logger.error("This is an error message");
logger.critical("This is a critical message");
```

Below is the text which will output to the file `log.txt`, and to the console. The console output will be colored.

```text
[2020-05-01 12:00:00.000] [example] [trace] This is a trace message
[2020-05-01 12:00:00.000] [example] [debug] This is a debug message
[2020-05-01 12:00:00.000] [example] [info] This is an info message
[2020-05-01 12:00:00.000] [example] [warning] This is a warning message
[2020-05-01 12:00:00.000] [example] [error] This is an error message
[2020-05-01 12:00:00.000] [example] [critical] This is a critical message
```

## Features

- Write to a file
- Write to the console
- Write to both
- Set a minimum log level
- Colored output
- Timestamps
- Multiple loggers with different names

## Documentation

[Documentation][https://docs.rs/ultimate_logger]

## Crates.io

[Crates.io][https://crates.io/crates/ultimate_logger]

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.
