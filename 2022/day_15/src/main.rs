type Coord = (u8, u8);

fn main() {
    println!("Hello, world!");
}

fn manhattan_distance((x1, y1): Coord, (x2, y2): Coord) -> u8 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn parse_entry(input: &str) -> (Coord, Coord) {
    input.split('=').for_each(|p| {
        println!("{p}");
    });
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parse_entry;

    #[test]
    fn part_one() {
        let input = include_str!("../test.txt");
        input.lines().take(1).for_each(|l| {
            parse_entry(l);
        });
        assert!(false);
    }
}
