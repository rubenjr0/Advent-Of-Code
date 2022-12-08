mod game_result;
mod moves;
mod strategy;

use std::time::Instant;

use strategy::Strategy;

fn main() {
    let input = include_str!("../input.txt");

    let t = Instant::now();
    let mut strategy = Strategy::new_part_one(input);
    let parsing_1 = t.elapsed();
    let t = Instant::now();
    strategy.run_strategy();
    let result_1 = t.elapsed();
    println!("Part one final score: {}", strategy.score());

    let t = Instant::now();
    let mut strategy = Strategy::new_part_two(input);
    let parsing_2 = t.elapsed();
    let t = Instant::now();
    strategy.run_strategy();
    let result_2 = t.elapsed();
    println!("Part two final score: {}", strategy.score());

    println!("# Times");
    println!("> Part one");
    println!(">> Parsing: {parsing_1:?}");
    println!(">> Result: {result_1:?}");

    println!("> Part two");
    println!(">> Parsing: {parsing_2:?}");
    println!(">> Result: {result_2:?}");
}

#[cfg(test)]
mod tests {
    use crate::{game_result::GameResult, moves::Move, Strategy};

    #[test]
    fn rock_wins_scissors() {
        let result = GameResult::evaluate(&Move::Rock, &Move::Scissors);
        assert_eq!(result, GameResult::Win);
    }
    #[test]
    fn scissors_loses_rock() {
        let result = GameResult::evaluate(&Move::Scissors, &Move::Rock);
        assert_eq!(result, GameResult::Lose);
    }
    #[test]
    fn paper_wins_rock() {
        let result = GameResult::evaluate(&Move::Paper, &Move::Rock);
        assert_eq!(result, GameResult::Win);
    }
    #[test]
    fn rock_loses_paper() {
        let result = GameResult::evaluate(&Move::Rock, &Move::Paper);
        assert_eq!(result, GameResult::Lose);
    }
    #[test]
    fn scissors_win_paper() {
        let result = GameResult::evaluate(&Move::Scissors, &Move::Paper);
        assert_eq!(result, GameResult::Win);
    }
    #[test]
    fn paper_loses_scissors() {
        let result = GameResult::evaluate(&Move::Paper, &Move::Scissors);
        assert_eq!(result, GameResult::Lose);
    }

    #[test]
    fn basic_strategy() {
        let strategy = "A Y\nB X\nC Z";
        let mut strategy = Strategy::new_part_one(strategy);

        strategy.run_strategy();
        assert_eq!(strategy.score(), 8 + 1 + 6);
    }
}
