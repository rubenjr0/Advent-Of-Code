use std::time::Instant;

mod cpu;
mod crt;

use cpu::{Instruction, CPU};

fn main() {
    let input = include_str!("../input.txt");
    let mut cpu = CPU::new();

    let timer = Instant::now();
    input.lines().for_each(|line| {
        let instruction = Instruction::parse(line);
        cpu.run_instruction(instruction);
    });
    let time = timer.elapsed();

    let signal_strength = cpu.signal_strength();
    println!("Part one");
    println!(" - The signal strength is {signal_strength}");
    println!(" - Ran in {time:?}");
    println!(
        " - {:?} / instruction\n",
        time / input.lines().count() as u32
    );
    println!("{}", cpu.display());
}

#[cfg(test)]
mod tests {
    use crate::cpu::{Instruction, CPU};

    #[test]
    fn part_one() {
        let mut cpu = CPU::new();

        let instructions = include_str!("../test.txt");
        instructions.lines().for_each(|line| {
            let instruction = Instruction::parse(line);
            cpu.run_instruction(instruction);
        });

        let signal_strength = cpu.signal_strength();

        assert_eq!(signal_strength, 13140);
    }
}
