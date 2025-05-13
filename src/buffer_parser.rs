use std::{
    collections::{HashMap, HashSet},
    path::is_separator,
    usize,
};

use crate::{
    buffer::{self, Buffer},
    logger::Logger,
};

impl Buffer {
    // pub only for debug later remove
    pub fn is_brake(
        &self,
        char: char,
        symbols_are_brakes: bool,
        use_symbols_as_characters: bool,
    ) -> bool {
        if use_symbols_as_characters {
            return !self.is_symbol(char);
        }
        if symbols_are_brakes {
            return self.brake_char.contains(&char);
        }

        return self.spacers.contains(&char);
    }
    pub fn is_symbol(&self, char: char) -> bool {
        return self.symbol_char.contains(&char);
    }
    pub fn get_word_start(
        &self,
        mut x: usize,
        mut y: usize,
        symbols_are_brakes: bool,
    ) -> (usize, usize) {
        // if this is end of line i have to move to a next line to don't cause any subtractions with
        // overflow
        if x == 0 {
            y -= 1;
            // it's going to be clamped so i can just set it to max
            x = usize::MAX;
        }

        // so there are no weird things happening
        x = x.clamp(1, self.line_len_min_1(y) - 1) - 1;

        y = y.clamp(1, (self.buffer_len_min_1() - 1).max(1));
        let mut encountered_non_spacers = false;
        let mut current_line_bytes = self.read_line(y).as_bytes();
        let mut current_line_len = self.line_len(y);

        // if I start in a symbols block i want to use symbols as characters not brakes to go to
        // the end/start of that block
        let use_symbols_as_characters =
            current_line_len != 0 && self.symbol_char.contains(&(current_line_bytes[x] as char));

        loop {
            let end: bool = if current_line_len == 0 {
                false
                // until i have encountered a word i have to skip all spaces so it goes thru things
                // like multiple spaces
            } else if !encountered_non_spacers {
                if !self.is_brake(
                    current_line_bytes[x] as char,
                    false,
                    use_symbols_as_characters,
                ) {
                    encountered_non_spacers = true;
                    if x == 0 {
                        return (x, y);
                    }
                }
                false
                // if this is end of line just end the word and return last pos (also current one) of line
            } else if x == 0 {
                return (x, y);
            } else {
                // in other cases check if this is a brake and if so end parsing
                self.is_brake(
                    current_line_bytes[x] as char,
                    symbols_are_brakes,
                    use_symbols_as_characters,
                )
            };
            // i have to go back a position because the last char was a brake and i won't to get the
            // last character not space after it
            if end {
                return ((x as i32 + 1).max(0) as usize, y);
            } else {
                if x == 0 {
                    if y <= 1 {
                        return (current_line_len, y);
                    } else {
                        // go to the next line
                        y -= 1;

                        x = self.line_len_min_1(y) - 1;
                        // update line stats
                        current_line_bytes = self.read_line(y).as_bytes();
                        current_line_len = self.line_len(y);
                    }
                }

                x -= 1;
            }
        }
    }
    pub fn get_word_end(
        &self,
        mut x: usize,
        mut y: usize,
        symbols_are_brakes: bool,
    ) -> (usize, usize) {
        // so there are no weird things happening
        x += 1;
        x = x.clamp(0, self.line_len_min_1(y) - 1);
        y = y.clamp(1, (self.buffer_len_min_1() - 1).max(1));
        let mut encountered_non_spacers = false;
        let buffer_len = self.buffer_len_min_1();
        let mut current_line_bytes = self.read_line(y).as_bytes();
        let mut current_line_len = self.line_len(y);
        let mut current_line_len_min_1 = self.line_len_min_1(y);
        let use_symbols_as_characters =
            current_line_len != 0 && self.symbol_char.contains(&(current_line_bytes[x] as char));
        loop {
            let end: bool = if current_line_len == 0 {
                false
            // until i have encountered a word i have to skip all spaces so it goes thru things
            // like multiple spaces
            } else if !encountered_non_spacers {
                if !self.is_brake(
                    current_line_bytes[x] as char,
                    false,
                    use_symbols_as_characters,
                ) {
                    encountered_non_spacers = true;
                }
                false

                // if this is end of line just end the word and return last pos (also current one) of line
            } else if x >= current_line_len_min_1 - 1 {
                return (x, y);
            } else {
                self.is_brake(
                    current_line_bytes[x] as char,
                    symbols_are_brakes,
                    use_symbols_as_characters,
                )
            };
            if end {
                // i have to go back a position because the last char was a brake and i won't to get the
                // last character not space after it

                return ((x as i32 - 1).max(0) as usize, y);
            } else {
                x += 1;

                if x >= current_line_len {
                    if y == buffer_len - 1 {
                        return (current_line_len, y);
                    } else {
                        // go to the next line
                        y += 1;
                        x = 0;
                        // update line stats
                        current_line_bytes = self.read_line(y).as_bytes();
                        current_line_len = self.line_len(y);
                        current_line_len_min_1 = self.line_len_min_1(y);
                    }
                }
            }
        }
    }
}
