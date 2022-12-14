use std::time::{Duration, Instant};

use infinite_scan::InfiniteScan;

use crate::{finite_scan::FiniteScan, scan::Scan};

mod finite_scan;
mod infinite_scan;
mod scan;

fn part_one(input: &str) -> ((usize, Duration), Duration) {
    let build_time = Instant::now();
    let scan = FiniteScan::new(input);
    let build_time = build_time.elapsed();

    (get_solution(scan), build_time)
}

fn part_two(input: &str) -> ((usize, Duration), Duration) {
    let build_time = Instant::now();
    let scan = InfiniteScan::new(input);
    let build_time = build_time.elapsed();

    (get_solution(scan), build_time)
}

fn get_solution(mut scan: impl Scan) -> (usize, Duration) {
    let simulation_time = Instant::now();
    scan.simulate();
    let simulation_time = simulation_time.elapsed();

    (scan.get_units_of_sand(), simulation_time)
}

fn solve(input: &str, part: &str, solver: &dyn Fn(&str) -> ((usize, Duration), Duration)) {
    let ((solution, build_time), simulation_time) = solver(input);
    println!("Part {part}");
    println!(" - Solution: {solution}");
    println!(" - Build time: {build_time:?}");
    println!(" - Simulation time: {simulation_time:?}");
}

fn main() {
    let input = include_str!("../input.txt");
    solve(input, "one", &part_one);
    solve(input, "two", &part_two);
}
