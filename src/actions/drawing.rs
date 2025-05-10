use std::io::Stdout;

use crossterm::{
    QueueableCommand,
    style::{self, style},
};

use super::handler;
use crate::{EditorValues, buffer::Buffer};

pub(super) fn redraw_lines(
    from: usize,
    to: usize,
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    let terminal_len = crossterm::terminal::size().unwrap().0 as usize;
    let buffer_len = buffer.buffer_len();
    for y in from..to {
        let mut text_to_draw = if y < buffer_len {
            buffer.read_line(y)
        } else {
            ""
        }
        .to_string();

        let text_layer_size = text_to_draw.len();

        if text_layer_size < terminal_len {
            for _ in text_layer_size..terminal_len {
                text_to_draw += " ";
            }
        }
        handler::override_cursor_position(0, y, stdout);
        stdout.queue(style::Print(&text_to_draw)).unwrap();
    }

    handler::move_cursor_up_to(
        editor_values.cursor_x,
        editor_values.cursor_y,
        editor_values,
        stdout,
    );
}

pub(super) fn redraw_line(
    y: usize,
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    redraw_lines(y, y + 1, buffer, editor_values, stdout);
}
