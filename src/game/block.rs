use crate::game::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub edge1: Point,
    pub edge2: Point,
    pub edge3: Point,
    pub edge4: Point,
}

impl Block {
    // Constructor matching the auto-generated Java record constructor
    pub fn new(edge1: Point, edge2: Point, edge3: Point, edge4: Point) -> Self {
        Self {
            edge1,
            edge2,
            edge3,
            edge4,
        }
    }

    // Java records use the field name as the getter method (e.g., block.edge2())
    pub fn edge2(&self) -> Point {
        self.edge2
    }
}
