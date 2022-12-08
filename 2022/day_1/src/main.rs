fn main() {
    let mut elves: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|elve| elve.split("\n").flat_map(|food| food.parse::<u32>()).sum())
        .collect();

    let part_one_solution = *elves.iter().max().unwrap();
    elves.sort();
    let part_two_solution: u32 = elves.iter().rev().take(3).sum();

    println!("The elf carrying the most calories is carrying {part_one_solution}");
    println!("The top 3 elves are carrying {part_two_solution}");
}
