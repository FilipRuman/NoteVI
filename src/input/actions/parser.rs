use std::{default, i32, usize};

use crate::logger::Logger;

use super::action::Action;

struct ActionParser {
    values: Vec<String>,
    action_name: String,
    line: u16,
}

pub fn complete_parsing_action(name: String, values: Vec<String>, line: u16) -> Action {
    let action_parser = ActionParser {
        action_name: name.to_owned(),
        values,
        line,
    };

    match name.as_str() {
        "MoveCursor" => Action::MoveCursor {
            x: get_value_i32(0, &action_parser),
            y: get_value_i32(1, &action_parser),
        },
        "ChangeCursorPosition" => Action::ChangeCursorPosition {
            x: get_value_i32(0, &action_parser),
            y: get_value_i32(1, &action_parser),
        },

        "InsertLine" => Action::InsertLine,
        "InsertText" => Action::InsertText(safe_aces_value(0, &action_parser)),
        "WriteText" => Action::WriteText(safe_aces_value(0, &action_parser)),
        "RemoveText" => Action::RemoveText {
            from: get_value_usize(0, &action_parser),
            to: get_value_usize(1, &action_parser),
            line: get_value_usize(2, &action_parser),
            move_cursor_back: get_value_bool(3, &action_parser),
        },
        "BackspaceLetters" => Action::BackspaceLetters {
            count: get_value_usize(0, &action_parser),
        },

        "DeleteLetters" => Action::DeleteLetters {
            count: get_value_usize(0, &action_parser),
        },

        "Quit" => Action::Quit,
        "NormalMode" => Action::NormalMode,
        "InsertMode" => Action::InsertMode,
        "ToDo" => Action::ToDo,
        default => {
            Logger::default_log(format!(
                "action name: {:?} at line {} is not known",
                default, action_parser.line
            ));
            panic!();
        }
    }
}
fn get_value_i32(index: usize, action_parser: &ActionParser) -> i32 {
    match safe_aces_value(index, action_parser).parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            Logger::default_log(format!(
                "format of values for action: {:?} at line {} was not right",
                action_parser.action_name, action_parser.line
            ));
            panic!();
        }
    }
}
fn get_value_usize(index: usize, action_parser: &ActionParser) -> usize {
    match safe_aces_value(index, action_parser).parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            Logger::default_log(format!(
                "format of values for action: {:?} at line {} was not right",
                action_parser.action_name, action_parser.line
            ));
            panic!();
        }
    }
}
fn get_value_bool(index: usize, action_parser: &ActionParser) -> bool {
    match safe_aces_value(index, action_parser).parse::<bool>() {
        Ok(val) => val,
        Err(_) => {
            Logger::default_log(format!(
                "format of values for action: {:?} at line {} was not right",
                action_parser.action_name, action_parser.line
            ));
            panic!();
        }
    }
}
fn safe_aces_value(index: usize, action_parser: &ActionParser) -> String {
    if index >= action_parser.values.len() {
        Logger::default_log(format!(
            "amount of values for action: {:?} at line {} was not enough",
            action_parser.action_name, action_parser.line
        ));
        panic!();
    } else {
        let mut raw_string = action_parser.values[index].to_owned();
        raw_string
    }
}
