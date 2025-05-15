use crate::{
    buffer::Buffer,
    logger::{self, Logger},
};

pub struct Clipboard {
    pub save: Vec<String>,
}
impl Clipboard {
    pub fn paste(&self, x: usize, y: usize, buffer: &mut Buffer) {
        buffer.add_text_layers_up_to(y);

        let save_len = self.save.len();
        if save_len == 0 {
            return;
        }

        let first_line = buffer.read_line(y).to_owned();
        let text_to_add_to_end = &first_line[x..first_line.len()];
        let first_line_new_text = first_line[0..x].to_string() + &self.save[0];
        buffer.override_line(y, first_line_new_text);
        if save_len > 2 {
            buffer.insert_lines_of_text(y + 1, self.save[1..save_len - 1].to_vec());
        }

        if save_len > 1 {
            let last_save = &self.save[save_len - 1];
            let last_line = buffer.read_line(save_len + y - 1);
            let last_line_new_text = last_save.to_string() + text_to_add_to_end + last_line;
            buffer.override_line(y, last_line_new_text);
        }
    }
    pub fn copy(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        buffer: &Buffer,
    ) {
        if from_y >= buffer.buffer_len() {
            return;
        }
        self.save.clear();
        if from_y == to_y {
            let line = buffer.read_line(from_y);
            self.save.push(line[from_x..to_x + 1].to_string());

            Logger::default_log(format!(
                "copy into clipboard: 1 line, save-{:?} ",
                self.save
            ));
            return;
        }

        let line = buffer.read_line(from_y);
        self.save.push(line[from_x..line.len()].to_string());
        if from_y > to_y + 1 {
            self.save
                .append(&mut buffer.read_lines(from_y + 1, to_y - 1).to_vec());
        }

        if from_y > to_y {
            let line = buffer.read_line(to_y);
            self.save.push(line[0..to_x].to_string());
        }

        Logger::default_log(format!("copy into clipboard: save-{:?} ", self.save));
    }
}
