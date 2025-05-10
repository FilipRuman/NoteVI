use crate::{EditMode, EditorValues, buffer::Buffer};
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

#[derive(Clone, Debug)]
pub enum Action {
    MoveCursor {
        x: i32,
        y: i32,
    },
    ChangeCursorPosition {
        x: i32,
        y: i32,
    },
    InsertText(String),
    WriteText(String),
    RemoveText {
        from: usize,
        to: usize,
        line: usize,
        move_cursor_back: bool,
    },
    BackspaceLetters {
        count: usize,
    },
    DeleteLetters {
        count: usize,
    },

    Quit,
    NormalMode,
    InsertMode,
    ToDo,

    InsertLine,
    //
    //TODO add parsing
    // StartSelection,
    // EndSelection,
    // MoveToEndOfWord,
    // MoveToBeginningOfWord,
    //
    // SelectLineBeforeCursor,
    // SelectLineAfterCursor,
    // SelectLine,
    // SelectWord,
    // DeleteSelected,
    // CopySelected,
    // PasteSelected,
    // MoveSelected,
}

pub fn handle_actions(
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    actions: Vec<Action>,
    buffer: &mut Buffer,
) {
    for action in actions {
        match action {
            Action::MoveCursor { x, y } => move_cursor_by(x, -y, editor_values, stdout, buffer),
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
            Action::WriteText(text) => write_text(text, editor_values, stdout, buffer),
            Action::RemoveText {
                from,
                to,
                line,
                move_cursor_back,
            } => remove_text(
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
                    remove_text(
                        editor_values.cursor_x - count,
                        editor_values.cursor_x,
                        editor_values.cursor_y,
                        editor_values,
                        stdout,
                        buffer,
                        true,
                    )
                } else {
                    remove_text(
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
                    remove_text(
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
                redraw_lines(
                    editor_values.cursor_y,
                    buffer.buffer_len(),
                    buffer,
                    editor_values,
                    stdout,
                );
                move_cursor_by(0, 1, editor_values, stdout, buffer);
            }
        }
    }
}

fn remove_text(
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
            move_cursor_up_to(previous_line_len, line - 1, editor_values, stdout);
        } else {
            move_cursor_by(from as i32 - to as i32, 0, editor_values, stdout, buffer);
        }
    }
    // all lines might move back so i have to redraw all lines from this  to end
    redraw_lines(1, buffer.buffer_len() + 1, buffer, editor_values, stdout);
}
fn remove_line(
    line: usize,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    buffer: &mut Buffer,
    move_back: bool,
) {
    buffer.remove_line(line);
    if move_back {
        move_cursor_by(0, 0, editor_values, stdout, buffer);
    }
    // all lines move back so i have to redraw all lines from this  to end
    redraw_lines(line, buffer.buffer_len() + 1, buffer, editor_values, stdout);
}
fn move_cursor_by(
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

    let global_x = i32::clamp(
        editor_values.cursor_x as i32 + x,
        0,
        usize::min(text_line_size, terminal_size.0 as usize) as i32,
    ) as usize;

    editor_values.cursor_x = global_x;

    editor_values.cursor_y = global_y;

    stdout.queue(cursor::MoveTo(
        editor_values.cursor_x as u16,
        editor_values.cursor_y as u16 - 1,
    ));
}
fn move_cursor_up_to(x: usize, y: usize, editor_values: &mut EditorValues, stdout: &mut Stdout) {
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
fn override_cursor_position(x: usize, y: usize, stdout: &mut Stdout) {
    stdout
        .queue(cursor::MoveTo(x as u16, y as u16 - 1))
        .expect("cursor move did not Succeed");
}
fn write_text(
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
    redraw_line(editor_values.cursor_y, buffer, editor_values, stdout);

    let text_len = text.len();
    move_cursor_by(text_len as i32, 0, editor_values, stdout, buffer);
}

fn redraw_lines(
    from: usize,
    to: usize,
    buffer: &Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
) {
    let terminal_len = size().unwrap().0 as usize;
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
        override_cursor_position(0, y, stdout);
        stdout.queue(style::Print(&text_to_draw)).unwrap();
    }

    move_cursor_up_to(
        editor_values.cursor_x,
        editor_values.cursor_y,
        editor_values,
        stdout,
    );
}

fn redraw_line(y: usize, buffer: &Buffer, editor_values: &mut EditorValues, stdout: &mut Stdout) {
    redraw_lines(y, y + 1, buffer, editor_values, stdout);
}
