use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use console::{Key, Term};
use crate::game;

pub fn keyboard() {
    let (tx, rx) = mpsc::channel::<Key>();
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
        let mut last_received_key = None;
        while let Ok(key) = rx.try_recv() {
            last_received_key = Some(key);
        }
        if let Some(key) = last_received_key {
            direction = key;
            if direction == Key::Escape {
                break;
            }
        }
        print!("\u{001b}[H");
        println!("Use WASD or Arrow Keys. Press Esc to quit.");
        println!("Player position: ");
        println!("My direction is = {:?}", direction);
        thread::sleep(Duration::from_millis(100));
        direction = Key::Unknown;
    }
}