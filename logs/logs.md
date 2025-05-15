
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "tt", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "WriteText", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: Identifier, value: "Test", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_c", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "_v", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "ToDo", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "dd", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daw", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "true", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "daW", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "false", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: Identifier, value: "DeleteSelection", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saw", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "true", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "saW", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: Identifier, value: "false", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "i", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "a", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "InsertMode", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "l", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "h", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "-1", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "k", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "j", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "MoveCursor", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Number, value: "0", line: 17 }, Token { kind: Comma, value: ",", line: 17 }, Token { kind: Number, value: "-1", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "e", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "true", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "E", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "false", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "b", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "true", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "B", line: 21 }, Token { kind: Colon, value: ":", line: 21 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 21 }, Token { kind: OpenParen, value: "(", line: 21 }, Token { kind: Identifier, value: "false", line: 21 }, Token { kind: CloseParen, value: ")", line: 21 }, Token { kind: SemiColon, value: ";", line: 21 }, Token { kind: CloseCurly, value: "}", line: 23 }, Token { kind: Identifier, value: "insert", line: 26 }, Token { kind: OpenCurly, value: "{", line: 26 }, Token { kind: String, value: "jj", line: 27 }, Token { kind: Colon, value: ":", line: 27 }, Token { kind: Identifier, value: "NormalMode", line: 27 }, Token { kind: OpenParen, value: "(", line: 27 }, Token { kind: CloseParen, value: ")", line: 27 }, Token { kind: SemiColon, value: ";", line: 27 }, Token { kind: String, value: "<Escape>", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Backspace>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "BackspaceLetters", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: Number, value: "1", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Delete>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "DeleteLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Enter>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "InsertLine", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: CloseCurly, value: "}", line: 33 }, Token { kind: EndOfFile, value: "Eof", line: 34 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('k') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(2), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('t') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs dasfda fsdsfd fs dfsdsfdsfdasfd sa dff d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 2 
remove_text: output sasdf s dfsfd sfd sfd sfd
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,20 self.to_x,32 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d fs dfsdsfdsfdasfd sa dff d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,20 self.to_x,23 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output saasdfasfdf af dafs s dfsdsfdsfdasfd sa dff d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,20 self.to_x,35 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Left } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d saII dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d saI dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa""" dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa"" dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa" dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('>') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('<') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa< dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('"') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa<" dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa< dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa' dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(',') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\\') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa dff d
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('|') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('|') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('|') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa||| dff d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('|') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('|') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('|') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d sa||| dff  |||d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,22 self.to_x,30 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output saasdfasfdf af dafs d f  |||d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('t') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('t') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output Test 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('t') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output Test Test 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('t') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output Test Test Test
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('s') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,5 self.to_x,8 self.from_y,1 self.to_y1
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('t') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,5 self.to_x,8 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output Test t Test
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  safsdafdsa  fdsaffsd afdsfsdfaa sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  safsdafdsa   fdsaffsd afdsfsdfaa sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  safsdafdsa  /// fdsaffsd afdsfsdfaa sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  safsdafdsa  /// fdsaffsd  /// afdsfsdfaa sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,17 self.to_x,24 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  /// afdsfsdfaa sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  /// afdsfsdfa sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  /// afdsfsdfaO sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
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
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  /// fdsfsdfaO sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  /// IfdsfsdfaO sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,23 self.to_x,32 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  ///O sfdf sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  ///O sfd sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('F') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  ///O sfdF sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,24 self.to_x,27 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output  safsdafdsa  ///d  ///OF sdafs dfs dafs daffdsafsdaf dsa safdfdsa
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output sa sa dsfsadffd s dfsaafsd afdsa dfsad sfas dfasf dafs dafd safsd a fsdsf da sfd
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output sa sa dsfsadffd s dfsaafsd afdsa dfsad ... sfas dfasf dafs dafd safsd a fsdsf da sfd
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output sa sa dsfsadffd s dfsaafsd afdsa dfsad ... sfas ... dfasf dafs dafd safsd a fsdsf da sfd
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,43 self.to_x,48 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output sa sa dsfsadffd s dfsaafsd afdsa dfsad ... .. dfasf dafs dafd safsd a fsdsf da sfd
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Up } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf ds dfsf d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf ds dfsf ... d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf ds ... dfsf ... d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,46 self.to_x,51 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf ds ... ... d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf dws ... ... d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,39 self.to_x,43 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf ... ... d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,45 self.to_x,48 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds fdasf ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,33 self.to_x,37 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas fdsafds f ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,25 self.to_x,31 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa dfas fdasfdas s f ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,16 self.to_x,23 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa dfas s s f ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s sasfd sa dfasI s s f ... ..
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s sasfd sa dfasI s s f ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,11 self.to_x,15 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s sasfd sa I s s f ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,2 self.to_x,9 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('y') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('s') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,10 self.to_x,14 self.from_y,1 self.to_y1
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('t') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('r') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n nn n n nnnnnnnnnnnnn
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n nn n  nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n nn n nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n nn  nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n nn nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n n nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n  nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn n nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn  nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nn nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  n nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn   nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn  nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnnn nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nnn nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    nn nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    n nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..     nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    IIIIIII nnnnnnnnnnnnn
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
remove_text: line 1 
remove_text: output s a I s s f ... ..    IIIIII nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    IIIII nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    IIII nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    III nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    II nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    I nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..     nnnnnnnnnnnnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnnnnnnnn
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnnnnnnnn '''''
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnnnnnnnn ''''
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnnnnnnnn '''
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnnnnnnnn ''
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('p') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnnnnnnnn ''p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,26 self.to_x,38 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// n ''p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    ///  ''p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnI ''p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnn ''p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// nnnnnnP ''p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// KnnnnnnP ''p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,26 self.to_x,37 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// ... p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// PoooI ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,26 self.to_x,30 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// I ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    ///  ... p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// Poooi ... p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// Pooo ... p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// PoooI ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,26 self.to_x,30 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// I ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    ///  ... p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// PoooI ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,26 self.to_x,30 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// I ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// PoooooooI ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,26 self.to_x,36 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s a I s s f ... ..    /// PooooooI ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('b') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,24 self.to_x,33 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output s a I s s f ... ..    //I ... p
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s a I s s f ... ..    // ... p
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s  sfdassf dfd asdf safff fsdfds asfad f 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s  sfdassf dfd asdf safff /// fsdfds asfad f 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output s  sfdassf dfd asdf /// safff /// fsdfds asfad f 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  asfd asfd afsd af sdffas dasf d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  asfd asfd afsd af sdffas /// dasf d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  asfd asfd afsd af /// sdffas /// dasf d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,22 self.to_x,29 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output  asfd asfd afsd af /// /// dasf d
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output sfaasf dfd sasfad f df sad fsaafd sfs daffd as fdsasadf
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,37 self.to_x,43 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output sfaasf dfd sasfad f df sad fsaafd sfs as fdsasadf
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('F') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('D') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('F') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: F(1) } last keystrokes:[]
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
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa sadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ''' KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa '' KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ' KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa  KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa  KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(',') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(',') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(',') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP ffadsdfsafdas
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP  ''' ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP  '' ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP  ' ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP   ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP  ... ffadsdfsafdas
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP ... ffadsdfsafdas
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, KsadffdsafdsadsfadfsaP ... ffadsdfsafdas
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
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
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,34 self.to_x,57 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, ... ffadsdfsafdas
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,38 self.to_x,51 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, ...s
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, ...
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output fFDFddfaddsaf safdfdsaafsdfdsa ,,, ...
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,13 self.to_x,30 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output fFDFddfaddsaf ,,, ...
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  OOO
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  OO
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  O
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(':') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(':') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(':') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  ::: OO
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  ::: O
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(':') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(':') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(':') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  ::: OIO :::
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  ::: OIIO :::
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  ::: OII :::
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  ::: OIIP :::
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,9 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output  ::: :::
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  ::: OIIIIP :::
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,4 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: out of the line bounds from:4 to:4 line_len:15
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,4 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: out of the line bounds from:4 to:4 line_len:15
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,4 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: out of the line bounds from:4 to:4 line_len:15
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,11 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output  ::: :::


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output  ::: sadf :::
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('t') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('L') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('L') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('L') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('L') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output LLL
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output LL
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output L
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// OIIIIIP /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// OIIIIIP /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,12 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output //// /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// PPPP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// PPP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// PP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// P/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// IIIOOOOPPP/////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// IIIOOOOPP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// IIIOOOOP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// IIIOOOOP/////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// IIIhOOOOP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// IIIOOOOP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// IIIOOOOP/////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// IIOOOOP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// IOOOOP/////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// IOOOOP/////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// IOOOOP /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,11 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output //// /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('N') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('N') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////IIIIIN /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////IIIII /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////IIII /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////III /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////II /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////I /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////IOP /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output ////IOP /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output ////IOOOOP /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// IOOOOP /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,11 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output //// /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// OOOP /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// OOO /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// OO /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //// O /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output //// OPI /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ////OPI /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output ////OPI /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,3 self.to_x,7 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output /// /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output // /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output / /////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output / /////
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output / ////
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output / ///
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output / //
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output / /
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output /OPI/
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('%') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('W') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('%') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('!') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('%') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('W') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('%') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('W') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output /OPI/ 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,0 self.to_x,4 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output / 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output /OPI/ 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output /OPI/ 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output /OPwI/ 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output /OPwI/ 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output /OPI/ 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output /OPI/ 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,0 self.to_x,4 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output / 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output / 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('`') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output // 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output // 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output //
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output /
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: out of the line bounds from:0 to:0 line_len:0
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output self.test
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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output owself.test
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output owo.self.test
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,3 self.to_x,8 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output owo.test
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output owo.self.test
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.I 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.OIIIIIP
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('.') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('t') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads.OIIIIIP.tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,7 self.to_x,14 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads..tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('F') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('N') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.IIF.tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.II.tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.I.tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.IOO.tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.IO.tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads.I.tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads..tets
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads.OIIIIP.tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,7 self.to_x,13 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads.OIIIIIIIP.tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,7 self.to_x,16 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads..tets
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('N') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('N') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('M') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('M') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads..tets IIIKKKNNMMKOIIO
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,13 self.to_x,27 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets O
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads..tets 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads..tets OIIIIIIIIIP
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,13 self.to_x,23 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets P
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads..tets 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads..tets OIIIIIIIIIIIP
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,13 self.to_x,25 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets P
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads..tets 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('N') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads..tets OO
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output safads..tets O
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('P') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output safads..tets OIIIIIIP     
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,13 self.to_x,21 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output safads..tets      
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------


# Start key handler
parse_shortcuts_to_key_handler
tokens: [Token { kind: Identifier, value: "normal", line: 2 }, Token { kind: OpenCurly, value: "{", line: 2 }, Token { kind: String, value: "q", line: 3 }, Token { kind: Colon, value: ":", line: 3 }, Token { kind: Identifier, value: "Quit", line: 3 }, Token { kind: OpenParen, value: "(", line: 3 }, Token { kind: CloseParen, value: ")", line: 3 }, Token { kind: SemiColon, value: ";", line: 3 }, Token { kind: String, value: "_c", line: 4 }, Token { kind: Colon, value: ":", line: 4 }, Token { kind: Identifier, value: "ToDo", line: 4 }, Token { kind: OpenParen, value: "(", line: 4 }, Token { kind: CloseParen, value: ")", line: 4 }, Token { kind: SemiColon, value: ";", line: 4 }, Token { kind: String, value: "_v", line: 5 }, Token { kind: Colon, value: ":", line: 5 }, Token { kind: Identifier, value: "ToDo", line: 5 }, Token { kind: OpenParen, value: "(", line: 5 }, Token { kind: CloseParen, value: ")", line: 5 }, Token { kind: SemiColon, value: ";", line: 5 }, Token { kind: String, value: "dd", line: 6 }, Token { kind: Colon, value: ":", line: 6 }, Token { kind: Identifier, value: "DeleteCurrentLine", line: 6 }, Token { kind: OpenParen, value: "(", line: 6 }, Token { kind: CloseParen, value: ")", line: 6 }, Token { kind: SemiColon, value: ";", line: 6 }, Token { kind: String, value: "daw", line: 7 }, Token { kind: Colon, value: ":", line: 7 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: Identifier, value: "true", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: Identifier, value: "DeleteSelection", line: 7 }, Token { kind: OpenParen, value: "(", line: 7 }, Token { kind: CloseParen, value: ")", line: 7 }, Token { kind: SemiColon, value: ";", line: 7 }, Token { kind: String, value: "daW", line: 8 }, Token { kind: Colon, value: ":", line: 8 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: Identifier, value: "false", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: Identifier, value: "DeleteSelection", line: 8 }, Token { kind: OpenParen, value: "(", line: 8 }, Token { kind: CloseParen, value: ")", line: 8 }, Token { kind: SemiColon, value: ";", line: 8 }, Token { kind: String, value: "saw", line: 9 }, Token { kind: Colon, value: ":", line: 9 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 9 }, Token { kind: OpenParen, value: "(", line: 9 }, Token { kind: Identifier, value: "true", line: 9 }, Token { kind: CloseParen, value: ")", line: 9 }, Token { kind: SemiColon, value: ";", line: 9 }, Token { kind: String, value: "saW", line: 10 }, Token { kind: Colon, value: ":", line: 10 }, Token { kind: Identifier, value: "SelectCurrentWords", line: 10 }, Token { kind: OpenParen, value: "(", line: 10 }, Token { kind: Identifier, value: "false", line: 10 }, Token { kind: CloseParen, value: ")", line: 10 }, Token { kind: SemiColon, value: ";", line: 10 }, Token { kind: String, value: "i", line: 11 }, Token { kind: Colon, value: ":", line: 11 }, Token { kind: Identifier, value: "InsertMode", line: 11 }, Token { kind: OpenParen, value: "(", line: 11 }, Token { kind: CloseParen, value: ")", line: 11 }, Token { kind: SemiColon, value: ";", line: 11 }, Token { kind: String, value: "a", line: 12 }, Token { kind: Colon, value: ":", line: 12 }, Token { kind: Identifier, value: "InsertMode", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: Identifier, value: "MoveCursor", line: 12 }, Token { kind: OpenParen, value: "(", line: 12 }, Token { kind: Number, value: "1", line: 12 }, Token { kind: Comma, value: ",", line: 12 }, Token { kind: Number, value: "0", line: 12 }, Token { kind: CloseParen, value: ")", line: 12 }, Token { kind: SemiColon, value: ";", line: 12 }, Token { kind: String, value: "l", line: 13 }, Token { kind: Colon, value: ":", line: 13 }, Token { kind: Identifier, value: "MoveCursor", line: 13 }, Token { kind: OpenParen, value: "(", line: 13 }, Token { kind: Number, value: "1", line: 13 }, Token { kind: Comma, value: ",", line: 13 }, Token { kind: Number, value: "0", line: 13 }, Token { kind: CloseParen, value: ")", line: 13 }, Token { kind: SemiColon, value: ";", line: 13 }, Token { kind: String, value: "h", line: 14 }, Token { kind: Colon, value: ":", line: 14 }, Token { kind: Identifier, value: "MoveCursor", line: 14 }, Token { kind: OpenParen, value: "(", line: 14 }, Token { kind: Number, value: "-1", line: 14 }, Token { kind: Comma, value: ",", line: 14 }, Token { kind: Number, value: "0", line: 14 }, Token { kind: CloseParen, value: ")", line: 14 }, Token { kind: SemiColon, value: ";", line: 14 }, Token { kind: String, value: "k", line: 15 }, Token { kind: Colon, value: ":", line: 15 }, Token { kind: Identifier, value: "MoveCursor", line: 15 }, Token { kind: OpenParen, value: "(", line: 15 }, Token { kind: Number, value: "0", line: 15 }, Token { kind: Comma, value: ",", line: 15 }, Token { kind: Number, value: "1", line: 15 }, Token { kind: CloseParen, value: ")", line: 15 }, Token { kind: SemiColon, value: ";", line: 15 }, Token { kind: String, value: "j", line: 16 }, Token { kind: Colon, value: ":", line: 16 }, Token { kind: Identifier, value: "MoveCursor", line: 16 }, Token { kind: OpenParen, value: "(", line: 16 }, Token { kind: Number, value: "0", line: 16 }, Token { kind: Comma, value: ",", line: 16 }, Token { kind: Number, value: "-1", line: 16 }, Token { kind: CloseParen, value: ")", line: 16 }, Token { kind: SemiColon, value: ";", line: 16 }, Token { kind: String, value: "e", line: 17 }, Token { kind: Colon, value: ":", line: 17 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 17 }, Token { kind: OpenParen, value: "(", line: 17 }, Token { kind: Identifier, value: "true", line: 17 }, Token { kind: CloseParen, value: ")", line: 17 }, Token { kind: SemiColon, value: ";", line: 17 }, Token { kind: String, value: "E", line: 18 }, Token { kind: Colon, value: ":", line: 18 }, Token { kind: Identifier, value: "GoToEndOfWord", line: 18 }, Token { kind: OpenParen, value: "(", line: 18 }, Token { kind: Identifier, value: "false", line: 18 }, Token { kind: CloseParen, value: ")", line: 18 }, Token { kind: SemiColon, value: ";", line: 18 }, Token { kind: String, value: "b", line: 19 }, Token { kind: Colon, value: ":", line: 19 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 19 }, Token { kind: OpenParen, value: "(", line: 19 }, Token { kind: Identifier, value: "true", line: 19 }, Token { kind: CloseParen, value: ")", line: 19 }, Token { kind: SemiColon, value: ";", line: 19 }, Token { kind: String, value: "B", line: 20 }, Token { kind: Colon, value: ":", line: 20 }, Token { kind: Identifier, value: "GoToStartOfWord", line: 20 }, Token { kind: OpenParen, value: "(", line: 20 }, Token { kind: Identifier, value: "false", line: 20 }, Token { kind: CloseParen, value: ")", line: 20 }, Token { kind: SemiColon, value: ";", line: 20 }, Token { kind: String, value: "tw", line: 22 }, Token { kind: Colon, value: ":", line: 22 }, Token { kind: Identifier, value: "WriteText", line: 22 }, Token { kind: OpenParen, value: "(", line: 22 }, Token { kind: Identifier, value: "Test", line: 22 }, Token { kind: CloseParen, value: ")", line: 22 }, Token { kind: SemiColon, value: ";", line: 22 }, Token { kind: String, value: "ts", line: 23 }, Token { kind: Colon, value: ":", line: 23 }, Token { kind: Identifier, value: "DebugSelection", line: 23 }, Token { kind: OpenParen, value: "(", line: 23 }, Token { kind: CloseParen, value: ")", line: 23 }, Token { kind: SemiColon, value: ";", line: 23 }, Token { kind: CloseCurly, value: "}", line: 24 }, Token { kind: Identifier, value: "insert", line: 27 }, Token { kind: OpenCurly, value: "{", line: 27 }, Token { kind: String, value: "jj", line: 28 }, Token { kind: Colon, value: ":", line: 28 }, Token { kind: Identifier, value: "NormalMode", line: 28 }, Token { kind: OpenParen, value: "(", line: 28 }, Token { kind: CloseParen, value: ")", line: 28 }, Token { kind: SemiColon, value: ";", line: 28 }, Token { kind: String, value: "<Escape>", line: 29 }, Token { kind: Colon, value: ":", line: 29 }, Token { kind: Identifier, value: "NormalMode", line: 29 }, Token { kind: OpenParen, value: "(", line: 29 }, Token { kind: CloseParen, value: ")", line: 29 }, Token { kind: SemiColon, value: ";", line: 29 }, Token { kind: String, value: "<Backspace>", line: 30 }, Token { kind: Colon, value: ":", line: 30 }, Token { kind: Identifier, value: "BackspaceLetters", line: 30 }, Token { kind: OpenParen, value: "(", line: 30 }, Token { kind: Number, value: "1", line: 30 }, Token { kind: CloseParen, value: ")", line: 30 }, Token { kind: SemiColon, value: ";", line: 30 }, Token { kind: String, value: "<Delete>", line: 31 }, Token { kind: Colon, value: ":", line: 31 }, Token { kind: Identifier, value: "DeleteLetters", line: 31 }, Token { kind: OpenParen, value: "(", line: 31 }, Token { kind: Number, value: "1", line: 31 }, Token { kind: CloseParen, value: ")", line: 31 }, Token { kind: SemiColon, value: ";", line: 31 }, Token { kind: String, value: "<Enter>", line: 32 }, Token { kind: Colon, value: ":", line: 32 }, Token { kind: Identifier, value: "InsertLine", line: 32 }, Token { kind: OpenParen, value: "(", line: 32 }, Token { kind: CloseParen, value: ")", line: 32 }, Token { kind: SemiColon, value: ";", line: 32 }, Token { kind: CloseCurly, value: "}", line: 34 }, Token { kind: EndOfFile, value: "Eof", line: 35 }]
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('q') }], actions_after_activations: [Quit], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('c') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, keycode: Char('v') }], actions_after_activations: [ToDo], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }], actions_after_activations: [DeleteCurrentLine], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }, DeleteSelection], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: true }], length: 3 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }], actions_after_activations: [SelectCurrentWords { characters_are_brakes: false }], length: 3 }
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
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('e') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('E') }], actions_after_activations: [GoToEndOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('b') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: true }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('B') }], actions_after_activations: [GoToStartOfWord { characters_are_brakes: false }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }], actions_after_activations: [WriteText("Test")], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }], actions_after_activations: [DebugSelection], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }], actions_after_activations: [NormalMode], length: 2 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Esc }], actions_after_activations: [NormalMode], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Backspace }], actions_after_activations: [BackspaceLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Delete }], actions_after_activations: [DeleteLetters { count: 1 }], length: 1 }
parse_single_shortcut
Parsed shortcut: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, keycode: Enter }], actions_after_activations: [InsertLine], length: 1 }
input_handler::startup
normal_segments:{[Keystroke { ctrl: false, keycode: Char('j') }]: Some(13), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('s') }]: Some(19), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(7), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: false, keycode: Char('E') }]: Some(15), [Keystroke { ctrl: false, keycode: Char('s') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(6), [Keystroke { ctrl: false, keycode: Char('h') }]: Some(11), [Keystroke { ctrl: false, keycode: Char('q') }]: Some(0), [Keystroke { ctrl: false, keycode: Char('s') }]: None, [Keystroke { ctrl: false, keycode: Char('b') }]: Some(16), [Keystroke { ctrl: false, keycode: Char('a') }]: Some(9), [Keystroke { ctrl: false, keycode: Char('k') }]: Some(12), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(8), [Keystroke { ctrl: false, keycode: Char('t') }]: None, [Keystroke { ctrl: false, keycode: Char('l') }]: Some(10), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]: None, [Keystroke { ctrl: false, keycode: Char('e') }]: Some(14), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('W') }]: Some(5), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(3), [Keystroke { ctrl: true, keycode: Char('c') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('B') }]: Some(17), [Keystroke { ctrl: false, keycode: Char('t') }, Keystroke { ctrl: false, keycode: Char('w') }]: Some(18)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssaddfsaf
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssaddfsa
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssaddfs
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssaddf
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssadd
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssad
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ssa
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output ss
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output s
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('O') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output   OO
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output   O
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output   
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output  
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output 
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output daw
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,0 self.to_x,3 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output 
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIIIIII
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output     IIIIIII
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,11 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output     
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('D') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('D') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('F') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('f') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('n') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('-') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('-') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('-') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDDFfnn--
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDDFfnn-
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDDFfnn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDDFfn
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDDFf
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDDF
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IDD
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     ID
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output     )*/*//))/*)/*)/*
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,4 self.to_x,20 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output     
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('\'') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('J') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('L') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('S') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('A') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('K') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('L') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('F') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJKLSAKL
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJKLSAK
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJKLSA
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJKLS
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJKL
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJK
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     IIJ
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('`') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('`') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('`') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('`') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('`') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('I') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output     II*`/*`/`/`/`I
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,14 self.to_x,15 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output     II*`/*`/`//`I
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('u') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/*`/`//I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/*`/`/I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/*`/`I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/*`/I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/*`I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/*I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`/I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*`I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
remove_text: line 1 
remove_text: output     II*I
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('(') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('&') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('/') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('*') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char(')') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
remove_text: line 1 
remove_text: output     II*()&/)*//*)I
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('a') }]
select_word: self.from_x,6 self.to_x,17 self.from_y,1 self.to_y1
delete_selection
remove_text: line 1 
remove_text: output     III
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
