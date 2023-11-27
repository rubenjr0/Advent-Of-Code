use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Direction {
    Forward,
    Up,
    Down,
}

fn str_to_direction(dir: &str) -> Direction {
    match dir {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("{} is not a valid direction!", dir),
    }
}

fn run_course(course: &Vec<(Direction, i32)>) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    for (direction, quantity) in course {
        match direction {
            Direction::Forward => position += quantity,
            Direction::Up => depth -= quantity,
            Direction::Down => depth += quantity,
        }
    }
    position * depth
}

fn run_course_with_aim(course: &Vec<(Direction, i32)>) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (direction, quantity) in course {
        match direction {
            Direction::Forward => {
                position += quantity;
                depth += quantity * aim
            }
            Direction::Up => aim -= quantity,
            Direction::Down => aim += quantity,
        }
    }
    position * depth
}

fn main() {
    let path = Path::new("input");

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open {}: {}", path.display(), reason),
    };

    let reader = BufReader::new(file);
    let mut course: Vec<(Direction, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let direction = str_to_direction(&direction);
        let quantity = split.next().unwrap();
        let quantity = i32::from_str_radix(quantity, 10).unwrap();
        course.push((direction, quantity));
    }

    println!("Part One: {}", run_course(&course));
    println!("Part Two: {}", run_course_with_aim(&course));
}
