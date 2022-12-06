use colorful::{self, Colorful};
use std::{
    fmt::Display,
    thread,
    time::{Duration, Instant},
};

type State = Vec<Vec<char>>;

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse_instructions(instructions: &str) -> Vec<Instruction> {
        instructions
            .lines()
            .map(|l| {
                let mut data = l.split_whitespace().flat_map(|c| c.parse());
                Instruction {
                    quantity: data.next().unwrap(),
                    from: data.next().unwrap(),
                    to: data.next().unwrap(),
                }
            })
            .collect()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Moving {} from {} to {}",
            format!("{}", self.quantity).bold().yellow(),
            format!("{}", self.from).bold().blue(),
            format!("{}", self.to).bold().red(),
        )
    }
}

#[derive(Debug)]
struct Crane {
    stacks: State,
    instructions: Vec<Instruction>,
}

impl Crane {
    fn new(crates: &str, instructions: &str) -> Crane {
        Crane {
            stacks: build_stacks(crates),
            instructions: Instruction::parse_instructions(instructions),
        }
    }

    fn run_instructions_one(&mut self) -> String {
        for instruction in self.instructions.iter() {
            visualize_state(&self.stacks, &instruction);
            for _ in 0..instruction.quantity {
                let datum = self.stacks[instruction.from - 1].pop().unwrap();
                self.stacks[instruction.to - 1].push(datum);
            }
        }
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    fn run_instructions_two(&mut self) -> String {
        for instruction in self.instructions.iter() {
            let insert_at = self.stacks[instruction.to - 1].len();
            for _ in 0..instruction.quantity {
                let element = self.stacks[instruction.from - 1].pop().unwrap();
                self.stacks[instruction.to - 1].insert(insert_at, element);
            }
        }
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}

fn get_cols(s: &str) -> Vec<usize> {
    s.char_indices()
        .filter(|(_, c)| c == &'[')
        .map(|(i, _)| i / 4)
        .collect()
}

fn build_stacks(crates: &str) -> State {
    let number_stacks: usize = crates
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .flat_map(|c| c.parse::<u8>())
        .count();
    let mut stacks: State = vec![vec![]; number_stacks];
    crates
        .lines()
        .take(crates.lines().count() - 1)
        .for_each(|line| {
            let columns = get_cols(line);
            line.split_whitespace()
                .filter(|c| !c.is_empty())
                .zip(columns)
                .for_each(|(c, idx)| stacks[idx].insert(0, c.chars().nth(1).unwrap()));
        });
    stacks
}

fn visualize_state(state: &State, instruction: &Instruction) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let n_rows = state.iter().map(|c| c.len()).max().unwrap();
    for row in 0..n_rows {
        let r = n_rows - row;
        for (idx, column) in state.iter().enumerate() {
            let element = column.get(r);
            if let Some(c) = element {
                let text = format!("{c:5}").white();
                print!(
                    "{}",
                    if idx + 1 == instruction.from && column.len() - instruction.quantity <= r {
                        text.bold().yellow()
                    } else {
                        text
                    }
                );
            } else {
                print!("     ");
            }
        }
        println!();
    }
    for i in 1..=state.len() {
        let i_txt = format!("{i:<5}").bold();
        print!(
            "{}",
            if i == instruction.from {
                i_txt.blue()
            } else if i == instruction.to {
                i_txt.red()
            } else {
                i_txt.green()
            }
        );
    }
    println!("\n{instruction}");
    thread::sleep(Duration::from_millis(1500));
    println!();
}

fn main() {
    let input = include_str!("../input.txt");

    let mut sections = input.split("\n\n");
    let crates = sections.next().unwrap();
    let instructions = sections.next().unwrap();

    let t = Instant::now();
    let mut crane = Crane::new(crates, instructions);
    let tp = t.elapsed();
    let one = crane.run_instructions_one();
    let ts = t.elapsed() - tp;
    println!("Part one: {one} (parsing: {tp:?}, solution: {ts:?})");

    let t = Instant::now();
    let mut crane = Crane::new(crates, instructions);
    let tp = t.elapsed();
    let two = crane.run_instructions_two();
    let ts = t.elapsed() - tp;
    println!("Part one: {two} (parsing: {tp:?}, solution: {ts:?})");
}
