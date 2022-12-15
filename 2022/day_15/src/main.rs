use rayon::prelude::*;

mod t;

type Unit = isize;
type UUnit = usize;
type Coord = (Unit, Unit);

#[derive(Debug)]
struct SensorCoverage {
    sensor: Coord,
    beacon: Coord,
    distance: UUnit,
}

#[derive(Debug, PartialEq)]
enum Coverage {
    Sensor,
    Beacon,
    Free,
    Covered,
}

impl SensorCoverage {
    fn new(sensor: Coord, beacon: Coord) -> SensorCoverage {
        SensorCoverage {
            sensor,
            beacon,
            distance: manhattan_distance(&sensor, &beacon),
        }
    }
}

fn manhattan_distance((x1, y1): &Coord, (x2, y2): &Coord) -> UUnit {
    x1.abs_diff(*x2) + y1.abs_diff(*y2)
}

fn parse_entry(input: &str) -> (Coord, Coord) {
    let input = input.replace(':', ",");
    let mut ns = input
        .split('=')
        .skip(1)
        .map(|p| p.split(',').next().unwrap());
    (
        (
            ns.next().unwrap().parse().unwrap(),
            ns.next().unwrap().parse().unwrap(),
        ),
        (
            ns.next().unwrap().parse().unwrap(),
            ns.next().unwrap().parse().unwrap(),
        ),
    )
}

fn is_in_range(pair: &SensorCoverage, target_row: Unit) -> bool {
    target_row.abs_diff(pair.sensor.1) <= pair.distance
}

fn parse_data(input: &str) -> Vec<SensorCoverage> {
    input
        .lines()
        .map(|l| {
            let (sensor, beacon) = parse_entry(l);
            SensorCoverage::new(sensor, beacon)
        })
        .collect()
}

fn compute_row_coverage(data: &Vec<SensorCoverage>, row: Unit) -> (Vec<Coverage>, Unit, Unit) {
    let filtered_data: Vec<&SensorCoverage> = data
        .par_iter()
        .filter(move |p| is_in_range(p, row))
        .collect();
    let max_coverage = filtered_data.iter().map(|p| p.distance).max().unwrap() as Unit;
    let x_min = filtered_data
        .par_iter()
        .map(|p| p.sensor.0.min(p.beacon.0))
        .min()
        .unwrap()
        - max_coverage;
    let x_max = filtered_data
        .par_iter()
        .map(|p| p.sensor.0.max(p.beacon.0))
        .max()
        .unwrap()
        + max_coverage;
    let coverage: Vec<_> = (x_min..=x_max)
        .into_par_iter()
        .map(|x| {
            let mut status = Coverage::Free;
            for s in filtered_data.iter().filter(|s| {
                (s.sensor.0 - s.distance as Unit..=s.sensor.0 + s.distance as Unit).contains(&x)
            }) {
                if s.beacon.1 == row && s.beacon.0 == x {
                    status = Coverage::Beacon
                } else if s.sensor.1 == row && s.sensor.0 == x {
                    status = Coverage::Sensor
                } else if manhattan_distance(&s.sensor, &(x, row)) <= s.distance {
                    status = Coverage::Covered
                }
            }
            status
        })
        .collect();
    (coverage, x_min, x_max)
}

fn part_one(data: &Vec<SensorCoverage>, target_row: Unit) -> usize {
    let (coverage, _, _) = compute_row_coverage(data, target_row);
    coverage.iter().filter(|b| b == &&Coverage::Covered).count()
}

fn part_two(data: &Vec<SensorCoverage>, space: UUnit) -> Option<UUnit> {
    (0..=space).into_par_iter().find_map_any(|y| {
        let (row, x_min, _) = compute_row_coverage(data, y as isize);
        let idx_z = if x_min < 0 { x_min.unsigned_abs() } else { 0 };
        let idx_max = idx_z + space;
        row[idx_z..idx_max]
            .par_iter()
            .enumerate()
            .find_any(|(_, b)| b == &&Coverage::Free)
            .and_then(|(x, _)| Some(4_000_000 * x + y))
    })
}

fn main() {
    let input = include_str!("../input.txt");
    let data = timeit!(parse_data(input), "parse_data");
    let solution = timeit!(part_one(&data, 2000000), "part one");
    assert_eq!(solution, 5073496);
    println!("Part one: {solution}");

    if let Some(tf) = timeit!(part_two(&data, 4_000_000), "part_two") {
        println!("{tf}");
    } else {
        eprintln!("fuck");
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_data, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = include_str!("../test.txt");
        let data = parse_data(input);
        let solution = part_one(&data, 10);
        assert_eq!(solution, 26);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test.txt");
        let data = parse_data(input);
        let solution = part_two(&data, 20);
        assert!(solution.is_some());
        assert_eq!(solution.unwrap(), 56000011);
    }
}
