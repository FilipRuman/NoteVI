use std::{cmp::min, io::Stdout};

use super::{drawing, handler};
use crate::{EditorValues, buffer::Buffer};
pub(super) fn write_text(
    text: String,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    buffer: &mut Buffer,
) {
    buffer.write_text(
        min(
            buffer.line_len(editor_values.cursor_y),
            editor_values.cursor_x,
        ),
        editor_values.cursor_y,
        &text,
    );
    drawing::redraw_line(editor_values.cursor_y, buffer, editor_values, stdout);

    let text_len = text.len();
    handler::move_cursor_by(true, text_len as i32, 0, editor_values, stdout, buffer);
}
pub(super) fn remove_text(
    from: usize,
    to: usize,
    line: usize,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    buffer: &mut Buffer,
    move_back: bool,
) {
    let previous_line_len = buffer.line_len(line - 1);
    let move_back_a_line = buffer.remove_text(line, from, to);
    if move_back {
        if move_back_a_line {
            handler::move_cursor_up_to(previous_line_len, line - 1, editor_values, stdout);
        } else {
            handler::move_cursor_by(
                true,
                from as i32 - to as i32,
                0,
                editor_values,
                stdout,
                buffer,
            );
        }
    }
    // all lines might move back so i have to redraw all lines from this  to end
    drawing::redraw_lines(1, buffer.buffer_len() + 1, buffer, editor_values, stdout);
}
pub(super) fn remove_line(
    line: usize,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    buffer: &mut Buffer,
    move_back: bool,
) -> String {
    let output = buffer.read_line(line).to_owned();
    buffer.remove_line(line);
    if move_back {
        handler::move_cursor_by(true, 0, 0, editor_values, stdout, buffer);
    }
    // all lines move back so i have to redraw all lines from this  to end
    drawing::redraw_lines(line, buffer.buffer_len() + 1, buffer, editor_values, stdout);

    return output;
}
