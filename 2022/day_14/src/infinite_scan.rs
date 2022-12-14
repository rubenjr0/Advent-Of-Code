use crate::scan::{Coordinate, Lines, SandState, Scan, Tile};

#[derive(Debug)]
pub struct InfiniteScan {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    _sand_source: Coordinate,
    _active_sand: Option<Coordinate>,
    units_of_sand: usize,
}
impl Scan for InfiniteScan {
    fn new(input: &str) -> Self {
        let lines: Vec<Lines> = input
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|c| {
                        let (x, y) = c.split_once(",").unwrap();
                        (x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect()
            })
            .collect();
        let height = *lines.iter().flatten().map(|(_, y)| y).max().unwrap() + 3;
        let width = lines.iter().flatten().map(|(x, _)| x).max().unwrap() + 1;
        let width = width * 2;
        let lines: Vec<Lines> = lines
            .into_iter()
            .map(|l| l.into_iter().map(|(x, y)| (x, y)).collect())
            .collect();
        let mut scan = Self {
            tiles: vec![vec![Tile::Air; width]; height],
            width: width,
            _sand_source: (500, 0),
            _active_sand: None,
            units_of_sand: 0,
        };
        scan.tiles[scan._sand_source.1][scan._sand_source.0] = Tile::Generator;
        for lines in &lines {
            scan.draw_lines(&lines);
        }
        scan.draw_lines(&vec![(0, (height - 1)), (width - 1, (height - 1))]);
        scan
    }

    fn get_units_of_sand(&self) -> usize {
        self.units_of_sand
    }

    fn draw_lines(&mut self, lines: &Lines) {
        lines.windows(2).for_each(|line| {
            let (x1, y1) = line[0];
            let (x2, y2) = line[1];
            if x1 == x2 {
                let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                for y in min_y..=max_y {
                    self.tiles[y][x1] = Tile::Rock;
                }
            } else {
                let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                for x in min_x..=max_x {
                    self.tiles[y1][x] = Tile::Rock;
                }
            }
        });
    }

    fn simulate(&mut self) {
        loop {
            if self.tick() {
                break;
            }
        }
    }

    fn tick(&mut self) -> bool {
        if let Some((x, y)) = self._active_sand {
            match self.get_next_move((x, y)) {
                SandState::Next(next) => {
                    self.tiles[y][x] = Tile::Air;
                    self.tiles[next.1][next.0] = Tile::Sand;
                    self._active_sand = Some(next);
                }
                SandState::Overflow => return false,
                SandState::Rest => {
                    self._active_sand = None;
                    self.units_of_sand += 1;
                    if (x, y) == self._sand_source {
                        return true;
                    }
                }
            }
        } else {
            let (x, y) = self._sand_source;
            self._active_sand = Some((x, y));
            self.tiles[y][x] = Tile::Sand;
        }
        false
    }
}

impl InfiniteScan {
    fn get_next_move(&self, (x, y): Coordinate) -> SandState {
        if self.tiles[y + 1][x] == Tile::Air {
            return SandState::Next((x, y + 1));
        }
        if self.tiles[y + 1][x - 1] == Tile::Air {
            return SandState::Next((x - 1, y + 1));
        }
        if x == self.width - 1 {
            return SandState::Overflow;
        } else if self.tiles[y + 1][x + 1] == Tile::Air {
            return SandState::Next((x + 1, y + 1));
        }
        SandState::Rest
    }
}
