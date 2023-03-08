use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub struct LogFile {
    file: File,
}

impl LogFile {
    pub fn new(path: &String) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path.clone())
            .unwrap_or_else(|e| {
                panic!(
                    "Error opening log file: {}\nPath to log file was: {}",
                    e, path
                );
            });

        Self { file }
    }

    pub fn write(&mut self, to_write: String) {
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
