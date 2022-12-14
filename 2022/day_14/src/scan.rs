#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Air,
    Rock,
    Sand,
    Generator,
}

pub enum SandState {
    Rest,
    Overflow,
    Next(Coordinate),
}

pub type Coordinate = (usize, usize);
pub type Lines = Vec<Coordinate>;

pub trait Scan {
    fn new(input: &str) -> Self;
    fn get_units_of_sand(&self) -> usize;
    fn draw_lines(&mut self, lines: &Lines);
    fn tick(&mut self) -> bool;
    fn simulate(&mut self);
}
