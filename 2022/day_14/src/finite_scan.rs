use std::fmt::Display;

use colorful::Colorful;

use crate::scan::{Coordinate, Lines, SandState, Scan, Tile};

#[derive(Debug)]
pub struct FiniteScan {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
    _sand_source: Coordinate,
    _active_sand: Option<Coordinate>,
    units_of_sand: usize,
}
impl Scan for FiniteScan {
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
        let height = *lines.iter().flatten().map(|(_, y)| y).max().unwrap() + 1;
        let x_offset = *lines.iter().flatten().map(|(x, _)| x).min().unwrap();
        let width = lines.iter().flatten().map(|(x, _)| x).max().unwrap() - x_offset + 1;
        let lines: Vec<Lines> = lines
            .into_iter()
            .map(|l| l.into_iter().map(|(x, y)| (x - x_offset, y)).collect())
            .collect();
        let mut scan = Self {
            tiles: vec![vec![Tile::Air; width]; height],
            height,
            width,
            _sand_source: (500 - x_offset, 0),
            _active_sand: None,
            units_of_sand: 0,
        };
        scan.tiles[scan._sand_source.1][scan._sand_source.0] = Tile::Generator;
        for lines in &lines {
            scan.draw_lines(&lines);
        }
        scan
    }

    fn get_units_of_sand(&self) -> usize {
        self.units_of_sand
    }

    fn simulate(&mut self) {
        loop {
            let overflows = self.tick();
            #[cfg(feature = "visualize")]
            {
                println!("{self}");
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
            if overflows {
                break;
            }
        }
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

    fn tick(&mut self) -> bool {
        if let Some((x, y)) = self._active_sand {
            match self.get_next_move((x, y)) {
                SandState::Next(next) => {
                    self.tiles[y][x] = Tile::Air;
                    self.tiles[next.1][next.0] = Tile::Sand;
                    self._active_sand = Some(next);
                }
                SandState::Overflow => return true,
                SandState::Rest => {
                    self._active_sand = None;
                    self.units_of_sand += 1;
                }
            }
        } else {
            let (x, y) = self._sand_source;
            self._active_sand = Some((x, y + 1));
            self.tiles[y + 1][x] = Tile::Sand;
        }
        false
    }
}

impl FiniteScan {
    fn get_next_move(&self, (x, y): Coordinate) -> SandState {
        if y == self.height - 1 {
            return SandState::Overflow;
        }
        if self.tiles[y + 1][x] == Tile::Air {
            return SandState::Next((x, y + 1));
        }
        if x == 0 {
            return SandState::Overflow;
        } else if self.tiles[y + 1][x - 1] == Tile::Air {
            return SandState::Next((x - 1, y + 1));
        }
        if x == self.width {
            return SandState::Overflow;
        } else if self.tiles[y + 1][x + 1] == Tile::Air {
            return SandState::Next((x + 1, y + 1));
        }
        SandState::Rest
    }
}

impl Display for FiniteScan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{esc}[2J{esc}[1;1H", esc = 27 as char)?;
        for line in &self.tiles {
            for tile in line {
                let c = match tile {
                    Tile::Air => format!(" "),
                    Tile::Rock => format!("#").bg_light_gray().bold().to_string(),
                    Tile::Sand => format!("o").yellow().bold().to_string(),
                    Tile::Generator => format!("+").red().bold().to_string(),
                };
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
