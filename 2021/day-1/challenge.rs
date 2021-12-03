use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn df(xs: &Vec<i16>) -> Vec<i16> {
    let df_len = xs.len() - 1;
    let mut dfv: Vec<i16> = Vec::new();
    for i in 0..df_len {
        let v = xs[i + 1] - xs[i];
        dfv.push(v);
    }
    dfv
}

fn count_solutions(xs: Vec<i16>) -> usize {
    xs.into_iter()
        .filter(|&x| x > 0)
        .collect::<Vec<i16>>()
        .len()
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open {}: {}", display, reason),
    };

    let reader = BufReader::new(file);
    let mut lines: Vec<i16> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let val = i16::from_str_radix(&line, 10).unwrap();
        lines.push(val);
    }

    let df_1 = df(&lines);

    println!("Part 1: {}", count_solutions(df_1));

    let mut windows: Vec<i16> = Vec::new();

    for i in 2..lines.len() {
        windows.push(lines[i - 2] + lines[i - 1] + lines[i]);
    }

    let df_2 = df(&windows);

    println!("Part 2: {}", count_solutions(df_2));
}
