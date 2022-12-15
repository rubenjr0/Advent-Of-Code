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

fn parse_data(input: &str) -> impl Iterator<Item = SensorCoverage> + '_ {
    input.lines().map(|l| {
        let (sensor, beacon) = parse_entry(l);
        SensorCoverage::new(sensor, beacon)
    })
}

fn get_on_range(
    data: impl Iterator<Item = SensorCoverage>,
    target_row: Unit,
) -> Vec<SensorCoverage> {
    data.filter(move |p| is_in_range(p, target_row)).collect()
}

fn count_hidden_on_row(data: impl Iterator<Item = SensorCoverage>, target_row: Unit) -> usize {
    let filtered_data = timeit!(get_on_range(data, target_row), "get_on_range");
    let max_coverage = filtered_data.iter().map(|p| p.distance).max().unwrap() as Unit;
    let x_min = filtered_data
        .iter()
        .map(|p| p.sensor.0.min(p.beacon.0))
        .min()
        .unwrap()
        - max_coverage;
    let x_max = filtered_data
        .iter()
        .map(|p| p.sensor.0.max(p.beacon.0))
        .max()
        .unwrap()
        + max_coverage;
    let mut covered = 0;
    for x in x_min..=x_max {
        if filtered_data
            .iter()
            .filter(|s| {
                (s.sensor.0 - s.distance as Unit..=s.sensor.0 + s.distance as Unit).contains(&x)
            })
            .any(|s| {
                !(s.beacon.1 == target_row && s.beacon.0 == x)
                    && !(s.sensor.1 == target_row && s.sensor.0 == x)
                    && manhattan_distance(&s.sensor, &(x, target_row)) <= s.distance
            })
        {
            covered += 1;
        }
    }
    covered
}

fn part_one(input: &str, target_row: Unit) -> usize {
    let data = timeit!(parse_data(input), "parse_data");
    timeit!(count_hidden_on_row(data, target_row), "count_hidden_on_row")
}

fn main() {
    let input = include_str!("../input.txt");
    let solution = timeit!(part_one(input, 2000000), "part one");
    assert_eq!(solution, 5073496);
    println!("Part one: {solution}");
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    #[test]
    fn test_part_one() {
        let input = include_str!("../test.txt");
        let solution = part_one(input, 10);
        assert_eq!(solution, 26);
    }
}
