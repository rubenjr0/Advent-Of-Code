use eyre::Result;
use height_map::HeightMap;

mod height_map;

fn main() -> Result<()> {
    let test = include_str!("../test.txt");
    let mut height_map = HeightMap::parse(test)?;
    println!("{height_map}");
    dbg!(height_map.backtrack());
    Ok(())
}

#[cfg(test)]
mod tests {}
