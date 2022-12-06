use std::time::Instant;

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

#[derive(Debug)]
struct Crane {
    stacks: Vec<Vec<char>>,
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

fn build_stacks(crates: &str) -> Vec<Vec<char>> {
    let number_stacks: usize = crates
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .flat_map(|c| c.parse::<u8>())
        .count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; number_stacks];
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
