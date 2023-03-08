use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::process;

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
                eprintln!("Error opening file: {}", e);

                process::exit(1);
            });

        Self { file }
    }

    pub fn write(&mut self, to_write: String) {
        self.file
            .write_all(to_write.as_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Error writing to file: {}", e);

                process::exit(1);
            })
    }
}
