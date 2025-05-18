use crate::{
    EditorValues,
    buffer::{self, Buffer},
    logger::Logger,
};

pub struct SelectionManager {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub selection_mode_start_x: usize,
    pub selection_mode_start_y: usize,
}
impl SelectionManager {
    pub fn handle_visual_mode(&mut self, editor_values: &EditorValues) {
        self.from_x = self.selection_mode_start_x;
        self.from_y = self.selection_mode_start_y;

        self.to_x = editor_values.cursor_x;
        self.to_y = editor_values.cursor_y;

        if self.from_y > self.to_y || (self.from_y == self.to_y && self.from_x > self.to_x) {
            self.sawp_to_and_from();
        }
    }
    pub fn select_word(
        &mut self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        symbols_are_brakes: bool,
    ) {
        let dimensions = buffer.get_current_word_dimensions(x, y, symbols_are_brakes);
        let start = dimensions.0;
        let end = dimensions.1;
        self.set_line_selections_on_one_line(start, end, y);
        Logger::default_log(format!(
            "select_word: self.from_x,{} self.to_x,{} self.from_y,{} self.to_y{}",
            self.from_x, self.to_x, self.from_y, self.to_y,
        ));
    }
    pub fn select_line(&mut self, buffer: &mut Buffer, y: usize) {
        self.set_line_selections_on_one_line(0, buffer.line_len_min_1(y) - 1, y)
    }
    pub fn set_line_selections_on_one_line(&mut self, from: usize, to: usize, line: usize) {
        self.from_x = from;
        self.from_y = line;

        self.to_x = to;
        self.to_y = line;
    }
    pub fn delete_selection(&mut self, buffer: &mut Buffer) {
        if self.from_y > self.to_y || (self.from_y == self.to_y && self.from_x > self.to_x) {
            self.sawp_to_and_from();
        }

        Logger::default_log(format!("delete_selection"));

        if self.from_y == self.to_y {
            buffer.remove_text(self.from_y, self.from_x, self.to_x);

            self.clear_selection();
            return;
        }
        if self.from_y + 1 < self.to_y {
            buffer.remove_lines(self.from_y + 1, self.to_y - 1);
        }

        buffer.remove_text(self.to_y, 0, self.to_x);
        self.clear_selection();
    }

    fn sawp_to_and_from(&mut self) {
        let new_to_x = self.from_x;
        let new_to_y = self.from_y;
        self.from_x = self.to_x;
        self.from_y = self.to_y;
        self.to_x = new_to_x;
        self.to_y = new_to_y;
    }
    pub fn clear_selection(&mut self) {
        self.to_x = 0;
        self.to_y = 0;
        self.from_x = 0;
        self.from_y = 0;
    }
}
