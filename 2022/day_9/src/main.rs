use std::time::Instant;

use direction::parse_movement;
use rope::Rope;

mod direction;
mod point;
mod rope;

fn main() {
    let input = include_str!("../input.txt");
    let mut rope = Rope::new(2);
    let part_one_time = Instant::now();
    input
        .lines()
        .flat_map(|line| parse_movement(line))
        .for_each(|(direction, amount)| rope.apply_movement(&direction, amount));
    let part_one_time = part_one_time.elapsed();
    println!(
        "Part one: {} unique points visited in {part_one_time:?}",
        rope.visited().len()
    );

    let mut rope = Rope::new(10);
    let part_two_time = Instant::now();
    input
        .lines()
        .flat_map(|line| parse_movement(line))
        .for_each(|(direction, amount)| rope.apply_movement(&direction, amount));
    let part_two_time = part_two_time.elapsed();
    println!(
        "Part two: {} unique points visited in {part_two_time:?}",
        rope.visited().len()
    );
}

#[cfg(test)]
mod tests {
    use crate::{direction::parse_movement, rope::Rope};

    #[test]
    fn part_one() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let mut rope = Rope::new(2);

        input
            .lines()
            .flat_map(|line| parse_movement(line))
            .for_each(|(direction, amount)| {
                rope.apply_movement(&direction, amount);
            });

        assert_eq!(rope.visited().len(), 13);
    }

    #[test]
    fn part_two() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

        let mut rope = Rope::new(10);

        input
            .lines()
            .flat_map(|line| parse_movement(line))
            .for_each(|(direction, amount)| {
                rope.apply_movement(&direction, amount);
            });
        for p in rope.visited() {
            println!("{p:?}");
        }
        assert_eq!(rope.visited().len(), 36);
    }
}
