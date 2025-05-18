use std::io::Stdout;

use crossterm::{
    QueueableCommand,
    style::{self, Stylize, style},
};

use super::handler;
use crate::{
    EditorValues,
    buffer::Buffer,
    selection_manager::{self, SelectionManager},
};
use crate::{logger::Logger, text_formatting};
pub fn handle_displaying_visual_mode_selection(
    selection_manager: &mut SelectionManager,
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    let from = selection_manager.from_y;
    let to = selection_manager.to_y;

    let terminal_len = crossterm::terminal::size().unwrap().0 as usize;
    let buffer_len = buffer.buffer_len();

    for y in from - 1..to + 1 {
        if y == 0 {
            continue;
        }
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
        if y > from && y < to {
            handler::override_cursor_position(0, y, stdout);

            // add background
            stdout.queue(style::Print(
                text_formatting::get_style_for_line(text_to_draw).on_green(),
            ));
        } else if from == to && y == to {
            let split_1 = text_to_draw.split_at(selection_manager.from_x);
            let split_2 = split_1
                .1
                .split_at(selection_manager.to_x - selection_manager.from_x + 1);

            let before_selection = split_1.0;
            let after_selection = split_2.1;
            let with_background = split_2.0;

            handler::override_cursor_position(0, y, stdout);
            stdout.queue(style::Print(text_formatting::get_style_for_line(
                before_selection.to_string(),
            )));

            handler::override_cursor_position(selection_manager.from_x, y, stdout);
            stdout.queue(style::Print(
                text_formatting::get_style_for_line(with_background.to_string()).on_green(),
            ));
            handler::override_cursor_position(selection_manager.to_x + 1, y, stdout);
            stdout.queue(style::Print(text_formatting::get_style_for_line(
                after_selection.to_string(),
            )));
        } else if y == from {
            let split = text_to_draw.split_at(selection_manager.from_x);
            let withiout_background = split.0;
            let with_background = split.1;

            handler::override_cursor_position(0, y, stdout);
            stdout.queue(style::Print(text_formatting::get_style_for_line(
                withiout_background.to_string(),
            )));

            handler::override_cursor_position(selection_manager.from_x, y, stdout);
            stdout.queue(style::Print(
                text_formatting::get_style_for_line(with_background.to_string()).on_green(),
            ));
        } else if y == to {
            let split = text_to_draw.split_at(selection_manager.to_x);
            let withiout_background = split.1;
            let with_background = split.0;

            handler::override_cursor_position(0, y, stdout);
            stdout.queue(style::Print(
                text_formatting::get_style_for_line(with_background.to_string()).on_green(),
            ));
            handler::override_cursor_position(selection_manager.to_x, y, stdout);
            stdout.queue(style::Print(text_formatting::get_style_for_line(
                withiout_background.to_string(),
            )));
        } else {
            handler::override_cursor_position(0, y, stdout);
            // without background
            stdout.queue(style::Print(text_formatting::get_style_for_line(
                text_to_draw,
            )));
        }
    }

    handler::move_cursor_up_to(
        editor_values.cursor_x,
        editor_values.cursor_y,
        editor_values,
        stdout,
    );
}
pub fn redraw_lines(
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

        stdout.queue(style::Print(text_formatting::get_style_for_line(
            text_to_draw,
        )));
    }

    handler::move_cursor_up_to(
        editor_values.cursor_x,
        editor_values.cursor_y,
        editor_values,
        stdout,
    );
}
pub fn redraw_whole_viewport(
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    let terminal_len = crossterm::terminal::size().unwrap().0 as usize;
    redraw_lines(1, terminal_len + 1, buffer, editor_values, stdout);
}
pub(super) fn redraw_whole_buffer_from(
    from: usize,
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    redraw_lines(from, buffer.buffer_len() + 1, buffer, editor_values, stdout);
}
pub(super) fn redraw_line(
    y: usize,
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    redraw_lines(y, y + 1, buffer, editor_values, stdout);
}
