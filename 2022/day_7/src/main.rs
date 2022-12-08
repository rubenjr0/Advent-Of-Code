mod computer;
mod filesystem;
mod node;

use computer::Computer;

fn main() {
    let input = include_str!("../input.txt");

    let mut computer = Computer::new();
    input
        .lines()
        .filter(|l| l != &"$ ls")
        .map(|line| line.replace("$ ", ""))
        .for_each(|i| {
            computer.interpret_entry(&i);
        });

    let small_directories = computer.small_directories();
    let solution = small_directories.iter().map(|n| n.size()).sum::<usize>();
    // assert_eq!(solution, 1118405);
    println!("Part 1: {} small directories:", small_directories.len());
    for dir in &small_directories {
        println!(" - {}", dir.name());
    }
    println!(" >> Total size: {} bytes", solution);

    println!();

    print!("Part 2: ");
    let deletion_candidate = computer.deletion_candidate();
    if let Some(candidate) = deletion_candidate {
        println!(
            "Deletion candidate: {} ({} bytes)",
            candidate.name(),
            candidate.size()
        );
    } else {
        println!("No deletion candidate found");
    }
}
