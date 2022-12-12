use std::fmt::Display;

use eyre::Result;

type Grid = Vec<Vec<u8>>;
type Coordinate = (usize, usize);

#[derive(Debug)]
pub struct HeightMap {
    grid: Grid,
    height: usize,
    width: usize,
    starting_point: Coordinate,
    ending_point: Coordinate,
    path: Vec<Coordinate>,
}

impl HeightMap {
    pub fn parse(input: &str) -> Result<HeightMap> {
        let mut starting_point = None;
        let mut ending_point = None;
        let grid: Grid = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| match c {
                        'S' => {
                            starting_point = Some((row, col));
                            0
                        }
                        'E' => {
                            ending_point = Some((row, col));
                            25
                        }
                        _ => c as u8 - 97,
                    })
                    .collect()
            })
            .collect();
            let starting_point = starting_point.unwrap();
            let height = grid.len();
            let width = grid[0].len();
            Ok(HeightMap {
            grid,
            height,
            width,
            starting_point,
            ending_point: ending_point.unwrap(),
            path: vec![starting_point],
        })
    }

    pub fn distance(&self, (x, y): &Coordinate) -> usize {
        let (xf, yf) = self.ending_point;
        x.abs_diff(xf) + y.abs_diff(yf)
    }

    fn get_coordinate(&self, (x, y): &Coordinate) -> u8 {
        self.grid[*y][*x]
    }

    fn can_move(&self, from: &Coordinate, to: &Coordinate) -> bool {
        let x = self.get_coordinate(from);
        let y = self.get_coordinate(to);
        y <= x + 1
    }

    pub fn move_candidates(&self) -> Vec<(Coordinate, usize)> {
        let last_position = self.path.last().unwrap();
        let mut candidates = vec![];
        if last_position.0 > 0 {
            let p = (last_position.0 - 1, last_position.1);
            if self.can_move(last_position, &p) {
                candidates.push(p);
            }
        }
        if last_position.0 < self.width - 1 {
            let p = (last_position.0 + 1, last_position.1);
            if self.can_move(last_position, &p) {
                candidates.push(p);
            }
        }
        if last_position.1 > 0 {
            let p = (last_position.0, last_position.1 - 1);
            if self.can_move(last_position, &p) {
                candidates.push(p);
            }
        }
        if last_position.1 < self.height - 1 {
            let p = (last_position.0, last_position.1 + 1);
            if self.can_move(last_position, &p) {
                candidates.push(p);
            }
        }
        let mut candidates: Vec<_> = candidates.into_iter().filter(|c| !self.path.contains(c)).map(|c| (c, self.distance(&c))).collect();
        candidates.sort_by_key(|c| c.1);
        candidates
    }

    pub fn backtrack(&mut self) -> bool {
        let candidates= self.move_candidates();
        // println!("Evaluating {candidates:?}");
        for (candidate, distance) in candidates{
            self.path.push(candidate);
            if candidate == self.ending_point {
                println!("SOLVED");
                return true;
            } 
            if !self.backtrack() {
                self.path.pop();
            } else {
                return true;
            }
        }
        return false;
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{col:3}")?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::HeightMap;

    #[test]
    fn parsing() {
        let test = include_str!("../test.txt");
        let height_map = HeightMap::parse(test);
        assert!(height_map.is_ok())
    }
}
