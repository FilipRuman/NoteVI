
normal_segments:{[Keystroke { ctrl: false, char: Char('y') }]: None, []: None, [Keystroke { ctrl: false, char: Char('d') }]: None}


# ------------    init    ------------


new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Esc } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('o') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('o') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[Keystroke { ctrl: false, char: Char('d') }]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
new action


# ------------    init    ------------


## test shortcut parse 1.: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }], actions_after_activations: [ToDo], length: 2 }
## test shortcut parse 1.: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, char: Char('i') }], actions_after_activations: [InsertMode], length: 1 }
normal_segments:{[Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('d') }]: None, []: None}
new action


# ------------    init    ------------


## test shortcut parse 1.: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: false, char: Char('I') }, Keystroke { ctrl: false, char: Char('o') }, Keystroke { ctrl: false, char: Char('W') }, Keystroke { ctrl: false, char: Char('o') }, Keystroke { ctrl: true, char: Char('D') }, Keystroke { ctrl: false, char: Char('D') }], actions_after_activations: [ToDo], length: 6 }
## test shortcut parse 2.: Shortcut { keystrokes_to_activate: [Keystroke { ctrl: true, char: Char('I') }, Keystroke { ctrl: true, char: Char('D') }, Keystroke { ctrl: false, char: Char('D') }], actions_after_activations: [InsertMode], length: 3 }
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, []: None}
new action


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[], [Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[], [Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[], [Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[]: None, [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None}
new action


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('a') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: true, char: Char('c') }]: None, [Keystroke { ctrl: false, char: Char('i') }]: None, [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: true, char: Char('v') }]: None, [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('q') }]: None}
new action


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3)}
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('s') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('f') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('e') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('w') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('r') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('e') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('w') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('r') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('2') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('3') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('u') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('4') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('9') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('0') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('2') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('3') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('8') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('4') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('9') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('0') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('2') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('3') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('8') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('r') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('w') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('e') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('f') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('s') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('f') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
new action
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
new action
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: true, char: Char('v') }]: Some(1)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('r') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('u') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('g') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('h') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('a') }]: Some(5)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: true, char: Char('v') }]: Some(1)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('d') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('k') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('l') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: false, char: Char('y') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: true, char: Char('v') }]: Some(1)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('i') }]: Some(4)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('1') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('2') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('3') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('4') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('5') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('6') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('7') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('8') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('9') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('0') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('y') }]: None}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('c') }] ||| segments: [[Keystroke { ctrl: true, char: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, char: Char('v') }] ||| segments: [[Keystroke { ctrl: true, char: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }] ||| segments: [[Keystroke { ctrl: false, char: Char('d') }], [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }] ||| segments: [[Keystroke { ctrl: false, char: Char('y') }], [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('i') }] ||| segments: [[Keystroke { ctrl: false, char: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('a') }] ||| segments: [[Keystroke { ctrl: false, char: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('q') }] ||| segments: [[Keystroke { ctrl: false, char: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }] ||| segments: [[Keystroke { ctrl: false, char: Char('j') }], [Keystroke { ctrl: false, char: Char('j') }, Keystroke { ctrl: false, char: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: true, char: Char('v') }]: Some(1), [Keystroke { ctrl: true, char: Char('c') }]: Some(0), [Keystroke { ctrl: false, char: Char('y') }, Keystroke { ctrl: false, char: Char('y') }]: Some(3), [Keystroke { ctrl: false, char: Char('a') }]: Some(5), [Keystroke { ctrl: false, char: Char('i') }]: Some(4), [Keystroke { ctrl: false, char: Char('q') }]: Some(6), [Keystroke { ctrl: false, char: Char('d') }]: None, [Keystroke { ctrl: false, char: Char('y') }]: None, [Keystroke { ctrl: false, char: Char('d') }, Keystroke { ctrl: false, char: Char('d') }]: Some(2)}

 # start end 

handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char(' ') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Esc } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Esc } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Esc } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, char: Char('j') } last keystrokes:[Keystroke { ctrl: false, char: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, char: Char('q') } last keystrokes:[]


# ------------    init    ------------


get_segments: keystrokes: [Keystroke { ctrl: true, keycode: Char('c') }] ||| segments: [[Keystroke { ctrl: true, keycode: Char('c') }]] 
get_segments: keystrokes: [Keystroke { ctrl: true, keycode: Char('v') }] ||| segments: [[Keystroke { ctrl: true, keycode: Char('v') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }] ||| segments: [[Keystroke { ctrl: false, keycode: Char('d') }], [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, keycode: Char('y') }, Keystroke { ctrl: false, keycode: Char('y') }] ||| segments: [[Keystroke { ctrl: false, keycode: Char('y') }], [Keystroke { ctrl: false, keycode: Char('y') }, Keystroke { ctrl: false, keycode: Char('y') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, keycode: Char('i') }] ||| segments: [[Keystroke { ctrl: false, keycode: Char('i') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, keycode: Char('a') }] ||| segments: [[Keystroke { ctrl: false, keycode: Char('a') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, keycode: Char('q') }] ||| segments: [[Keystroke { ctrl: false, keycode: Char('q') }]] 
get_segments: keystrokes: [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }] ||| segments: [[Keystroke { ctrl: false, keycode: Char('j') }], [Keystroke { ctrl: false, keycode: Char('j') }, Keystroke { ctrl: false, keycode: Char('j') }]] 
normal_segments:{[Keystroke { ctrl: false, keycode: Char('a') }]: Some(5), [Keystroke { ctrl: false, keycode: Char('y') }, Keystroke { ctrl: false, keycode: Char('y') }]: Some(3), [Keystroke { ctrl: false, keycode: Char('d') }, Keystroke { ctrl: false, keycode: Char('d') }]: Some(2), [Keystroke { ctrl: false, keycode: Char('d') }]: None, [Keystroke { ctrl: true, keycode: Char('c') }]: Some(0), [Keystroke { ctrl: true, keycode: Char('v') }]: Some(1), [Keystroke { ctrl: false, keycode: Char('i') }]: Some(4), [Keystroke { ctrl: false, keycode: Char('y') }]: None, [Keystroke { ctrl: false, keycode: Char('q') }]: Some(6)}

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
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Backspace } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('w') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('p') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('2') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('3') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('o') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('e') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('d') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('s') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('i') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('a') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Esc } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[]
handle_new_keystroke Insertmode keystroke:Keystroke { ctrl: false, keycode: Char('j') } last keystrokes:[Keystroke { ctrl: false, keycode: Char('j') }]
handle_new_keystroke Normalmode keystroke:Keystroke { ctrl: false, keycode: Char('q') } last keystrokes:[]


# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 



# ------------    init    ------------



 # start end 

