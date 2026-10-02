use crate::game::bird_part::BirdParts;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub s: String,
    pub list: Vec<BirdParts>,
}

impl Tuple {
    pub fn new(s: impl Into<String>, list: Vec<BirdParts>) -> Self {
        Self {
            s: s.into(),
            list,
        }
    }
}