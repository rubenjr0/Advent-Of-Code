use crate::{game_result::GameResult, moves::Move};

#[derive(Debug)]
pub struct Strategy {
    moves: Vec<(Move, Move)>,
    score: u16,
}

impl Strategy {
    pub fn new_part_one(input: &str) -> Strategy {
        let moves = input
            .split("\n")
            .flat_map(|line| {
                let mut moves = line.split_whitespace().map(|m| Move::translate(m));
                let rival = moves.next();
                let me = moves.next();
                if rival.is_some() && me.is_some() {
                    Some((rival.unwrap(), me.unwrap()))
                } else {
                    None
                }
            })
            .collect();
        Strategy { moves, score: 0 }
    }

    pub fn new_part_two(input: &str) -> Strategy {
        let moves = input
            .split("\n")
            .flat_map(|line| {
                let mut moves = line.split_whitespace();
                if let Some(rival) = moves.next() {
                    let rival = Move::translate(rival);
                    let need_to = GameResult::translate(moves.next().unwrap());
                    Some((rival, rival.move_to(need_to)))
                } else {
                    None
                }
            })
            .collect();
        Strategy { moves, score: 0 }
    }

    pub fn score(&self) -> u16 {
        self.score
    }

    pub fn run_strategy(&mut self) {
        for (rival, me) in &self.moves {
            let result = GameResult::evaluate(&me, &rival);
            let score = *me as u16 + result as u16;
            println!(
                "{me:?} vs {rival:?}: {result:?} [{:6>} + {score}]",
                self.score
            );
            self.score += score;
        }
    }
}
