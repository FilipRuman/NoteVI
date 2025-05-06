use crate::{
    action_handler::Action,
    key_handler::{Keystroke, Shortcut},
};

// pub fn get_shortcuts_normal() -> Vec<Shortcut> {
//     [
//         Shortcut::new_parse_keystrokes("_c", [Action::ToDo].to_vec()),
//         Shortcut::new_parse_keystrokes("_v", [Action::ToDo].to_vec()),
//         Shortcut::new_parse_keystrokes("dd", [Action::ToDo].to_vec()),
//         Shortcut::new_parse_keystrokes("i", [Action::InsertMode].to_vec()),
//         Shortcut::new_parse_keystrokes(
//             "a",
//             [Action::InsertMode, Action::MoveCursor { x: 1, y: 0 }].to_vec(),
//         ),
//         //
//         Shortcut::new_parse_keystrokes("l", [Action::MoveCursor { x: 1, y: 0 }].to_vec()),
//         Shortcut::new_parse_keystrokes("h", [Action::MoveCursor { x: -1, y: 0 }].to_vec()),
//         Shortcut::new_parse_keystrokes("k", [Action::MoveCursor { x: 0, y: 1 }].to_vec()),
//         Shortcut::new_parse_keystrokes("j", [Action::MoveCursor { x: 0, y: -1 }].to_vec()),
//         //
//         Shortcut::new_parse_keystrokes("q", [Action::Quit].to_vec()),
//     ]
//     .to_vec()
// }
pub fn get_shortcuts_input() -> Vec<Shortcut> {
    [
        Shortcut::new_parse_keystrokes("jj", [Action::NormalMode].to_vec()),
        Shortcut::new_parse_keystrokes("<Escape>", [Action::NormalMode].to_vec()),
        Shortcut::new_parse_keystrokes(
            "<Backspace>",
            [Action::BackspaceLetters { count: 1 }].to_vec(),
        ),
        Shortcut::new_parse_keystrokes("<Delete>", [Action::DeleteLetters { count: 1 }].to_vec()),
    ]
    .to_vec()
}
