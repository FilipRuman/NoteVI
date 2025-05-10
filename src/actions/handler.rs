use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize, style},
    terminal::size,
};
use std::{
    cmp::{max, min},
    io::Stdout,
    u16, usize,
};

use super::action::Action;
use super::buffer_editing;
use super::drawing;
use crate::{EditMode, EditorValues, buffer::Buffer};
pub fn handle_actions(
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    actions: Vec<Action>,
    buffer: &mut Buffer,
) {
    for action in actions {
        match action {
            Action::MoveCursor { x, y } => {
                move_cursor_by(true, x, -y, editor_values, stdout, buffer)
            }
            Action::InsertText(_) => todo!(),
            Action::Quit => editor_values.quit = true,
            Action::NormalMode => {
                editor_values.mode = EditMode::Normal;
                stdout
                    .execute(cursor::SetCursorStyle::BlinkingBlock)
                    .unwrap();
            }
            Action::InsertMode => {
                editor_values.mode = EditMode::Insert;
                stdout.execute(cursor::SetCursorStyle::BlinkingBar).unwrap();
            }
            Action::ChangeCursorPosition { x: _, y: _ } => todo!(),
            Action::WriteText(text) => {
                buffer_editing::write_text(text, editor_values, stdout, buffer)
            }
            Action::RemoveText {
                from,
                to,
                line,
                move_cursor_back,
            } => buffer_editing::remove_text(
                from,
                to,
                line,
                editor_values,
                stdout,
                buffer,
                move_cursor_back,
            ),
            Action::ToDo => todo!(),
            Action::BackspaceLetters { count } => {
                // so i don't  subtract with overflow :D
                if editor_values.cursor_x >= count {
                    buffer_editing::remove_text(
                        editor_values.cursor_x - count,
                        editor_values.cursor_x,
                        editor_values.cursor_y,
                        editor_values,
                        stdout,
                        buffer,
                        true,
                    )
                } else {
                    buffer_editing::remove_text(
                        0,
                        editor_values.cursor_x,
                        editor_values.cursor_y,
                        editor_values,
                        stdout,
                        buffer,
                        true,
                    )
                }
            }
            Action::DeleteLetters { count } => {
                if count <= buffer.line_len(editor_values.cursor_y) {
                    buffer_editing::remove_text(
                        editor_values.cursor_x,
                        editor_values.cursor_x + count,
                        editor_values.cursor_y,
                        editor_values,
                        stdout,
                        buffer,
                        false,
                    )
                }
            }
            Action::InsertLine => {
                buffer.insert_line(editor_values.cursor_y);
                drawing::redraw_lines(
                    editor_values.cursor_y,
                    buffer.buffer_len(),
                    buffer,
                    editor_values,
                    stdout,
                );
                move_cursor_by(false, 0, 1, editor_values, stdout, buffer);
            }
        }
    }
}

pub(super) fn move_cursor_by(
    change_desired_x: bool,
    x: i32,
    y: i32,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    buffer: &Buffer,
) {
    let terminal_size = size().unwrap();

    let global_y =
        i32::clamp(editor_values.cursor_y as i32 + y, 1, terminal_size.1 as i32) as usize;

    let text_line_size = buffer.line_len(global_y);
    let target_x = if x == 0 {
        editor_values.desired_cursor_x as i32
    } else {
        editor_values.cursor_x as i32 + x
    };
    let global_x = i32::clamp(
        target_x,
        0,
        usize::min(text_line_size, terminal_size.0 as usize) as i32,
    ) as usize;

    editor_values.cursor_x = global_x;
    editor_values.cursor_y = global_y;

    if change_desired_x && x != 0 {
        editor_values.desired_cursor_x = editor_values.cursor_x;
    }

    stdout.queue(cursor::MoveTo(
        editor_values.cursor_x as u16,
        editor_values.cursor_y as u16 - 1,
    ));
}
pub(super) fn move_cursor_up_to(
    x: usize,
    y: usize,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    let size = size().unwrap();

    editor_values.cursor_x = usize::clamp(x, 0, size.0 as usize);
    editor_values.cursor_y = usize::clamp(y, 1, size.1 as usize);

    stdout
        .queue(cursor::MoveTo(
            editor_values.cursor_x as u16,
            editor_values.cursor_y as u16 - 1,
        ))
        .expect("cursor move did not Succeed");
}
pub(super) fn override_cursor_position(x: usize, y: usize, stdout: &mut Stdout) {
    stdout
        .queue(cursor::MoveTo(x as u16, y as u16 - 1))
        .expect("cursor move did not Succeed");
}
