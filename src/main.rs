mod game;
use console::{Key, Term};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
 game::run();
}
