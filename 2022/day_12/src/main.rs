use std::time::Instant;

use eyre::Result;
use height_map::HeightMap;

mod height_map;
mod point;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let height_map = HeightMap::parse(input)?;
    let bfs_timer = Instant::now();
    let solution = height_map.find_shortest_path();
    let bfs_timer = bfs_timer.elapsed();
    println!("Part one");
    if let Some(shortest_path) = solution {
        println!(
            " - The shortest path has {} steps [{bfs_timer:?}]",
            shortest_path.len() - 1
        );
    } else {
        println!(" - There's no shortest path D:");
    }
    let bfs_timer = Instant::now();
    let solution = height_map.find_shortest_possible();
    let bfs_timer = bfs_timer.elapsed();
    println!("Part two");
    if let Some(shortest_path) = solution {
        println!(
            " - The shortest possible path has {} steps [{bfs_timer:?}]",
            shortest_path.len() - 1
        );
    } else {
        println!(" - There's no shortest path D:");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::HeightMap;

    #[test]
    fn part_one() {
        let test = include_str!("../test.txt");
        let height_map = HeightMap::parse(test).unwrap();
        let solution = height_map.find_shortest_path();
        assert!(solution.is_some());
        assert_eq!(solution.expect("No solution").len() - 1, 31);
    }

    #[test]
    fn part_two() {
        let test = include_str!("../test.txt");
        let height_map = HeightMap::parse(test).unwrap();
        let solution = height_map.find_shortest_possible();
        assert!(solution.is_some());
        assert_eq!(solution.expect("No solution").len() - 1, 29);
    }
}
