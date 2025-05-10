use std::usize;

pub struct Buffer {
    text_lines: Vec<String>,
}
impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            text_lines: Vec::new(),
        }
    }
    pub fn write_text(&mut self, x: usize, y: usize, text: &String) {
        if self.text_lines.len() <= y {
            self.add_text_layers_up_to(y);
        }
        self.text_lines[y].insert_str(x, text);
    }
    fn add_text_layers_up_to(&mut self, y: usize) {
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
        if to == 0 && line != 1 {
            let current_line_text = &(self.text_lines[line].to_owned());
            self.text_lines[line - 1] += current_line_text;
            self.remove_line(line);
            return true;
        }
        if from >= to || to > line_len {
            return false;
        }
        let split_output = self.text_lines[line].split_at(from);
        let mut output_text = split_output.0.to_string();
        output_text += split_output.1.split_at(to - from).1;

        self.text_lines[line] = output_text;
        false
    }
    pub fn read_line(&self, line: usize) -> &String {
        &self.text_lines[line]
    }
    pub fn remove_line(&mut self, line: usize) {
        self.text_lines.remove(line);
    }
    pub fn remove_lines(&mut self, from: usize, to: usize) {
        for line in from..to {
            self.text_lines.remove(line);
        }
    }
    pub fn insert_line(&mut self, line: usize) {
        self.text_lines.insert(line, String::new());
    }

    pub fn line_len(&self, y: usize) -> usize {
        if self.text_lines.len() > y {
            return self.text_lines[y].len();
        }
        0
    }
    pub fn buffer_len(&self) -> usize {
        self.text_lines.len()
    }
}
