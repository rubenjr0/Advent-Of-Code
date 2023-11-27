use eyre::{eyre, Result};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(direction: &str) -> Result<Direction> {
        Ok(match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(eyre!("Invalid direction: {direction}")),
        })
    }
}

pub fn parse_movement(input: &str) -> Result<(Direction, i16)> {
    let mut input = input.split_whitespace();
    let direction = input.next().unwrap();
    let direction = Direction::parse(direction)?;
    let distance = input.next().unwrap().parse()?;
    Ok((direction, distance))
}
