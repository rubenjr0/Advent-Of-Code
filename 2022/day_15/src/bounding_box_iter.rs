use crate::coord::Coord;

pub enum BoundingBoxIterationStatus {
    Started,
    BottomRight,
    BottomLeft,
    TopLeft,
    TopRight,
    Done,
}

pub struct BoundingBoxIterator<'a> {
    center: &'a Coord,
    position: Coord,
    status: BoundingBoxIterationStatus,
}

impl<'a> BoundingBoxIterator<'a> {
    pub fn new(center: &'a Coord, position: Coord) -> Self {
        Self {
            center,
            position,
            status: BoundingBoxIterationStatus::Started,
        }
    }
}

impl Iterator for BoundingBoxIterator<'_> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        match self.status {
            BoundingBoxIterationStatus::Started => {
                self.status = BoundingBoxIterationStatus::BottomRight
            }
            BoundingBoxIterationStatus::BottomRight => {
                self.position.x += 1;
                self.position.y -= 1;
                if self.position.y == self.center.y {
                    self.status = BoundingBoxIterationStatus::BottomLeft
                }
            }
            BoundingBoxIterationStatus::BottomLeft => {
                self.position.x -= 1;
                self.position.y -= 1;
                if self.position.x == self.center.x {
                    self.status = BoundingBoxIterationStatus::TopLeft
                }
            }
            BoundingBoxIterationStatus::TopLeft => {
                self.position.x -= 1;
                self.position.y += 1;
                if self.position.y == self.center.y {
                    self.status = BoundingBoxIterationStatus::TopRight
                }
            }
            BoundingBoxIterationStatus::TopRight => {
                self.position.x += 1;
                self.position.y += 1;
                if self.position.x == self.center.x {
                    self.status = BoundingBoxIterationStatus::Done
                }
            }
            BoundingBoxIterationStatus::Done => return None,
        }
        Some(Coord {
            x: self.position.x,
            y: self.position.y,
        })
    }
}
