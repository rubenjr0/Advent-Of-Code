use crate::game_result::GameResult;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    pub fn translate(input: &str) -> Move {
        match input {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    pub fn move_to(&self, result: GameResult) -> Move {
        let moves = vec![Move::Rock, Move::Paper, Move::Scissors];
        match result {
            GameResult::Lose => *moves.iter().find(|m| self.wins_over(m)).unwrap(),
            GameResult::Draw => *self,
            GameResult::Win => *moves.iter().find(|m| m.wins_over(self)).unwrap(),
        }
    }

    pub fn wins_over(&self, other: &Move) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            _ => false,
        }
    }
}
