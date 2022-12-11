use crate::monkey::{Item, Monkey};

pub struct KeepAway {
    monkeys: Vec<Monkey>,
    inspections: Vec<usize>,
    bm: usize,
}

impl KeepAway {
    pub fn new(monkeys: Vec<Monkey>) -> KeepAway {
        let n_monkeys = monkeys.len();
        let bm = monkeys.iter().map(|m| m.modulus()).product();
        KeepAway {
            monkeys,
            inspections: vec![0; n_monkeys],
            bm,
        }
    }

    fn play_round(&mut self, relief: bool) {
        for id in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[id];
            self.inspections[id] += monkey.num_items();
            let throws = monkey.play_round(relief, self.bm, id);
            for (throw_id, throw_item) in &throws {
                self.monkeys[*throw_id].catch_item(*throw_item);
            }
        }
    }

    pub fn simulate(&mut self, rounds: usize, relief: bool) -> Item {
        for _ in 0..rounds {
            self.play_round(relief);
        }
        self.monkey_business()
    }

    pub fn monkey_business(&mut self) -> Item {
        self.inspections.sort();
        self.inspections.iter().rev().take(2).product()
    }
}
