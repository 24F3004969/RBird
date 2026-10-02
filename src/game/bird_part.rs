use crate::game::point::Point;

#[derive(Debug, Clone)]
pub struct BirdParts {
    pub text: String,
    pub point: Point,
}

impl BirdParts {
    pub fn new(text: String, point: Point) -> Self {
        Self { text, point }
    }
}