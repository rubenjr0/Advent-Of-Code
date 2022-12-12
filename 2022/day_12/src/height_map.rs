use eyre::Result;
use pathfinding::{num_traits::Zero, prelude::bfs};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::point::Point;

type Grid = Vec<Vec<u8>>;

#[derive(Debug)]
pub struct HeightMap {
    grid: Grid,
    height: usize,
    width: usize,
    start: Point,
    objective: Point,
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
        let starting_point = Point::new(starting_point.1, starting_point.0);
        let objective = objective.unwrap();
        let objective = Point::new(objective.1, objective.0);
        Ok(HeightMap {
            grid,
            height,
            width,
            start: starting_point,
            objective: objective,
        })
    }

    fn get_coordinate(&self, point: &Point) -> u8 {
        self.grid[point.y][point.x]
    }

    fn can_move(&self, from: &Point, to: &Point) -> bool {
        let a = self.get_coordinate(from);
        let b = self.get_coordinate(to);
        b <= a + 1
    }

    pub fn move_candidates(&self, point: &Point) -> impl Iterator<Item = Point> {
        let mut candidates = vec![];
        if point.x > 0 {
            let left = point.left();
            if self.can_move(point, &left) {
                candidates.push(left);
            }
        }
        if point.x < self.width - 1 {
            let right = point.right();
            if self.can_move(point, &right) {
                candidates.push(right);
            }
        }
        if point.y > 0 {
            let top = point.top();
            if self.can_move(point, &top) {
                candidates.push(top);
            }
        }
        if point.y < self.height - 1 {
            let bottom = point.bottom();
            if self.can_move(point, &bottom) {
                candidates.push(bottom);
            }
        }
        candidates.into_iter()
    }

    pub fn find_shortest_possible(&self) -> Option<Vec<Point>> {
        self.grid
            .par_iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.par_iter()
                    .enumerate()
                    .filter(|(_, p)| p.is_zero())
                    .flat_map(move |(x, _)| self.find_shortest_path_bfs(&Point { x, y }))
                    .min_by_key(|s| s.len())
            })
            .min_by_key(|s| s.len())
    }

    pub fn find_shortest_path(&self) -> Option<Vec<Point>> {
        self.find_shortest_path_bfs(&self.start)
    }

    fn find_shortest_path_bfs(&self, start: &Point) -> Option<Vec<Point>> {
        bfs(
            start,
            |p| self.move_candidates(p).collect::<Vec<_>>(),
            |p| p == &self.objective,
        )
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
