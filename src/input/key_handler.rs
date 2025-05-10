use std::{char, collections::HashMap, fmt::format, ops::ControlFlow, process::Output, usize};

use crossterm::event::KeyCode;

use crate::{EditMode, EditorValues, actions::action::Action, logger::Logger};

pub struct Layer {
    pub shortcuts: Vec<Shortcut>,
    pub segments: HashMap<Vec<Keystroke>, Option<usize>>,
    pub insert_text_on_type: bool,
}
impl Layer {
    pub fn get_actions_for_shortcut_index(&self, index: usize) -> Vec<Action> {
        self.shortcuts[index].actions_after_activations.clone()
    }
}
pub struct KeyHandler {
    pub last_keystrokes: Vec<Keystroke>,
    pub insert_layer: Layer,
    pub normal_layer: Layer,
}

impl KeyHandler {
    pub fn new(normal_shortcuts: Vec<Shortcut>, insert_shortcuts: Vec<Shortcut>) -> KeyHandler {
        let normal_segments = KeyHandler::split_shortcuts_into_segments(&normal_shortcuts);
        let insert_segments = KeyHandler::split_shortcuts_into_segments(&insert_shortcuts);

        Logger::default_log(format!("normal_segments:{:?}", normal_segments));

        KeyHandler {
            last_keystrokes: Vec::new(),
            normal_layer: Layer {
                shortcuts: normal_shortcuts,
                segments: normal_segments,
                insert_text_on_type: false,
            },
            insert_layer: Layer {
                shortcuts: insert_shortcuts,
                segments: insert_segments,
                insert_text_on_type: true,
            },
        }
    }
    fn split_shortcuts_into_segments(
        shortcuts: &Vec<Shortcut>,
    ) -> HashMap<Vec<Keystroke>, Option<usize>> {
        let mut output = HashMap::new();

        for shortcut_index in 0..shortcuts.len() {
            let shortcut = &shortcuts[shortcut_index];

            let segments = &shortcut.get_segments();
            let length = shortcut.length;
            for i in 0..length {
                let last = length == i + 1;
                let segment = (&segments[i]).to_owned();
                let value = if last {
                    Some(shortcut_index)
                } else {
                    Option::None
                };
                output.insert(segment, value);
            }
        }

        output
    }

    pub fn handle_new_keystroke(
        &mut self,
        keystroke: Keystroke,
        editor_values: &EditorValues,
    ) -> Vec<Action> {
        Logger::default_log(format!(
            "handle_new_keystroke {:?}mode keystroke:{:?} last keystrokes:{:?}",
            editor_values.mode, keystroke, self.last_keystrokes,
        ));

        let layer = match editor_values.mode {
            EditMode::Normal => &self.normal_layer,
            EditMode::Insert => &self.insert_layer,
        };
        let mut output = Vec::new();

        // handles typing text in insert mode
        if layer.insert_text_on_type {
            output.push(Action::WriteText(type_text_in_insert_mode(&keystroke)));
        }

        self.last_keystrokes.push(keystroke);
        // searches current keystrokes in a HashMap to see if they are a shortcut
        let shortcut_from_current_keystrokes = layer.segments.get(&self.last_keystrokes);
        match shortcut_from_current_keystrokes {
            Some(shortcut_option) => match shortcut_option {
                Some(shortcut_index) => {
                    // whole shortcut matches
                    if layer.insert_text_on_type {
                        // remove the char that you have just typed
                        output.remove(output.len() - 1);
                        self.remove_last_typed_keystrokes(&mut output);
                    }

                    self.last_keystrokes.clear();
                    output.append(
                        &mut layer.get_actions_for_shortcut_index(shortcut_index.to_owned()),
                    );
                }
                // there is a shortcut that starts with those keystrokes
                None => {}
            },

            None => {
                // there is no shortcut containing this sequence of keystrokes on the beginning
                self.last_keystrokes.clear();
            }
        }
        output
    }
    fn remove_last_typed_keystrokes(&self, output: &mut Vec<Action>) {
        let mut count = 0;
        for keystroke in &self.last_keystrokes {
            count += get_length_of_typed_text_by_keystroke(keystroke);
        }
        if count > 0 {
            output.push(Action::BackspaceLetters { count: count - 1 });
        }
    }
}

