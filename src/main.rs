mod action_handler;
mod buffer;
mod input_handler;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyEvent, read},
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    boxed,
    io::{Write, stdout},
};

pub struct EditorValues {
    cursor_y: u16,
    cursor_x: u16,
    mode: EditMode,
    quit: bool,
}
pub enum EditMode {
    Normal,
    Insert,
}

fn main() {
    let mut editor_values = EditorValues {
        cursor_y: 20,
        cursor_x: 20,
        mode: EditMode::Normal,
        quit: false,
    };

    let mut stdout = init();

    loop {
        stdout.flush().unwrap();

        match read().unwrap() {
            crossterm::event::Event::Key(event) => {
                let actions = input_handler::handle_input(&mut editor_values, event);
                action_handler::handle_actions(&mut editor_values, &mut stdout, actions);
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

fn init() -> std::io::Stdout {
    let mut stdout = stdout();
    terminal::enable_raw_mode();
    stdout.execute(EnterAlternateScreen).unwrap();

    stdout
}
