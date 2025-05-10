

# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 13 }, Token { kind: OpenCurly, value: "{", line: 13 }, Token { kind: String, value: "jj", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "NormalMode", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: CloseCurly, value: "}", line: 18 }, Token { kind: EndOfFile, value: "Eof", line: 19 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
##### Expected: OpenParen but found: Token { kind: SemiColon, value: ";", line: 14 } 


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 13 }, Token { kind: OpenCurly, value: "{", line: 13 }, Token { kind: String, value: "jj", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "NormalMode", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: CloseCurly, value: "}", line: 18 }, Token { kind: EndOfFile, value: "Eof", line: 19 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 13 }, Token { kind: OpenCurly, value: "{", line: 13 }, Token { kind: String, value: "jj", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "NormalMode", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "<Escape>", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Backspace>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "BackspaceLetters", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Delete>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "DeleteLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: CloseCurly, value: "}", line: 18 }, Token { kind: EndOfFile, value: "Eof", line: 19 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('c') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('}') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('}') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(';') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('}') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: PageUp } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('(') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('c') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('#') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('#') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('#') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('#') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('y') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('(') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('[') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('[') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('[') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('[') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('0') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('0') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('2') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('3') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('0') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('2') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('<') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('<') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('<') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('<') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(',') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(',') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('{') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Right } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Tab } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Right } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: true, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Tab } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('g') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('d') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Down } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('v') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Delete } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('v') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: true, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Enter } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('k') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
tokens: [Token { kind: Identifier, value: "normal", line: 0 }, Token { kind: OpenCurly, value: "{", line: 0 }, Token { kind: String, value: "q", line: 1 }, Token { kind: Colon, value: ":", line: 1 }, Token { kind: Identifier, value: "Quit", line: 1 }, Token { kind: OpenParen, value: "(", line: 1 }, Token { kind: CloseParen, value: ")", line: 1 }, Token { kind: SemiColon, value: ";", line: 1 }, Token { kind: String, value: "tt", line: 2 }, Token { kind: Colon, value: ":", line: 2 }, Token { kind: Identifier, value: "WriteText", line: 2 }, Token { kind: OpenParen, value: "(", line: 2 }, Token { kind: Identifier, value: "Test", line: 2 }, Token { kind: CloseParen, value: ")", line: 2 }, Token { kind: SemiColon, value: ";", line: 2 }, Token { kind: String, value: "_c", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "ToDo", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_v", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "dd", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "i", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "InsertMode", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "a", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "InsertMode", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "MoveCursor", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Number, value: "1", line: 7 }, Token { kind: Comma, value: ",", line: 7 }, Token { kind: Number, value: "0", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "l", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "MoveCursor", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Number, value: "1", line: 8 }, Token { kind: Comma, value: ",", line: 8 }, Token { kind: Number, value: "0", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "h", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "MoveCursor", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Number, value: "-1", line: 9 }, Token { kind: Comma, value: ",", line: 9 }, Token { kind: Number, value: "0", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "k", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "MoveCursor", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Number, value: "0", line: 10 }, Token { kind: Comma, value: ",", line: 10 }, Token { kind: Number, value: "1", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "j", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "MoveCursor", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Number, value: "0", line: 11 }, Token { kind: Comma, value: ",", line: 11 }, Token { kind: Number, value: "-1", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: CloseCurly, value: "}", line: 12 }, Token { kind: Identifier, value: "insert", line: 14 }, Token { kind: OpenCurly, value: "{", line: 14 }, Token { kind: String, value: "jj", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "NormalMode", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "<Escape>", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "NormalMode", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "<Backspace>", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "BackspaceLetters", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "<Delete>", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "DeleteLetters", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Number, value: "1", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: CloseCurly, value: "}", line: 19 }, Token { kind: EndOfFile, value: "Eof", line: 20 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [ToDo], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('a') }], actions_after_activations: [InsertMode, MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('l') }], actions_after_activations: [MoveCursor { x: 1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('h') }], actions_after_activations: [MoveCursor { x: -1, y: 0 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('k') }], actions_after_activations: [MoveCursor { x: 0, y: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [MoveCursor { x: 0, y: -1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
normal_segments:{[Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(9)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
