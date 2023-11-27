use std::collections::HashSet;

use crate::{direction::Direction, point::Point};

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Point>,
    visited: HashSet<(i16, i16)>,
}

impl Rope {
    pub fn new(knots: usize) -> Rope {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Rope {
            knots: (0..knots).map(|_| Point::new()).collect(),
            visited,
        }
    }

    pub fn apply_movement(&mut self, direction: &Direction, amount: i16) {
        for _ in 0..amount {
            self.update(direction);
            self.visited
                .insert(self.knots.last().unwrap().get_position());
        }
    }

    fn update(&mut self, direction: &Direction) {
        self.knots[0].move_in_direction(direction);
        for idx in 1..self.knots.len() {
            let (dx, dy) = self
                .knots
                .get(idx)
                .unwrap()
                .distance(self.knots.get(idx - 1).unwrap());
            let point = self.knots.get_mut(idx).unwrap();
            if dy == 0 {
                if dx < -1 {
                    point.move_in_direction(&Direction::Right);
                } else if dx > 1 {
                    point.move_in_direction(&Direction::Left);
                }
            } else if dx == 0 {
                if dy < -1 {
                    point.move_in_direction(&Direction::Up);
                } else if dy > 1 {
                    point.move_in_direction(&Direction::Down);
                }
            } else if dx.abs() + dy.abs() > 2 {
                if dx < 0 {
                    point.move_in_direction(&Direction::Right);
                } else {
                    point.move_in_direction(&Direction::Left);
                }
                if dy < 0 {
                    point.move_in_direction(&Direction::Up);
                } else {
                    point.move_in_direction(&Direction::Down);
                }
            }
        }
    }

    pub fn visited(&self) -> &HashSet<(i16, i16)> {
        &self.visited
    }
}
