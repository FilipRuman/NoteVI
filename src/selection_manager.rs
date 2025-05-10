use crate::buffer::Buffer;

pub struct SelectionManager {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
}
impl SelectionManager {
    pub fn delete_selection(&mut self, buffer: &mut Buffer) {
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
    pub fn clear_selection(&mut self) {
        self.to_x = 0;
        self.to_y = 0;
        self.from_x = 0;
        self.from_y = 0;
    }
}
