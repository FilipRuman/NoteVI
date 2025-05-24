use std::rc::{Rc, Weak};

use crate::{
    EditorValues,
    buffer::{self, Buffer},
};

pub struct UndoNode {
    pub line_value: String,
    pub line_y: usize,
    pub cursor_x: usize,
    pub parent: Option<Box<UndoNode>>,
    pub children: Vec<Box<UndoNode>>,
}
pub struct test<'a> {
    pub(crate) test: &'a String,
}
pub fn add_new_node(editor_values: &mut EditorValues) {
    let new_node = UndoNode {
        cursor_x: editor_values.cursor_x,
        line_y: editor_values.cursor_y,
        children: vec![Rc::clone(&editor_values.current_undo_node)],

        parent: None,
        line_value: String::new(),
    };

    editor_values.current_undo_node.parent = Some(Box::new(new_node));
    let new_rc = Rc::new(new_node);

    editor_values.current_undo_node = Rc::clone(&new_rc);
}
pub fn set_current_line_value_for_current_node(editor_values: &mut EditorValues, buffer: &Buffer) {
    editor_values.current_undo_node.line_value = buffer.read_line(editor_values.cursor_y);
}
