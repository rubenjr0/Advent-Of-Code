#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn top(&self) -> Self {
        Point::new(self.x, self.y - 1)
    }

    pub fn right(&self) -> Self {
        Point::new(self.x + 1, self.y)
    }

    pub fn bottom(&self) -> Self {
        Point::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Self {
        Point::new(self.x - 1, self.y)
    }
}
