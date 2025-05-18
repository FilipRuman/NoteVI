use std::{char, fs};

use crate::{
    buffer::{self, Buffer},
    logger::Logger,
};

pub const SAVE_FILES_PATH: &str = "./saves/";
pub fn save_buffer(buffer: &Buffer, file_path: String) {
    Logger::default_log(format!("saving buffer"));
    let result = fs::write(file_path, buffer.read_all_buffer());
    match result {
        Ok(_) => {}
        Err(E) => Logger::default_log(format!("saving buffer did not succeed {}", E)),
    }
}
pub fn create_file(file_path: String) {
    Logger::default_log(format!("create_file {}", file_path));
    let result = fs::write(file_path, "");
    match result {
        Ok(_) => {}
        Err(E) => Logger::default_log(format!("creating file did not succeed {}", E)),
    };
}
pub fn remove_file(path: String) {
    Logger::default_log(format!("remove file path:{}", path));
    let result = fs::remove_file(path);
    match result {
        Ok(_) => {}
        Err(E) => Logger::default_log(format!("removing file did not succeed {}", E)),
    };
}
pub fn rename_file(from: String, to: String) {
    Logger::default_log(format!("rename file form:{} to:{}", from, to));
    let result = fs::rename(from, to);
    match result {
        Ok(_) => {}
        Err(E) => Logger::default_log(format!("renaming file did not succeed {}", E)),
    };
}
pub fn get_all_files_in_directory(path: String) -> Vec<String> {
    let dir_iterator = match fs::read_dir(path) {
        Ok(iterator) => iterator,
        Err(_) => {
            Logger::default_log("get_all_files_in_directory: path does not exist!".to_string());
            panic!()
        }
    };

    let mut files = Vec::new();
    for result in dir_iterator {
        match result {
            Ok(dir_entry) => {
                files.push(dir_entry.file_name().to_str().unwrap().to_string());
            }
            Err(_) => {
                continue;
            }
        }
    }
    Logger::default_log(format!("get_all_files_in_directory: files {:?}", files));
    return files;
}

pub fn read_file_into_buffer(buffer: &mut Buffer, file_path: String) {
    let result = fs::read(file_path);
    let bytes = match result {
        Ok(str) => str,
        Err(E) => {
            Logger::default_log(format!("reading file did not succeed {}", E));
            panic!("reading file did not succeed");
        }
    };
    let mut text_lines: Vec<String> = Vec::new();
    let mut current_text = String::new();
    for i in 0..bytes.len() {
        let char = bytes[i] as char;
        if char != '\n' {
            current_text += &char.to_string();
            continue;
        }
        text_lines.push(current_text.to_string());
        current_text.clear();
    }

    buffer.set_buffer(text_lines);
}
