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

    Quit,
    NormalMode,
    InsertMode,
    ToDo,

    InsertLine,
    DeleteCurrentLine,
    GoToEndOfWord {
        characters_are_brakes: bool,
    },
    GoToStartOfWord {
        characters_are_brakes: bool,
    },
    //TODO add parsing
    // StartSelection,
    // EndSelection,
    // MoveToEndOfWord,
    // MoveToBeginningOfWord,
    //
    // SelectLineBeforeCursor,
    // SelectLineAfterCursor,
    // SelectLine,
    // SelectWord,
    // DeleteSelected,
    // CopySelected,
    // PasteSelected,
    // MoveSelected,
}
