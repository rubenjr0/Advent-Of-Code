mod game_result;
mod moves;
mod strategy;

use strategy::Strategy;

fn main() {
    let strategy = include_str!("../input.txt");
    let mut strategy = Strategy::new_part_two(strategy);
    strategy.run_strategy();
    println!("Final score: {}", strategy.score());
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
        let mut strategy = Strategy::new_part_two(strategy);

        strategy.run_strategy();
        assert_eq!(strategy.score(), 4 + 1 + 7);
    }
}
