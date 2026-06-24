mod game;
use console::{Key, Term};
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<Key>();
    game::run();
    print!("\x1B[2J\x1B[H"); // ANSI clear + home cursor
    thread::spawn(move || {
        let term = Term::buffered_stdout();
        loop {
            let key = term.read_key().unwrap();
            tx.send(key).unwrap();
        }
    });
    let mut direction: Key = Key::CtrlC;
    loop {
        let key = rx.try_recv();
        if !key.is_err() {
            direction = key.unwrap();
            if direction == Key::Escape {
                break;
            }
        }
        print!("\u{001b}[H"); // ANSI clear + home cursor
        println!("Use WASD or Arrow Keys. Press Esc to quit.");
        println!("Player position: ");
        println!("My direction is = {:?}", direction);
    }
}
