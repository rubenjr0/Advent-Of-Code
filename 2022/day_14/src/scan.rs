use std::fmt::Display;

use colorful::Colorful;

pub type Coordinate = (usize, usize);
pub type Lines = Vec<Coordinate>;
type Tiles = Vec<Vec<Tile>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Air,
    Rock,
    Sand,
    Generator,
}

pub enum SandState {
    Rest,
    Overflow,
    Next(Coordinate),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScanType {
    Finite,
    Infinite,
}

#[derive(Debug)]
pub struct Scan {
    tiles: Tiles,
    height: usize,
    width: usize,
    sand_source: Coordinate,
    active_sand: Option<Coordinate>,
    units_of_sand: usize,
    scan_type: ScanType,
}

impl Scan {
    pub fn new(input: &str, scan_type: ScanType) -> Self {
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
        let height = *lines.iter().flatten().map(|(_, y)| y).max().unwrap()
            + match scan_type {
                ScanType::Finite => 1,
                ScanType::Infinite => 3,
            };
        let x_offset = match scan_type {
            ScanType::Finite => *lines.iter().flatten().map(|(x, _)| x).min().unwrap(),
            ScanType::Infinite => 0,
        };
        let mut width = lines.iter().flatten().map(|(x, _)| x).max().unwrap() - x_offset + 1;
        if scan_type == ScanType::Infinite {
            width *= 2;
        }
        let (sgx, sgy) = (500 - x_offset, 0);
        let lines: Vec<Lines> = lines
            .into_iter()
            .map(|l| l.into_iter().map(|(x, y)| (x - x_offset, y)).collect())
            .collect();
        let mut tiles = vec![vec![Tile::Air; width]; height];
        tiles[sgy][sgx] = Tile::Generator;
        for lines in &lines {
            Self::draw_lines(&mut tiles, &lines);
        }
        if scan_type == ScanType::Infinite {
            Self::draw_lines(
                &mut tiles,
                &&vec![(0, (height - 1)), (width - 1, (height - 1))],
            );
        }
        Self {
            tiles,
            height,
            width,
            sand_source: (sgx, sgy),
            active_sand: None,
            units_of_sand: 0,
            scan_type,
        }
    }

    pub fn get_units_of_sand(&self) -> usize {
        self.units_of_sand
    }

    fn draw_lines(tiles: &mut Tiles, lines: &Lines) {
        lines.windows(2).for_each(|line| {
            let (x1, y1) = line[0];
            let (x2, y2) = line[1];
            if x1 == x2 {
                let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                for y in min_y..=max_y {
                    tiles[y][x1] = Tile::Rock;
                }
            } else {
                let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                for x in min_x..=max_x {
                    tiles[y1][x] = Tile::Rock;
                }
            }
        });
    }

    fn get_next_move(&self, (x, y): Coordinate) -> SandState {
        if self.scan_type == ScanType::Finite && y == self.height - 1 {
            return SandState::Overflow;
        }
        if self.tiles[y + 1][x] == Tile::Air {
            return SandState::Next((x, y + 1));
        }
        if self.scan_type == ScanType::Finite && x == 0 {
            return SandState::Overflow;
        } else if self.tiles[y + 1][x - 1] == Tile::Air {
            return SandState::Next((x - 1, y + 1));
        }
        if x == self.width - 1 {
            return SandState::Overflow;
        } else if self.tiles[y + 1][x + 1] == Tile::Air {
            return SandState::Next((x + 1, y + 1));
        }
        SandState::Rest
    }

    fn tick(&mut self) -> bool {
        if let Some((x, y)) = self.active_sand {
            match self.get_next_move((x, y)) {
                SandState::Next(next) => {
                    self.tiles[y][x] = Tile::Air;
                    self.tiles[next.1][next.0] = Tile::Sand;
                    self.active_sand = Some(next);
                }
                SandState::Overflow => return self.scan_type == ScanType::Finite,
                SandState::Rest => {
                    self.active_sand = None;
                    self.units_of_sand += 1;
                    if self.scan_type == ScanType::Infinite && (x, y) == self.sand_source {
                        return true;
                    }
                }
            }
        } else {
            let (x, y) = self.sand_source;
            self.active_sand = Some((x, y));
            self.tiles[y][x] = Tile::Sand;
        }
        false
    }

    pub fn simulate(&mut self) {
        loop {
            #[cfg(feature = "visualize")]
            {
                if self.scan_type == ScanType::Finite {
                    println!("{self}");
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            }
            if self.tick() {
                break;
            }
        }
    }
}

impl Display for Scan {
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
