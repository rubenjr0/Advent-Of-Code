use std::{collections::HashSet, ops::RangeInclusive};

use rayon::prelude::*;

mod t;

type Coord = (isize, isize);

enum BoundingBoxIterationStatus {
    Started,
    BottomRight,
    BottomLeft,
    TopLeft,
    TopRight,
    Done,
}

struct BoundingBoxIterator {
    cx: isize,
    cy: isize,
    x: isize,
    y: isize,
    status: BoundingBoxIterationStatus,
}

impl Iterator for BoundingBoxIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        match self.status {
            BoundingBoxIterationStatus::Started => {
                self.status = BoundingBoxIterationStatus::BottomRight
            }
            BoundingBoxIterationStatus::BottomRight => {
                self.x += 1;
                self.y -= 1;
                if self.y == self.cy {
                    self.status = BoundingBoxIterationStatus::BottomLeft
                }
            }
            BoundingBoxIterationStatus::BottomLeft => {
                self.x -= 1;
                self.y -= 1;
                if self.x == self.cx {
                    self.status = BoundingBoxIterationStatus::TopLeft
                }
            }
            BoundingBoxIterationStatus::TopLeft => {
                self.x -= 1;
                self.y += 1;
                if self.y == self.cy {
                    self.status = BoundingBoxIterationStatus::TopRight
                }
            }
            BoundingBoxIterationStatus::TopRight => {
                self.x += 1;
                self.y += 1;
                if self.x == self.cx {
                    self.status = BoundingBoxIterationStatus::Done
                }
            }
            BoundingBoxIterationStatus::Done => return None,
        }
        Some((self.x, self.y))
    }
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    distance: usize,
}

impl Sensor {
    fn new(sc: Coord, bc: Coord) -> Self {
        let distance = manhattan_distance(sc, bc);
        Self {
            x: sc.0,
            y: sc.1,
            distance,
        }
    }

    fn contains(&self, p: Coord) -> bool {
        manhattan_distance((self.x, self.y), p) <= self.distance
    }

    fn slice_at_row(&self, row: isize) -> Option<RangeInclusive<isize>> {
        if self.y + (self.distance as isize) < row || row < self.y - self.distance as isize {
            None
        } else {
            let h = self.y.abs_diff(row).abs_diff(self.distance);
            Some(self.x - h as isize..=self.x + h as isize)
        }
    }

    fn iter(&self) -> BoundingBoxIterator {
        BoundingBoxIterator {
            cx: self.x,
            cy: self.y,
            x: self.x,
            y: self.y + self.distance as isize + 1,
            status: BoundingBoxIterationStatus::Started,
        }
    }
}

fn manhattan_distance((x1, y1): Coord, (x2, y2): Coord) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

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
    let sc = (
        xs.next().unwrap().parse().unwrap(),
        ys.next().unwrap().parse().unwrap(),
    );
    let bc = (
        xs.next().unwrap().parse().unwrap(),
        ys.next().unwrap().parse().unwrap(),
    );
    let sensor = Sensor::new(sc, bc);
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
        .filter(|s| s.y == row)
        .map(|s| s.x)
        .chain(
            beacons
                .par_iter()
                .filter(|(_, y)| y == &row)
                .map(|(x, _)| *x),
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
        for p in sensor.iter() {
            if p.0 < 0 || p.1 < 0 || p.0 > space || p.1 > space {
                continue;
            }
            if !sensors.iter().any(|s| s.contains(p)) {
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

    let (Some((x, y)), t_solution) = timeit!(part_two(&sensors, 4_000_000)) else {
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
    use crate::{compute_row_coverage, merge_slices, parse_data, part_two, Sensor};

    #[test]
    fn test_slice() {
        let sensor = dbg!(Sensor::new((0, 0), (1, 3)));
        assert_eq!(sensor.slice_at_row(2), Some(-2..=2));
        assert_eq!(sensor.slice_at_row(-3), Some(-1..=1));
        assert_eq!(sensor.slice_at_row(4), Some(0..=0));
        assert!(sensor.slice_at_row(-10).is_none());
    }

    #[test]
    fn test_borders() {
        let sensor = dbg!(Sensor::new((0, 0), (0, 2)));
        let points: Vec<_> = sensor.iter().collect();
        assert!(points.len() == 13);
        assert!(!points.into_iter().any(|p| sensor.contains(p)))
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
        assert_eq!(solution, Some((14, 11)));
    }
}
