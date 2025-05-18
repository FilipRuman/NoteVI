use crossterm::cursor::SavePosition;

use crate::{
    EditorValues,
    buffer::Buffer,
    file_manager::{self, SAVE_FILES_PATH},
    input::actions::drawing::{self},
    logger::Logger,
};
pub fn handle_creating_and_renaming_new_save_files(
    buffer: &mut Buffer,
    editor_values: &mut EditorValues,
) {
    Logger::default_log(format!("handle_creating_new_save_files"));
    // could convert this later to a hashset of speed
    let current_save_files = file_manager::get_all_files_in_directory(SAVE_FILES_PATH.to_string());

    let previous_files_len = editor_values.file_names.len();
    for y in 1..buffer.buffer_len() {
        let file_name = buffer.read_line(y);

        if file_name == "" {
            file_manager::remove_file(
                SAVE_FILES_PATH.to_string() + &editor_values.file_names[y - 1],
            );
            continue;
        }

        if current_save_files.contains(file_name) {
            continue;
        }

        Logger::default_log(format!("Found not matching file name"));
        if y >= previous_files_len {
            file_manager::create_file(SAVE_FILES_PATH.to_string() + file_name);
        } else {
            file_manager::rename_file(
                SAVE_FILES_PATH.to_string() + &editor_values.file_names[y - 1],
                SAVE_FILES_PATH.to_string() + file_name,
            );
        }
    }

    editor_values.file_names =
        file_manager::get_all_files_in_directory(file_manager::SAVE_FILES_PATH.to_string());
}

pub fn display_file_selection(
    buffer: &mut Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut std::io::Stdout,
) {
    Logger::default_log(format!("display_file_selection"));
    buffer.clear_buffer();
    editor_values.in_menu = true;
    for y in 0..editor_values.file_names.len() {
        let name = &editor_values.file_names[y];
        buffer.write_text(0, y + 1, name);
    }

    drawing::redraw_whole_viewport(buffer, editor_values, stdout);
}
pub fn open_current_file(
    buffer: &mut Buffer,
    editor_values: &mut EditorValues,
    stdout: &mut std::io::Stdout,
) {
    if editor_values.cursor_y >= buffer.buffer_len() {
        return;
    }
    let file_name = &buffer.read_line(editor_values.cursor_y);
    editor_values.current_file_path = file_manager::SAVE_FILES_PATH.to_string() + file_name;
    file_manager::read_file_into_buffer(buffer, editor_values.current_file_path.to_owned());

    editor_values.in_menu = false;
    drawing::redraw_whole_viewport(buffer, editor_values, stdout);
}
