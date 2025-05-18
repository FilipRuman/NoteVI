use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveUp},
    style::{self, Stylize, style},
    terminal::size,
};
use std::{
    cmp::{max, min},
    io::Stdout,
    u16, usize,
};

use super::drawing;
use super::{action::Action, drawing::redraw_lines};
use super::{buffer_editing, drawing::redraw_whole_buffer_from};
use crate::{
    EditMode, EditorValues,
    buffer::Buffer,
    clipboard::{self, Clipboard},
    file_manager::save_buffer,
    input::key_handler::KeyHandler,
    menu::{self, display_file_selection},
    selection_manager::SelectionManager,
};

pub fn handle_actions(
    editor_values: &mut EditorValues,
    stdout: &mut Stdout,
    actions: Vec<Action>,
    buffer: &mut Buffer,
    selection_manager: &mut SelectionManager,
    clipboard: &mut Clipboard,
) {
    for action in actions {
        match action {
            Action::MoveCursor { x, y } => {
                move_cursor_by(true, x, -y, editor_values, stdout, buffer)
            }
            Action::InsertText(_) => todo!(),
            Action::Quit => {
                if editor_values.in_menu {
                    editor_values.quit = true;
                    continue;
                }

                display_file_selection(buffer, editor_values, stdout);
            }
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
            Action::OpenCurrentFile => {
                if editor_values.in_menu {
                    menu::open_current_file(buffer, editor_values, stdout);
                }
            }
            Action::SaveBuffer => {
                save_buffer(buffer, editor_values.current_file_path.to_owned());
            }
            Action::DeleteCurrentLine { move_to_clipboard } => {
                let line = buffer_editing::remove_line(
                    editor_values.cursor_y,
                    editor_values,
                    stdout,
                    buffer,
                    true,
                );
                if move_to_clipboard {
                    clipboard.set_save(line);
                }
            }
            Action::GoToEndOfWord {
                characters_are_brakes,
            } => {
                let end_of_word = buffer.get_next_word_end(
                    editor_values.cursor_x,
                    editor_values.cursor_y,
                    characters_are_brakes,
                );
                move_cursor_up_to(end_of_word.0, end_of_word.1, editor_values, stdout);

                editor_values.desired_cursor_x = editor_values.cursor_x;
            }
            Action::GoToStartOfWord {
                characters_are_brakes,
            } => {
                let start_of_word = buffer.get_next_word_start(
                    editor_values.cursor_x,
                    editor_values.cursor_y,
                    characters_are_brakes,
                );
                move_cursor_up_to(start_of_word.0, start_of_word.1, editor_values, stdout);

                editor_values.desired_cursor_x = editor_values.cursor_x;
            }
            Action::SelectCurrentLine => {
                selection_manager.select_line(buffer, editor_values.cursor_y)
            }
            Action::SelectCurrentWords {
                characters_are_brakes,
            } => {
                selection_manager.select_word(
                    buffer,
                    editor_values.cursor_x,
                    editor_values.cursor_y,
                    characters_are_brakes,
                );
                // move cursor to  a begging of selected word
                move_cursor_up_to(
                    selection_manager.from_x,
                    editor_values.cursor_y,
                    editor_values,
                    stdout,
                );
            }
            Action::DeleteSelection { move_to_clipboard } => {
                let redraw_start_line = selection_manager.from_y;
                if move_to_clipboard {
                    clipboard.copy(
                        selection_manager.from_x,
                        selection_manager.from_y,
                        selection_manager.to_x,
                        selection_manager.to_y,
                        buffer,
                    );
                }
                selection_manager.delete_selection(buffer);
                redraw_whole_buffer_from(redraw_start_line, buffer, editor_values, stdout);
            }
            Action::DebugSelection => {
                stdout.queue(cursor::MoveTo(
                    selection_manager.from_x as u16,
                    selection_manager.from_y as u16 - 1,
                ));

                stdout.queue(style::Print("→")).unwrap();
                stdout.queue(cursor::MoveTo(
                    selection_manager.to_x as u16,
                    selection_manager.to_y as u16 - 1,
                ));

                stdout.queue(style::Print("←")).unwrap();
            }
            Action::CopySelected => clipboard.copy(
                selection_manager.from_x,
                selection_manager.from_y,
                selection_manager.to_x,
                selection_manager.to_y,
                buffer,
            ),
            Action::PasteFromClipboard => {
                clipboard.paste(editor_values.cursor_x, editor_values.cursor_y, buffer);
                redraw_whole_buffer_from(editor_values.cursor_y, buffer, editor_values, stdout);
            }
            Action::SaveBuffer => todo!(),
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
