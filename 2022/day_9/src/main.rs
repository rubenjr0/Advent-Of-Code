use movement::parse_movement;
use rope::Rope;

mod movement;
mod point;
mod rope;

fn main() {
    let input = include_str!("../input.txt");
    let mut rope = Rope::new();
    input
        .lines()
        .flat_map(|line| parse_movement(line))
        .for_each(|(direction, amount)| rope.apply_movement(&direction, amount));
    println!("Part one: {}", rope.unique_visited());
}

#[cfg(test)]
mod tests {
    use crate::{movement::parse_movement, point::Point, rope::Rope};

    #[test]
    fn part_one() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let mut rope = Rope::new();

        input
            .lines()
            .flat_map(|line| parse_movement(line))
            .for_each(|(direction, amount)| rope.apply_movement(&direction, amount));

        assert_eq!(rope.unique_visited(), 13);
    }
}
