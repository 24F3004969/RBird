#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsPart {
    pub condition: bool,
    pub x: String,
}

impl IsPart {
    pub fn new(condition: bool, x: impl Into<String>) -> Self {
        Self {
            condition,
            x: x.into(),
        }
    }
}