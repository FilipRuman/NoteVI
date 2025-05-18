#[derive(Clone, Debug)]
pub enum Action {
    MoveCursor {
        x: i32,
        y: i32,
    },
    ChangeCursorPosition {
        x: i32,
        y: i32,
    },
    InsertText(String),
    WriteText(String),
    RemoveText {
        from: usize,
        to: usize,
        line: usize,
        move_cursor_back: bool,
    },
    BackspaceLetters {
        count: usize,
    },
    DeleteLetters {
        count: usize,
    },

    OpenCurrentFile,
    Quit,
    NormalMode,
    InsertMode,
    ToDo,

    InsertLine,
    DeleteCurrentLine {
        move_to_clipboard: bool,
    },
    GoToEndOfWord {
        characters_are_brakes: bool,
    },
    GoToStartOfWord {
        characters_are_brakes: bool,
    },

    SelectCurrentLine,
    SelectCurrentWords {
        characters_are_brakes: bool,
    },
    DeleteSelection {
        move_to_clipboard: bool,
    },
    DebugSelection,

    CopySelected,
    PasteFromClipboard,
    SaveBuffer,
    //TODO add parsing
    // StartSelection,
    // EndSelection,
    //
    // MoveSelected,
}
