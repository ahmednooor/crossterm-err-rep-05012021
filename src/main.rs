use crossterm;
use std::io::{stdout, Write};

fn main() {
    let case_strs = [
        "Press Ctrl+[ : ",
        "Press Ctrl+] : ",
        "Press Ctrl+/ : ",
        "Press Ctrl+\\ : ",
    ];
    for case_str in &case_strs {
        print!("{}", case_str);
        stdout().flush().unwrap();
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => {
                print!("{:?}\n", key_event);
            },
            _ => {
                print!("Not read as key event.\n")
            },
        }
    }
}

// OUTPUT
// Press Ctrl+[ : KeyEvent { code: Esc, modifiers: NONE }
// Press Ctrl+] : KeyEvent { code: Char('5'), modifiers: CONTROL }
// Press Ctrl+/ : KeyEvent { code: Char('7'), modifiers: SHIFT | CONTROL }
// Press Ctrl+\ : KeyEvent { code: Char('4'), modifiers: CONTROL }
