use crate::{
    action_handler::Action,
    key_handler::{Keystroke, Shortcut},
};

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
