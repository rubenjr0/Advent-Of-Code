use std::ops::RangeInclusive;

use coord::Coord;
use rayon::prelude::*;
use sensor::Sensor;

mod bounding_box_iter;
mod coord;
mod sensor;
mod t;

fn parse_entry(input: &str) -> Sensor {
    let input = input.replace(':', ",");
    let mut xs = input
        .split("x=")
        .skip(1)
        .map(|p| p.split(',').next().unwrap());
    let mut ys = input
        .split("y=")
        .skip(1)
        .map(|p| p.split(',').next().unwrap());
    let sc = Coord::parse(&mut xs, &mut ys);
    let bc = Coord::parse(&mut xs, &mut ys);
    let sensor = Sensor::new(sc, &bc);
    sensor
}

fn parse_data(input: &str) -> Vec<Sensor> {
    let mut sensors = vec![];
    input.lines().for_each(|l| {
        let sensor = parse_entry(l);
        sensors.push(sensor);
    });
    sensors
}

fn merge_slices(mut slices: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    slices.sort_by_key(|r| *r.start());
    let mut slices = slices.iter();
    let mut merged_slices = Vec::new();
    let first = slices.next().unwrap();
    let mut current_start = first.start();
    let mut current_end = first.end();
    while let Some(next) = slices.next() {
        if current_end > next.end() {
            continue;
        } else if current_end >= &(next.start() - 1) {
            current_end = next.end();
        } else {
            merged_slices.push(*current_start..=*current_end);
            current_start = next.start();
            current_end = next.end();
        }
    }
    merged_slices.push(*current_start..=*current_end);
    merged_slices
}

fn compute_row_coverage(sensors: &Vec<Sensor>, row: isize) -> usize {
    let slices = sensors
        .par_iter()
        .flat_map(|s| s.slice_at_row(row))
        .collect();
    let ranges = merge_slices(slices);
    ranges
        .iter()
        .map(|range| range.end() - range.start())
        .sum::<isize>() as usize
}

fn part_one(sensors: &Vec<Sensor>, row: isize) -> usize {
    compute_row_coverage(&sensors, row)
}

fn part_two(sensors: &Vec<Sensor>, space: isize) -> Option<Coord> {
    sensors.par_iter().find_map_any(|s| {
        s.iter()
            .filter(|p| (0 <= p.x && p.x <= space) && (0 <= p.y && p.y <= space))
            .find(|p| !sensors.iter().any(|s| s.contains(&p)))
    })
}

fn main() {
    let input = include_str!("../input.txt");
    let (sensors, t_parsing) = timeit!(parse_data(input));
    println!("Data parsed in {t_parsing:?}");

    let (solution, t_solution) = timeit!(part_one(&sensors, 2000000));
    assert_eq!(solution, 5073496);
    println!("Part one");
    println!(" - Solution {solution} found in {t_solution:?}\n");

    let (Some(Coord{x,y}), t_solution) = timeit!(part_two(&sensors, 4_000_000)) else {
        panic!("Couldn't find a solution!");
    };
    let solution = 4_000_000 * x + y;
    assert_eq!(solution, 13081194638237);
    println!("Part two");
    eprintln!(" - Distress beacon found at ({x}, {y})");
    eprintln!(" - Solution {solution} found in {t_solution:?}");
}

#[cfg(test)]
mod tests {
    use crate::{coord::Coord, merge_slices, parse_data, part_one, part_two, Sensor};

    #[test]
    fn test_slice() {
        let sensor = dbg!(Sensor::new(Coord { x: 0, y: 0 }, &Coord { x: 1, y: 3 }));
        assert_eq!(sensor.slice_at_row(2), Some(-2..=2));
        assert_eq!(sensor.slice_at_row(-3), Some(-1..=1));
        assert_eq!(sensor.slice_at_row(4), Some(0..=0));
        assert!(sensor.slice_at_row(-10).is_none());
    }

    #[test]
    fn test_borders() {
        let sensor = dbg!(Sensor::new(Coord { x: 0, y: 0 }, &Coord { x: 0, y: 2 }));
        let points: Vec<_> = sensor.iter().collect();
        assert!(points.len() == 13);
        assert!(!points.into_iter().any(|p| sensor.contains(&p)))
    }

    #[test]
    fn test_merge_slices() {
        let slices = vec![12..=12, 2..=14, 2..=2, -2..=2, 16..=24, 14..=18];
        assert_eq!(merge_slices(slices), vec![-2..=24]);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../test.txt");
        let sensors = parse_data(input);
        let solution = part_one(&sensors, 10);
        assert_eq!(solution, 26);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test.txt");
        let sensors = parse_data(input);
        let solution = part_two(&sensors, 20);
        let expected = Coord { x: 14, y: 11 };
        assert_eq!(solution, Some(expected));
    }
}
