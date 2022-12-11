use std::time::Instant;

mod keep_away;
mod monkey;

use keep_away::KeepAway;
use monkey::Monkey;

fn main() {
    let input = include_str!("../input.txt");
    let monkeys = input
        .split("\n\n")
        .map(|block| Monkey::parse(block))
        .collect();

    let parsing_time = Instant::now();
    let mut keep_away = KeepAway::new(monkeys);
    let _parsing_time = parsing_time.elapsed();
    let simulation_time = Instant::now();
    let monkey_business = keep_away.simulate(20, true);
    let _simulation_time = simulation_time.elapsed();
    println!("Part one");
    println!(" - Monkey business is {monkey_business}");
    #[cfg(feature = "bench")]
    {
        println!(" - Parsing took {_parsing_time:?}");
        println!(" - Simulation took {_simulation_time:?}");
    }

    let monkeys = input
        .split("\n\n")
        .map(|block| Monkey::parse(block))
        .collect();
    let mut keep_away = KeepAway::new(monkeys);
    let _parsing_time = parsing_time.elapsed();
    let monkey_business = keep_away.simulate(10_000, false);
    let _simulation_time = simulation_time.elapsed();
    println!("Part two");
    println!(" - Monkey business is {monkey_business}");
    #[cfg(feature = "bench")]
    {
        println!(" - Parsing took {_parsing_time:?}");
        println!(" - Simulation took {_simulation_time:?}");
    }
}

#[cfg(test)]
mod tests {
    use crate::{keep_away::KeepAway, monkey::Monkey};

    #[test]
    fn part_one() {
        let test = include_str!("../test.txt");
        let monkeys: Vec<Monkey> = test
            .split("\n\n")
            .map(|block| Monkey::parse(block))
            .collect();

        assert_eq!(monkeys.len(), 4);

        let mut keep_away = KeepAway::new(monkeys);

        let monkey_business = keep_away.simulate(20, true);
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn part_two() {
        let test = include_str!("../test.txt");
        let monkeys: Vec<Monkey> = test
            .split("\n\n")
            .map(|block| Monkey::parse(block))
            .collect();

        let mut keep_away = KeepAway::new(monkeys);

        let monkey_business = keep_away.simulate(10000, false);
        assert_eq!(monkey_business, 2713310158);
    }
}
