use std::collections::VecDeque;
use rand::{Rng, RngExt};
use crate::game::block::Block;
use crate::game::point::Point;

#[derive(Debug)]
pub struct Obstacles {
    height: i32,
    width: i32,
    thickness: i32,
    k: i32,
    difference: i32,
    obstacles_list: VecDeque<Block>,
}

impl Obstacles {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0, "Width must be greater than zero");
        assert!(height > 0, "Height must be greater than zero");

        /*
         * Java:
         *
         * k = width / (int) Math.log10(width * width * width);
         *
         * The multiplication is performed as f64 here to avoid integer
         * overflow before calculating log10.
         */
        let logarithm = ((width as f64).powi(3)).log10() as i32;

        let k = if logarithm == 0 {
            15
        } else {
            width / logarithm
        };

        let mut obstacles = Self {
            height,
            width,
            thickness: 8,
            k,
            difference: 0,
            obstacles_list: VecDeque::new(),
        };

        obstacles.create_obstacles();
        obstacles
    }

    pub fn obstacles_list(&self) -> &VecDeque<Block> {
        &self.obstacles_list
    }

    pub fn obstacles_list_mut(&mut self) -> &mut VecDeque<Block> {
        &mut self.obstacles_list
    }

    fn create_obstacles(&mut self) {
        let start_x = self.width - 10;

        let mut depth =
            self.height / self.get_stupid_constant(3)
                + self.random_range_inclusive(1, self.height / 4);

        self.obstacles_list.push_back(Block::new(
            Point::new(start_x, 0),
            Point::new(start_x + self.thickness, 0),
            Point::new(start_x, depth),
            Point::new(start_x + self.thickness, depth),
        ));

        depth += self.get_stupid_constant(12);

        self.obstacles_list.push_back(Block::new(
            Point::new(start_x, depth),
            Point::new(start_x + self.thickness, depth),
            Point::new(start_x, self.height),
            Point::new(start_x + self.thickness, self.height),
        ));

        for _ in 1..=9 {
            self.difference = self.k;

            let edge2 = self
                .obstacles_list
                .back()
                .expect("Obstacle list should not be empty")
                .edge2();

            depth = self.height / self.get_stupid_constant(3)
                + self.random_range_inclusive(1, self.height / 4);

            println!("{depth}");

            self.have_fun(edge2, depth);
        }
    }

    pub fn add_obstacle(&mut self) {
        let edge2 = self
            .obstacles_list
            .back()
            .expect("Cannot add an obstacle because the list is empty")
            .edge2();

        let depth = self.height / self.get_stupid_constant(3)
            + self.random_range_inclusive(1, self.height / 4);

        self.difference = self.k;

        self.have_fun(edge2, depth);
    }

    fn have_fun(&mut self, edge2: Point, mut depth: i32) {
        let left_x = edge2.x + self.thickness + self.difference;
        let right_x = edge2.x + (2 * self.thickness) + self.difference;

        self.obstacles_list.push_back(Block::new(
            Point::new(left_x, 0),
            Point::new(right_x, 0),
            Point::new(left_x, depth),
            Point::new(right_x, depth),
        ));

        depth += self.get_stupid_constant(12);

        self.obstacles_list.push_back(Block::new(
            Point::new(left_x, depth),
            Point::new(right_x, depth),
            Point::new(left_x, self.height),
            Point::new(right_x, self.height),
        ));
    }

    fn get_stupid_constant(&self, constant: i32) -> i32 {
        constant + self.random_range_inclusive(1, self.height / constant)
    }

    fn random_range_inclusive(&self, minimum: i32, maximum: i32) -> i32 {
        if maximum <= minimum {
            return minimum;
        }

        rand::rng().random_range(minimum..=maximum)
    }
}