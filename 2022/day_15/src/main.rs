use std::{collections::HashSet, ops::RangeInclusive};

use coord::Coord;
use rayon::prelude::*;
use sensor::Sensor;

mod bounding_box_iter;
mod coord;
mod sensor;
mod t;

fn parse_entry(input: &str) -> (Sensor, Coord) {
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
    (sensor, bc)
}

fn parse_data(input: &str) -> (Vec<Sensor>, Vec<Coord>) {
    let mut sensors = vec![];
    let mut beacons = vec![];
    input.lines().for_each(|l| {
        let (sensor, beacon) = parse_entry(l);
        sensors.push(sensor);
        beacons.push(beacon);
    });
    (sensors, beacons)
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

fn compute_row_coverage(sensors: &Vec<Sensor>, beacons: &Vec<Coord>, row: isize) -> usize {
    let slices: Vec<_> = sensors
        .par_iter()
        .flat_map(|s| s.slice_at_row(row))
        .collect();
    let slices = merge_slices(slices);
    let obstructed: HashSet<_> = sensors
        .par_iter()
        .filter(|s| s.position.y == row)
        .map(|s| s.position.x)
        .chain(
            beacons
                .par_iter()
                .filter(|Coord { x: _, y }| y == &row)
                .map(|Coord { x, y: _ }| *x),
        )
        .collect();
    slices
        .into_par_iter()
        .map(|range| {
            range
                .into_par_iter()
                .map(|x| if obstructed.contains(&x) { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum()
}

fn part_two(sensors: &Vec<Sensor>, space: isize) -> Option<Coord> {
    for sensor in sensors {
        let mut iter = sensor.iter();
        while let Some(p) = iter.next() {
            if p.x < 0 || p.y < 0 || p.x > space || p.y > space {
                continue;
            }
            if !sensors.iter().any(|s| s.contains(&p)) {
                return Some(p);
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("../input.txt");
    let ((sensors, beacons), t_parsing) = timeit!(parse_data(input));
    println!("Data parsed in {t_parsing:?}");

    let (solution, t_solution) = timeit!(compute_row_coverage(&sensors, &beacons, 2000000));
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
    use crate::{compute_row_coverage, coord::Coord, merge_slices, parse_data, part_two, Sensor};

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
        let (sensors, beacons) = parse_data(input);
        let solution = compute_row_coverage(&sensors, &beacons, 10);
        assert_eq!(solution, 26);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test.txt");
        let (sensors, _) = parse_data(input);
        let solution = part_two(&sensors, 20);
        let expected = Coord { x: 14, y: 11 };
        assert_eq!(solution, Some(expected));
    }
}
