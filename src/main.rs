mod action_handler;
mod buffer;
mod input_handler;
use buffer::Buffer;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyEvent, read},
    style::{self, Stylize},
    terminal::{self, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    boxed,
    io::{Cursor, Write, stdout},
};

pub struct EditorValues {
    cursor_y: usize,
    cursor_x: usize,
    mode: EditMode,
    quit: bool,
}
pub enum EditMode {
    Normal,
    Insert,
}

fn main() {
    let mut editor_values = EditorValues {
        cursor_y: 1,
        cursor_x: 0,
        mode: EditMode::Normal,
        quit: false,
    };

    let mut buffer = Buffer::new();
    let mut stdout = init(&editor_values);

    loop {
        stdout.flush().unwrap();

        match read().unwrap() {
            crossterm::event::Event::Key(event) => {
                let actions = input_handler::handle_input(&mut editor_values, event);
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

fn exit(mut stdout: std::io::Stdout) {
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    stdout.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode();
}

fn init(editor_values: &EditorValues) -> std::io::Stdout {
    let mut stdout = stdout();
    terminal::enable_raw_mode();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(DisableLineWrap).unwrap();

    stdout
        .queue(cursor::MoveTo(
            editor_values.cursor_x as u16,
            editor_values.cursor_y as u16 - 1,
        ))
        .expect("cursor move did not Succeed");

    stdout
}
