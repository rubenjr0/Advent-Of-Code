use crate::moves::Move;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl GameResult {
    pub fn translate(input: &str) -> GameResult {
        match input {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("Invalid input"),
        }
    }
    pub fn evaluate(me: &Move, rival: &Move) -> GameResult {
        if me == rival {
            Self::Draw
        } else if me.wins_over(rival) {
            Self::Win
        } else {
            Self::Lose
        }
    }
}
