use crate::{
    EditorValues,
    buffer::Buffer,
    file_manager,
    input::actions::drawing::{self},
    logger::Logger,
};

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
    let file_name = &buffer.read_line(editor_values.cursor_y);
    editor_values.current_file_path = file_manager::SAVE_FILES_PATH.to_string() + file_name;
    file_manager::read_file_into_buffer(buffer, editor_values.current_file_path.to_owned());

    editor_values.in_menu = false;
    drawing::redraw_whole_viewport(buffer, editor_values, stdout);
}
