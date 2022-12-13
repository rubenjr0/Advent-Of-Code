use std::time::Instant;

use eyre::Result;
use serde_json::{json, Value};

fn pad_depth(depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
}

fn compare_blocks(left: &Value, right: &Value, depth: usize) -> bool {
    pad_depth(depth);
    println!("- Compare {left} vs {right}");
    if !left.is_array() || !right.is_array() {
        unreachable!("wtf")
    }
    let left = left.as_array().unwrap();
    let right = right.as_array().unwrap();
    let mut idx = 0;
    loop {
        // dbg!(left, right);
        let left = left.get(idx);
        let right = right.get(idx);
        if right.is_some() && left.is_none() {
            return true;
        }
        if right.is_none() && left.is_some() {
            return false;
        }
        if left.is_none() {
            return false;
        }
        let left = left.unwrap();
        let right = right.unwrap();
        pad_depth(depth);
        println!("  - Compare {left} vs {right}");
        if left.is_number() && right.is_number() {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();
            if left < right {
                pad_depth(depth);
                println!("    - Left side is smaller, so inputs are in the right order");
                return true;
            } else if left > right {
                println!("    - Right side is smaller, so inputs are not in the right order");
                return false;
            }
        } else {
            if left.is_array() && !right.is_array() {
                pad_depth(depth);
                let right = &json!([right]);
                println!("  - Mixed types; convert right to {right}");
                return compare_blocks(left, right, depth + 1);
            } else if !left.is_array() && right.is_array() {
                pad_depth(depth);
                let left = &json!([left]);
                println!("  - Mixed types");
                println!("  - Mixed types; convert left to {left}");
                return compare_blocks(left, right, depth + 1);
            } else {
                if depth == 0 && compare_blocks(left, right, depth + 1) {
                    return true;
                }
            }
        }
        idx += 1;
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let parsing_time = Instant::now();
    let blocks: Vec<_> = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|b| serde_json::from_str::<Value>(b).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let parsing_time = parsing_time.elapsed();

    let solution_time = Instant::now();
    let solution = blocks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let i = i + 1;
            println!("== Pair {i} ==");
            let c = compare_blocks(&b[0], &b[1], 0);
            println!("== {c}\n");
            (i, c)
        })
        .filter(|(_, b)| *b)
        .map(|(i, _)| i)
        .sum::<usize>();
    let solution_time = solution_time.elapsed();

    if solution >= 6695 {
        panic!("Too high: {solution}");
    } else if solution <= 2363 {
        panic!("Too low: {solution}");
    }

    println!("Part one");
    println!(" - Solution: {solution}");
    println!(" - Parsing time: {parsing_time:?}");
    println!(" - Solution time: {solution_time:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::compare_blocks;

    #[test]
    fn part_one() {
        let output = include_str!("../test.txt")
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .map(|b| serde_json::from_str::<Value>(b).unwrap())
                    .collect::<Vec<_>>()
            })
            .enumerate()
            .map(|(i, b)| (i + 1, compare_blocks(&b[0], &b[1], 0)))
            .filter(|(_, b)| *b)
            .map(|(i, _)| i)
            .sum::<usize>();
        assert_eq!(output, 13);
    }
}
