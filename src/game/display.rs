use console::Term;

pub struct Display {
    pub terminal: Term,
}

impl Display {
    pub fn new() -> Self {
        Self {
            terminal: Term::stdout(),
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}