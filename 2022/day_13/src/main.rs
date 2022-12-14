use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

use eyre::Result;
use serde_json::{json, Value};

type Blocks = Vec<Vec<Vec<Value>>>;

fn compare_blocks(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
    let mut idx = 0;
    loop {
        let left = left.get(idx);
        let right = right.get(idx);
        if right.is_some() && left.is_none() {
            return Ordering::Less;
        }
        if right.is_none() && left.is_some() {
            return Ordering::Greater;
        }
        if left.is_none() {
            return Ordering::Equal;
        }
        let mut left = left.unwrap().clone();
        let mut right = right.unwrap().clone();
        if left.is_number() && right.is_number() {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();
            if left < right {
                return Ordering::Less;
            } else if left > right {
                return Ordering::Greater;
            }
        } else {
            if left.is_array() && !right.is_array() {
                right = json!([right]);
            } else if !left.is_array() && right.is_array() {
                left = json!([left]);
            }
            match compare_blocks(left.as_array().unwrap(), right.as_array().unwrap()) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }
        idx += 1;
    }
}

fn parse_blocks(input: &str) -> (Blocks, Duration) {
    let parse_time = Instant::now();
    let blocks: Vec<_> = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|b| serde_json::from_str::<Vec<Value>>(b).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let parse_time = parse_time.elapsed();
    (blocks, parse_time)
}

fn part_one(blocks: &Blocks) -> (usize, Duration) {
    let solution_time = Instant::now();
    let solution = blocks
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1, compare_blocks(&b[0], &b[1])))
        .filter(|(_, b)| b == &Ordering::Less)
        .map(|(i, _)| i)
        .sum::<usize>();
    let solution_time = solution_time.elapsed();
    (solution, solution_time)
}

fn part_two(blocks: &Blocks) -> (usize, Duration) {
    let div_2 = serde_json::from_str::<Vec<Value>>("[[2]]").unwrap();
    let div_6 = serde_json::from_str::<Vec<Value>>("[[6]]").unwrap();

    let dividers = &vec![div_2.clone(), div_6.clone()];
    let solution_time = Instant::now();
    let mut packets: Vec<_> = blocks.into_iter().flatten().chain(dividers).collect();
    packets.sort_by(|a, b| compare_blocks(a, b));
    let mut decoder_key = 1;
    for (idx, packet) in packets.into_iter().enumerate() {
        if packet == &div_2 || packet == &div_6 {
            decoder_key *= idx + 1;
            if packet == &div_6 {
                break;
            }
        }
    }
    let solution_time = solution_time.elapsed();
    (decoder_key, solution_time)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    let (blocks, parse_time) = parse_blocks(input);

    let (solution, solution_time) = part_one(&blocks);
    println!("Parsing time: {parse_time:?}");
    println!("Part one: {solution} ({solution_time:?})");

    let (solution, solution_time) = part_two(&blocks);
    println!("Part two: {solution} ({solution_time:?})");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_blocks, part_one, part_two};

    #[test]
    fn test_part_one() {
        let test_input = include_str!("../test.txt");
        let (blocks, _) = parse_blocks(test_input);
        let (solution, _) = part_one(&blocks);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_part_two() {
        let test_input = include_str!("../test.txt");
        let (blocks, _) = parse_blocks(test_input);
        let (solution, _) = part_two(&blocks);
        assert_eq!(solution, 140);
    }
}
