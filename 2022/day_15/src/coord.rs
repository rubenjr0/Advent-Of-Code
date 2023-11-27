#[derive(Debug, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn parse<'a>(
        xs: &mut impl Iterator<Item = &'a str>,
        ys: &mut impl Iterator<Item = &'a str>,
    ) -> Coord {
        let x = xs.next().unwrap().parse().unwrap();
        let y = ys.next().unwrap().parse().unwrap();
        Coord { x, y }
    }

    pub fn manhattan_distance(&self, Coord { x, y }: &Coord) -> usize {
        self.x.abs_diff(*x) + self.y.abs_diff(*y)
    }
}
