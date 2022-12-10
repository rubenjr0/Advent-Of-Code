use std::fmt::Display;

use crate::direction::Direction;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn get_position(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    pub fn distance(&self, other: &Point) -> (i16, i16) {
        (
            (self.x as i16 - other.x as i16),
            (self.y as i16 - other.y as i16),
        )
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
