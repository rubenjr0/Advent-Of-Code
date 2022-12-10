use std::fmt::Display;

#[derive(Debug)]
pub struct CRT {
    pixels: Vec<Vec<bool>>,
}

impl CRT {
    pub fn new() -> CRT {
        CRT {
            pixels: vec![vec![false; 40]; 6],
        }
    }

    pub fn draw(&mut self, register: isize, cycle: usize) {
        let row = (cycle - 1) / 40;
        let col = ((cycle - 1) % 40) as isize;
        let left_sprt = register - 1;
        let right_sprt = register + 1;
        if left_sprt == col || register == col || right_sprt == col {
            self.pixels[row][col as usize] = true;
        }
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for pixel in row {
                if *pixel {
                    write!(f, "#")?
                } else {
                    write!(f, " ")?
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
