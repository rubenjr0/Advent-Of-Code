use std::ops::RangeInclusive;

use crate::{bounding_box_iter::BoundingBoxIterator, coord::Coord};

#[derive(Debug)]
pub struct Sensor {
    pub position: Coord,
    distance: usize,
}

impl Sensor {
    pub fn new(position: Coord, bc: &Coord) -> Self {
        let distance = position.manhattan_distance(bc);
        Self { position, distance }
    }

    pub fn contains(&self, p: &Coord) -> bool {
        self.position.manhattan_distance(p) <= self.distance
    }

    pub fn slice_at_row(&self, row: isize) -> Option<RangeInclusive<isize>> {
        if self.position.y + (self.distance as isize) < row
            || row < self.position.y - self.distance as isize
        {
            None
        } else {
            let h = self.position.y.abs_diff(row).abs_diff(self.distance);
            Some(self.position.x - h as isize..=self.position.x + h as isize)
        }
    }

    pub fn iter(&self) -> BoundingBoxIterator {
        BoundingBoxIterator::new(
            &self.position,
            Coord {
                x: self.position.x,
                y: self.position.y + self.distance as isize + 1,
            },
        )
    }
}
