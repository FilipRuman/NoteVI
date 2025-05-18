mod buffer;
mod buffer_parser;
mod clipboard;
mod file_manager;
mod input;
mod lexer;
mod logger;
mod menu;
mod selection_manager;
mod text_formatting;

use buffer::Buffer;
use clipboard::Clipboard;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, read},
    terminal::{self, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
};
use input::input_handler::{self, handle_key_input};
use logger::{LOGGING_PATH, Logger};
use selection_manager::SelectionManager;
use std::io::{Write, stdout};

pub struct EditorValues {
    cursor_y: usize,
    cursor_x: usize,
    // it is needed so, when you move your cursor thru text with "hols", your cursor stays at the
    // original
    current_file_path: String,
    desired_cursor_x: usize,
    mode: EditMode,
    quit: bool,
    file_names: Vec<String>,
    in_menu: bool,
}
#[derive(Debug)]
pub enum EditMode {
    Normal,
    Insert,
    Visual,
}

fn main() {
    let mut logger = Logger::new(LOGGING_PATH.to_string());
    logger.log("\n\n# ------------    init    ------------\n\n".to_string());

    let mut selection_manager = SelectionManager {
        from_y: 0,
        to_x: 0,
        from_x: 0,
        to_y: 0,
        selection_mode_start_y: 0,
        selection_mode_start_x: 0,
    };

    let mut editor_values = EditorValues {
        current_file_path: String::new(),
        desired_cursor_x: 0,
        cursor_y: 1,
        cursor_x: 0,
        mode: EditMode::Normal,
        quit: false,
        in_menu: true,
        file_names: Vec::new(),
    };
    let mut clipboard = Clipboard { save: Vec::new() };

    let mut buffer = Buffer::new();
    let mut stdout = init(&editor_values);
    logger.log("# Start key handler".to_string());
    let mut key_handler = input::input_handler::startup();

    editor_values.file_names =
        file_manager::get_all_files_in_directory(file_manager::SAVE_FILES_PATH.to_string());
    menu::display_file_selection(&mut buffer, &mut editor_values, &mut stdout);

    logger.log("\n # start end \n".to_string());

    loop {
        stdout.flush().unwrap();

        match read().unwrap() {
            crossterm::event::Event::Key(event) => {
                handle_key_input(
                    &mut editor_values,
                    event,
                    &mut key_handler,
                    &mut stdout,
                    &mut buffer,
                    &mut selection_manager,
                    &mut clipboard,
                );
            }
            _ => {}
        };
        if editor_values.quit {
            break;
        }
    }

    exit(stdout);
}
const MOVE_TO_ALTERNATIVE_SCREEN: bool = true;

fn exit(mut stdout: std::io::Stdout) {
    if MOVE_TO_ALTERNATIVE_SCREEN {
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.execute(LeaveAlternateScreen).unwrap();
    }
    terminal::disable_raw_mode();
}

fn init(editor_values: &EditorValues) -> std::io::Stdout {
    let mut stdout = stdout();

    terminal::enable_raw_mode();

    if MOVE_TO_ALTERNATIVE_SCREEN {
        stdout.execute(EnterAlternateScreen).unwrap();
        stdout.execute(DisableLineWrap).unwrap();
    }

    stdout
        .queue(cursor::MoveTo(
            editor_values.cursor_x as u16,
            editor_values.cursor_y as u16 - 1,
        ))
        .expect("cursor move did not Succeed");

    stdout
}
