fn parse_range(range: &str) -> (u8, u8) {
    let mut split = range.split("-").flat_map(|n| n.parse());
    let left = split.next().unwrap();
    let right = split.next().unwrap();
    (left, right)
}

fn parse_line(line: &str) -> Option<((u8, u8), (u8, u8))> {
    if line.len() > 1 {
        let mut split = line.split(",").map(|range| parse_range(range));
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        Some((left, right))
    } else {
        None
    }
}

fn are_contained((left_1, right_1): &(u8, u8), (left_2, right_2): &(u8, u8)) -> bool {
    (left_1 <= left_2 && right_1 >= right_2) || (left_2 <= left_1 && right_2 >= right_1)
}

fn do_overlap((left_1, right_1): &(u8, u8), (left_2, right_2): &(u8, u8)) -> bool {
    (left_1 <= left_2 && right_1 >= left_2) || (left_2 <= left_1 && right_2 >= left_1)
}

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<_> = input
        .split("\n")
        .flat_map(|line| parse_line(line))
        .collect();

    let part_one = data
        .iter()
        .filter(|(left, right)| are_contained(left, right))
        .count();

    let part_two = data
        .iter()
        .filter(|(left, right)| do_overlap(left, right))
        .count();

    println!("Part one: {part_one}");
    println!("Part two: {part_two}");
}

#[cfg(test)]
mod tests {
    use super::do_overlap;

    #[test]
    fn no_overlap() {
        let r1 = (2, 3);
        let r2 = (4, 5);
        assert!(!do_overlap(&r1, &r2));
        assert!(!do_overlap(&r2, &r1));
    }

    #[test]
    fn overlap() {
        let r1 = (2, 6);
        let r2 = (4, 8);
        assert!(do_overlap(&r1, &r2));
        assert!(do_overlap(&r2, &r1));
    }
}
