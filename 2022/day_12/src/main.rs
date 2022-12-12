use eyre::Result;
use height_map::HeightMap;

mod height_map;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let mut height_map = HeightMap::parse(input)?;
    // println!("{height_map}");
    let solution = height_map.solve();
    if let Some(solution) = solution {
        println!("Solved in {} steps", solution.len());
    } else {
        println!("No solution found");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::HeightMap;

    #[test]
    fn part_one() {
        let test = include_str!("../test.txt");
        let mut height_map = HeightMap::parse(test).unwrap();
        let solution = height_map.solve();
        assert_eq!(solution.as_ref().unwrap().len(), 31);
    }
}
