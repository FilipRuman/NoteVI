use std::{io::Stdout, u16};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize, style},
    terminal::size,
};

use crate::{EditMode, EditorValues};

pub enum Action {
    MoveCursor(i32, i32),
    ChangeCursorPosition(i32, i32),
    InsertText(String),
    WriteText(String),

    Quit,
    NormalMode,
    InsertMode,
}

pub fn handle_actions(editor_values: &mut EditorValues, stdout: &mut Stdout, actions: Vec<Action>) {
    for action in actions {
        match action {
            Action::MoveCursor(x, y) => move_cursor(x, y, editor_values, stdout),
            Action::InsertText(_) => todo!(),
            Action::Quit => editor_values.quit = true,
            Action::NormalMode => editor_values.mode = EditMode::Normal,
            Action::InsertMode => editor_values.mode = EditMode::Insert,
            Action::ChangeCursorPosition(_, _) => todo!(),
            Action::WriteText(text) => write_text(text, editor_values, stdout),
        }
    }
}
fn move_cursor(x: i32, y: i32, editor_values: &mut EditorValues, stdout: &mut Stdout) {
    let size = size().unwrap();

    editor_values.cursor_x = i32::clamp(editor_values.cursor_x as i32 + x, 0, size.0 as i32) as u16;
    editor_values.cursor_y = i32::clamp(editor_values.cursor_y as i32 + y, 1, size.1 as i32) as u16;

    stdout
        .queue(cursor::MoveTo(
            editor_values.cursor_x,
            size.1 - editor_values.cursor_y,
        ))
        .expect("cursor move did not Succeed");
}

fn write_text(text: String, editor_values: &mut EditorValues, stdout: &mut Stdout) {
    let text_len = text.len() as i32;
    stdout.queue(style::Print(text)).unwrap();
    move_cursor(text_len, 0, editor_values, stdout);
}
