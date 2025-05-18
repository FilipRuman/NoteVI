use std::{collections::HashSet, usize};

use crate::logger::Logger;

pub struct Buffer {
    text_lines: Vec<String>,
    pub(super) brake_char: HashSet<char>,
    pub(super) spacers: HashSet<char>,

    pub(super) symbol_char: HashSet<char>,
}
impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            text_lines: Vec::new(),
            brake_char: HashSet::from([
                ' ', '\n', '.', ':', '(', ')', '+', '=', '-', ';', '!', '@', '#', '$', '%', '^',
                '&', '*', '{', '}', ']', '[', ']', '\\', '\"', '/',
            ]),
            spacers: HashSet::from([' ', '\n']),
            symbol_char: HashSet::from([
                '.', ':', '(', ')', '+', '=', '-', ';', '!', '@', '#', '$', '%', '^', '&', '*',
                '{', '}', ']', '[', ']', '\\', '\"', '/',
            ]),
        }
    }
    pub fn set_buffer(&mut self, new_text: Vec<String>) {
        Logger::default_log(format!("set_buffer: {:?}", new_text));
        self.text_lines = new_text;
    }
    pub fn clear_buffer(&mut self) {
        self.text_lines.clear();
    }
    pub fn read_all_buffer(&self) -> String {
        let mut output = String::new();
        for line in &self.text_lines {
            output += line;
            output += "\n";
        }

        return output;
    }
    pub fn write_text(&mut self, x: usize, y: usize, text: &String) {
        if self.text_lines.len() <= y {
            self.add_text_layers_up_to(y);
        }
        self.text_lines[y].insert_str(x, text);
    }
    pub fn override_line(&mut self, y: usize, text: String) {
        //maybe add safety checks
        //if self.text_lines.len() <= y {
        //             self.add_text_layers_up_to(y);
        //         }

        self.text_lines[y] = text;
    }
    pub fn add_text_layers_up_to(&mut self, y: usize) {
        for _ in 0..(y + 1 - self.text_lines.len()) {
            self.text_lines.push("".to_string());
        }
    }
    // returns if has to move back a line
    pub fn remove_text(&mut self, line: usize, from: usize, to: usize) -> bool {
        if line >= self.buffer_len() {
            return true;
        }
        let line_len = self.line_len(line);

        // cursor is at the beginning of a line so move whole line back
        // also if the line selected is the last one don't do it because it will delete the whole line
        Logger::default_log(format!("remove_text: line {} ", line));
        if to >= line_len {
            self.remove_line(line);
            return true;
        }
        if to == 0 {
            let current_line_text = &(self.text_lines[line].to_owned());
            self.text_lines[line - 1] += current_line_text;

            Logger::default_log(format!("remove_text:  remove_line"));
            self.remove_line(line);
            return true;
        }
        if from >= to || to > line_len {
            Logger::default_log(format!(
                "remove_text: out of the line bounds from:{} to:{} line_len:{}",
                from, to, line_len
            ));
            return false;
        }
        let split_output = self.text_lines[line].split_at(from);
        let mut output_text = split_output.0.to_string();
        output_text += split_output.1.split_at(to - from).1;

        Logger::default_log(format!("remove_text: output {}", &output_text));
        self.text_lines[line] = output_text;

        false
    }
    pub fn read_line(&self, line: usize) -> &String {
        &self.text_lines[line]
    }
    pub fn read_lines(&self, from: usize, to: usize) -> &[String] {
        &self.text_lines[from..to]
    }
    pub fn remove_line(&mut self, line: usize) {
        self.text_lines.remove(line);
    }
    pub fn remove_lines(&mut self, from: usize, to: usize) {
        for line in from..to {
            self.text_lines.remove(from);
        }
    }
    pub fn insert_line(&mut self, line: usize) {
        self.text_lines.insert(line, String::new());
    }
    pub fn insert_lines_of_text(&mut self, from: usize, text_lines: Vec<String>) {
        // idk how to make this more efficient
        for i in 0..text_lines.len() {
            self.text_lines.insert(from + i, text_lines[i].to_string());
        }
    }
    pub fn insert_line_of_text(&mut self, y: usize, value: String) {
        self.text_lines.insert(y, value);
    }
    pub fn line_len(&self, y: usize) -> usize {
        if self.text_lines.len() > y {
            return self.text_lines[y].len();
        }
        0
    }
    pub fn line_len_min_1(&self, y: usize) -> usize {
        self.line_len(y).max(1)
    }
    pub fn buffer_len(&self) -> usize {
        self.text_lines.len()
    }
    pub fn buffer_len_min_1(&self) -> usize {
        self.text_lines.len().max(1)
    }
}
