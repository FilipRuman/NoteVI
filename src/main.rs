mod action_handler;
mod buffer;
#[path = "./input/input_handler.rs"]
mod input_handler;
#[path = "./input/key_handler.rs"]
mod key_handler;
#[path = "./lexer/lexer.rs"]
mod lexer;
#[path = "./lexer/tokens.rs"]
mod tokens;

#[path = "./input/actions_parser.rs"]
mod actions_parser;
#[path = "./debugging/logger.rs"]
mod logger;
mod selection_manager;
#[path = "./input/shortcuts.rs"]
mod shortcuts;
#[path = "./input/shortcuts_parser.rs"]
mod shortcuts_parser;

use action_handler::Action;
use buffer::Buffer;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, read},
    terminal::{self, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
};
use key_handler::KeyHandler;
use logger::Logger;
use std::{
    io::{Write, stdout},
    path,
};

pub struct EditorValues {
    cursor_y: usize,
    cursor_x: usize,
    // it is needed so, when you move your cursor thru text with "holes", your cursor stays at the
    // original x
    desired_cursor_x: usize,
    mode: EditMode,
    quit: bool,
}
#[derive(Debug)]
pub enum EditMode {
    Normal,
    Insert,
}

pub const LOGGING_PATH: &str = "./logs/logs.md";
fn main() {
    let mut logger = Logger::new(LOGGING_PATH.to_string());

    // logger.log(format!(
    //     "Test : {:?}",
    //     Shortcut::new_parse_keystrokes("_c", [Action::NormalMode].to_vec())
    // ));
    logger.log("\n\n# ------------    init    ------------\n\n".to_string());

    // logger.log(format!(
    //     "## test shortcut parse 1.: {:?}",
    //     Shortcut::new_parse_keystrokes("IoWo_DD", [Action::ToDo].to_vec())
    // ));
    // logger.log(format!(
    //     "## test shortcut parse 2.: {:?}",
    //     Shortcut::new_parse_keystrokes("_I_DD", [Action::InsertMode].to_vec())
    // ));

    let mut editor_values = EditorValues {
        desired_cursor_x: 0,
        cursor_y: 1,
        cursor_x: 0,
        mode: EditMode::Normal,
        quit: false,
    };

    let mut buffer = Buffer::new();
    let mut stdout = init(&editor_values);
    logger.log("# Start key handler".to_string());

    let shortcuts_output = shortcuts_parser::parse_shortcuts_to_key_handler();

    let mut key_handler = KeyHandler::new(shortcuts_output.normal, shortcuts_output.insert);

    logger.log("\n # start end \n".to_string());

    loop {
        stdout.flush().unwrap();

        match read().unwrap() {
            crossterm::event::Event::Key(event) => {
                let actions =
                    input_handler::handle_input(&mut editor_values, event, &mut key_handler);
                action_handler::handle_actions(
                    &mut editor_values,
                    &mut stdout,
                    actions,
                    &mut buffer,
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