fn type_text_in_insert_mode(keystroke: &Keystroke) -> String {
    // I do this so when i press like backspace or space i don't write "backspace" as a
    // word
    let mut output = match keystroke.keycode {
        crossterm::event::KeyCode::Char(char) => char.to_string(),
        _ => {
            return String::new();
        }
    };

    // handle characters that come in pairs like ""
    match output.as_str() {
        "\"" => output += "\"",
        "\'" => output += "\'",
        "<" => output += ">",
        "(" => output += ")",
        "{" => output += "}",
        "[" => output += "]",

        _ => {}
    }

    return output;
}
fn get_length_of_typed_text_by_keystroke(keystroke: &Keystroke) -> usize {
    let text = type_text_in_insert_mode(keystroke);
    return text.len();
}

#[derive(Clone, Debug)]
pub struct Shortcut {
    pub keystrokes_to_activate: Vec<Keystroke>,
    pub actions_after_activations: Vec<Action>,
    pub length: usize,
}
impl Shortcut {
    pub fn new(
        keystrokes_to_activate: Vec<Keystroke>,
        actions_after_activations: Vec<Action>,
    ) -> Shortcut {
        let length = &keystrokes_to_activate.len();
        Shortcut {
            keystrokes_to_activate,
            actions_after_activations,
            length: *length,
        }
    }
    pub fn new_parse_keystrokes(
        keystrokes: &str,
        actions_after_activations: Vec<Action>,
    ) -> Shortcut {
        let mut ctrl = false;
        let mut parsed: Vec<Keystroke> = Vec::new();
        let chars = keystrokes.chars().collect::<Vec<char>>();
        let length = keystrokes.len();

        let mut i = 0;
        while i < length {
            let char = chars[i];
            if char == '_' {
                ctrl = true;
                i += 1;
                continue;
            }
            if char == '<' {
                //Move past <
                i += 1;
                let mut text_inside = String::new();
                while chars[i] != '>' {
                    text_inside += &chars[i].to_string();
                    i += 1;
                }
                //Move past >
                i += 1;

                let keystroke = match match_string_inside_caret_to_a_key(ctrl, text_inside) {
                    Some(value) => value,
                    None => continue,
                };
                parsed.push(keystroke);
                continue;
            }

            parsed.push(Keystroke {
                ctrl,
                keycode: KeyCode::Char(char),
            });
            ctrl = false;
            i += 1;
        }
        let output = Shortcut::new(parsed, actions_after_activations);
        Logger::default_log(format!("Parsed shortcut: {:?}", output,));
        return output;
    }
    // splits keystrokes to vectors of higher and higher lengths so you can later see if current
    // keystrokes could lead to a shortcut or not
    pub fn get_segments(&self) -> Vec<Vec<Keystroke>> {
        let mut output: Vec<Vec<Keystroke>> = Vec::with_capacity(self.length);
        for segment_size in 1..self.length + 1 {
            let mut segment = Vec::with_capacity(segment_size);
            for keystroke_index in 0..segment_size {
                segment.push(self.keystrokes_to_activate[keystroke_index].clone());
            }
            output.push(segment);
        }

        // Logger::default_log(format!(
        //     "get_segments: keystrokes: {:?} ||| segments: {:?} ",
        //     self.keystrokes_to_activate, output
        // ));

        output
    }
}

fn match_string_inside_caret_to_a_key(ctrl: bool, text_inside: String) -> Option<Keystroke> {
    let keystroke = match text_inside.as_str() {
        "Escape" => Keystroke {
            ctrl,
            keycode: KeyCode::Esc,
        },
        "Enter" => Keystroke {
            ctrl,
            keycode: KeyCode::Enter,
        },
        "Delete" => Keystroke {
            ctrl,
            keycode: KeyCode::Delete,
        },
        "Backspace" => Keystroke {
            ctrl,
            keycode: KeyCode::Backspace,
        },
        "Tab" => Keystroke {
            ctrl,
            keycode: KeyCode::Tab,
        },
        _ => {
            return None;
        }
    };
    Some(keystroke)
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Keystroke {
    pub ctrl: bool,
    pub keycode: KeyCode,
}
