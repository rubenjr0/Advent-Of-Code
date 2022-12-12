use std::fmt::Display;

use eyre::Result;

type Grid = Vec<Vec<u8>>;
type Coordinate = (usize, usize);

#[derive(Debug)]
pub struct HeightMap {
    grid: Grid,
    height: usize,
    width: usize,
    objective: Coordinate,
    path: Vec<Coordinate>,
    best_path: Option<Vec<Coordinate>>,
    visited: Vec<Coordinate>,
}

impl HeightMap {
    pub fn parse(input: &str) -> Result<HeightMap> {
        let mut starting_point = None;
        let mut objective = None;
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
                            objective = Some((row, col));
                            25
                        }
                        _ => c as u8 - 97,
                    })
                    .collect()
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        let starting_point = starting_point.unwrap();
        Ok(HeightMap {
            grid,
            height,
            width,
            objective: objective.unwrap(),
            path: vec![starting_point],
            best_path: None,
            visited: vec![starting_point],
        })
    }

    pub fn distance_to_objective(&self, (y, x): &Coordinate) -> usize {
        let (yf, xf) = self.objective;
        y.abs_diff(yf) + x.abs_diff(xf)
    }

    fn get_coordinate(&self, (y, x): &Coordinate) -> u8 {
        self.grid[*y][*x]
    }

    fn can_move(&self, from: &Coordinate, to: &Coordinate) -> bool {
        let a = self.get_coordinate(from);
        let b = self.get_coordinate(to);
        b <= a + 1
    }

    pub fn move_candidates(&self) -> Vec<(Coordinate, usize)> {
        let position = self.path.last().unwrap();
        let mut candidates = vec![];
        if position.0 > 0 {
            let p = (position.0 - 1, position.1);
            if self.can_move(position, &p) {
                candidates.push(p);
            }
        }
        if position.0 < self.height - 1 {
            let p = (position.0 + 1, position.1);
            if self.can_move(position, &p) {
                candidates.push(p);
            }
        }
        if position.1 > 0 {
            let p = (position.0, position.1 - 1);
            if self.can_move(position, &p) {
                candidates.push(p);
            }
        }
        if position.1 < self.width - 1 {
            let p = (position.0, position.1 + 1);
            if self.can_move(position, &p) {
                candidates.push(p);
            }
        }
        let mut candidates: Vec<_> = candidates
            .into_iter()
            .filter(|c| !(self.path.contains(c) || self.visited.contains(c)))
            .map(|c| (c, self.distance_to_objective(&c)))
            .collect();
        candidates.sort_by_key(|c| c.1);
        candidates
    }

    pub fn solve(&mut self) -> &Option<Vec<Coordinate>> {
        self.backtrack();
        &self.best_path
    }

    fn backtrack(&mut self) {
        println!("{self}");
        if self.path.last().unwrap() == &self.objective {
            if self.best_path.is_none() || self.path.len() < self.best_path.as_ref().unwrap().len()
            {
                let mut best_path = self.path.clone();
                best_path.pop();
                self.best_path = Some(best_path);
            }
            return;
        }
        if self.best_path.is_some() && self.path.len() >= self.best_path.as_ref().unwrap().len() {
            return;
        }
        let candidates = self.move_candidates();
        for (candidate, _) in candidates {
            self.path.push(candidate);
            self.visited.push(candidate);
            self.backtrack();
            self.path.pop();
        }
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for (ri, row) in self.grid.iter().enumerate() {
            for (ci, col) in row.iter().enumerate() {
                if self.path.contains(&(ri, ci)) {
                    write!(f, " # ")?;
                    continue;
                }
                write!(f, " · ")?
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
