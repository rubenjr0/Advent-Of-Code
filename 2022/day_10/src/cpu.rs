use crate::crt::CRT;

#[derive(Debug)]
pub enum Instruction {
    Addx(isize),
    Noop,
}

impl Instruction {
    pub fn parse(instruction: &str) -> Instruction {
        let mut parts = instruction.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "addx" => Instruction::Addx(parts.next().unwrap().parse::<isize>().unwrap()),
            "noop" => Instruction::Noop,
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    register: isize,
    cycle: usize,
    display: CRT,
    signal_strength: isize,
    next_signal_cycle: usize,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register: 1,
            cycle: 0,
            display: CRT::new(),
            signal_strength: 0,
            next_signal_cycle: 20,
        }
    }

    pub fn display(&self) -> &CRT {
        &self.display
    }

    pub fn signal_strength(&self) -> isize {
        self.signal_strength
    }

    fn tick_cycle(&mut self) {
        self.cycle += 1;

        self.display.draw(self.register, self.cycle);

        if self.cycle == self.next_signal_cycle {
            self.signal_strength += self.cycle as isize * self.register;
            self.next_signal_cycle += 40;
        }
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Addx(value) => self.addx(value),
            Instruction::Noop => self.noop(),
        }
    }

    fn addx(&mut self, value: isize) {
        self.tick_cycle();
        self.tick_cycle();
        self.register += value;
    }

    fn noop(&mut self) {
        self.tick_cycle();
    }
}
