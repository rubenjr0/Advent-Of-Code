mod computer;
mod filesystem;
mod node;

use std::time::Instant;

use computer::Computer;

fn main() {
    let input = include_str!("../input.txt");

    let mut computer = Computer::new();
    let computer_creation_time = Instant::now();
    input
        .lines()
        .filter(|l| l != &"$ ls")
        .map(|line| line.replace("$ ", ""))
        .for_each(|i| {
            computer.interpret_entry(&i);
        });
    let computer_creation_time = computer_creation_time.elapsed();
    println!("Computer created in {computer_creation_time:?}\n");

    let solution_time = Instant::now();
    let small_directories = computer.small_directories();
    let solution_time = solution_time.elapsed();
    let solution = small_directories.iter().map(|n| n.size()).sum::<usize>();
    println!(
        "Part 1: {} small directories found in {solution_time:?}:",
        small_directories.len()
    );
    for dir in &small_directories {
        println!(" - {}", dir.name());
    }
    println!(" # Total size: {} bytes", solution);

    println!();

    print!("Part 2: ");
    let deletion_candidate_time = Instant::now();
    let deletion_candidate = computer.deletion_candidate();
    let deletion_candidate_time = deletion_candidate_time.elapsed();
    if let Some(candidate) = deletion_candidate {
        print!(
            "Deletion candidate: {} ({} bytes)",
            candidate.name(),
            candidate.size()
        );
    } else {
        print!("No deletion candidate found");
    }
    println!(" found in {deletion_candidate_time:?}");
}
