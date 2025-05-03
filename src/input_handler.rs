use std::io::Stdout;

use crossterm::event::KeyEvent;

use crate::{EditMode, EditorValues, action_handler::Action};

pub fn handle_input(editor_values: &EditorValues, event: KeyEvent) -> Vec<Action> {
    match editor_values.mode {
        EditMode::Normal => handle_normal_mode(editor_values, event),
        EditMode::Insert => handle_insert_mode(editor_values, event),
    }
}
fn handle_normal_mode(editor_values: &EditorValues, event: KeyEvent) -> Vec<Action> {
    let mut output: Vec<Action> = Vec::new();

    match event.code {
        crossterm::event::KeyCode::Char('q') => output.push(Action::Quit),

        crossterm::event::KeyCode::Char('h') => output.push(Action::MoveCursor(-1, 0)),
        crossterm::event::KeyCode::Char('l') => output.push(Action::MoveCursor(1, 0)),
        crossterm::event::KeyCode::Char('j') => output.push(Action::MoveCursor(0, -1)),
        crossterm::event::KeyCode::Char('k') => output.push(Action::MoveCursor(0, 1)),

        crossterm::event::KeyCode::Char('i') => output.push(Action::InsertMode),
        crossterm::event::KeyCode::Char('a') => {
            output.push(Action::InsertMode);
            output.push(Action::MoveCursor(1, 0));
        }
        _ => {}
    };

    output
}

fn handle_insert_mode(editor_values: &EditorValues, event: KeyEvent) -> Vec<Action> {
    let mut output: Vec<Action> = Vec::new();

    match event.code {
        crossterm::event::KeyCode::Char(c) => {
            output.push(Action::WriteText(c.to_string()));
        }
        crossterm::event::KeyCode::Esc => {
            output.push(Action::NormalMode);
        }
        _ => {}
    };
    output
}
