use std::collections::BTreeSet;

use crate::{movement::Direction, point::Point};

#[derive(Debug)]
pub struct Rope {
    head: Point,
    tail: Point,
    visited: BTreeSet<(i16, i16)>,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Point::new(),
            tail: Point::new(),
            visited: BTreeSet::new(),
        }
    }

    pub fn apply_movement(&mut self, direction: &Direction, amount: i16) {
        self.visited.insert((self.tail.x, self.tail.y));
        for _ in 0..amount {
            self.head.move_in_direction(&direction);
            self.update_tail(direction);
            self.visited.insert((self.tail.x, self.tail.y));
        }
    }

    fn update_tail(&mut self, direction: &Direction) {
        let (dx, dy) = self.tail.distance(&self.head);
        if dy == 0 && dx.abs() == 2 {
            self.tail.move_in_direction(direction);
        } else if dx == 0 && dy.abs() == 2 {
            self.tail.move_in_direction(direction)
        } else if dx.abs() + dy.abs() > 2 {
            if dx < 0 {
                self.tail.move_in_direction(&Direction::Right);
            } else {
                self.tail.move_in_direction(&Direction::Left);
            }
            if dy < 0 {
                self.tail.move_in_direction(&Direction::Up);
            } else {
                self.tail.move_in_direction(&Direction::Down);
            }
        }
    }

    pub fn unique_visited(&self) -> usize {
        self.visited.len()
    }
}
