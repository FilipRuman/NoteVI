use std::io::Stdout;

use crossterm::event::Event;
use crossterm::event::{KeyEvent, KeyModifiers};

use crate::EditorValues;
use crate::buffer::Buffer;
use crate::clipboard::Clipboard;
use crate::logger::Logger;
use crate::selection_manager::SelectionManager;

use super::actions::action::Action;
use super::actions::handler::handle_actions;
use super::key_handler::{KeyHandler, Keystroke};
use super::{actions, shortcuts_parser};

pub fn startup() -> KeyHandler {
    let shortcuts_output = shortcuts_parser::parse_shortcuts_to_key_handler();

    Logger::default_log("input_handler::startup".to_string());
    return KeyHandler::new(
        shortcuts_output.visual,
        shortcuts_output.normal,
        shortcuts_output.insert,
    );
}
pub fn handle_key_input(
    editor_values: &mut EditorValues,
    event: KeyEvent,
    key_handler: &mut KeyHandler,
    stdout: &mut Stdout,
    buffer: &mut Buffer,
    selection_manager: &mut SelectionManager,
    clipboard: &mut Clipboard,
) {
    let mut ctrl = false;
    for modifier in event.modifiers.iter() {
        match modifier {
            KeyModifiers::CONTROL => ctrl = true,
            _ => {}
        }
    }

    let key_stroke = Keystroke {
        ctrl,
        keycode: event.code,
    };

    let actions = key_handler.handle_new_keystroke(key_stroke, &editor_values);
    handle_actions(
        editor_values,
        stdout,
        actions,
        buffer,
        selection_manager,
        clipboard,
    );
}
