use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub(crate) struct LogFile {
    file: File,
}

impl LogFile {
    pub(crate) fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|e| {
                panic!(
                    "Error opening log file: {}\nPath to log file was: {}",
                    e, path
                );
            });

        Self { file }
    }

    pub(crate) fn write(&mut self, to_write: &str) {
        self.file
            .write_all(to_write.as_bytes())
            .unwrap_or_else(|e| {
                panic!(
                    "Error writing to log file: {}\nText to be written was:\n{}",
                    e, to_write
                );
            })
    }
}
