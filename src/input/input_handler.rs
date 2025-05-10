use crossterm::event::Event;
use crossterm::event::{KeyEvent, KeyModifiers};

use crate::actions::action::Action;
use crate::{
    EditorValues,
    actions::action,
    key_handler::{KeyHandler, Keystroke},
};

pub fn handle_input(
    editor_values: &EditorValues,
    event: KeyEvent,
    key_handler: &mut KeyHandler,
) -> Vec<Action> {
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

    key_handler.handle_new_keystroke(key_stroke, &editor_values)
}
