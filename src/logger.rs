use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub const LOGGING_PATH: &str = "./logs/logs.md";

pub struct Logger {
    pub file: File,
}

impl Logger {
    pub fn new(path: String) -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)
            .expect(&format!("path for logging file:{} was not correct!", &path));
        Logger { file }
    }
    pub fn log(&mut self, text: String) {
        self.file.write_all((text + "\n").as_bytes()).unwrap();
    }

    pub fn default_log(text: String) {
        Logger::new(LOGGING_PATH.to_string())
            .file
            .write_all((text + "\n").as_bytes())
            .unwrap();
    }
}
