fn main() {
    let input: Vec<&str> = include_str!("../input.txt").split("\n").collect();
    let p1: u16 = input
        .iter()
        .filter_map(|rucksack| {
            if let Some(common) = find_common_in_rucksack(rucksack) {
                Some(priority(common) as u16)
            } else {
                None
            }
        })
        .sum();
    println!("Part one: {p1}");

    let p2: u16 = input
        .chunks(3)
        .filter_map(|a| {
            if let Some(common) = find_common_in_group(a.to_vec()) {
                Some(priority(common) as u16)
            } else {
                None
            }
        })
        .sum();
    println!("Part two: {p2}");
}

fn find_common_in_rucksack(rucksack: &str) -> Option<char> {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let left: Vec<char> = left.chars().collect();
    let mut right = right.chars();
    right.find(|c| left.contains(c))
}

fn find_common_in_group(group: Vec<&str>) -> Option<char> {
    let group: Vec<Vec<char>> = group.iter().map(|g| g.chars().collect()).collect();
    let mut commons_in_2 = group[0].iter().filter(|c| group[1].contains(c));
    commons_in_2.find(|c| group[2].contains(c)).copied()
}

fn priority(item: char) -> u8 {
    item as u8 - if item.is_lowercase() { 96 } else { 38 }
}
