use std::{io::Write, thread, time::Duration};

use crate::monkey::{Item, Monkey};

pub struct KeepAway {
    monkeys: Vec<Monkey>,
    inspections: Vec<Item>,
}

impl KeepAway {
    pub fn new(monkeys: Vec<Monkey>) -> KeepAway {
        let n_monkeys = monkeys.len();
        KeepAway {
            monkeys,
            inspections: vec![0; n_monkeys],
        }
    }

    fn play_round(&mut self, relief: bool) {
        for id in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[id];
            self.inspections[id] += monkey.num_items() as Item;
            let throws = monkey.play_round(relief, id);
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
        #[cfg(feature = "debug")]
        println!("{:?}", self.inspections);
        let mut nums = self.inspections.iter().rev().take(2);
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        let (n, ov) = left.overflowing_mul(*right);
        if ov {
            println!("OVER FLOW");
        } else {
            println!("GOOD");
        }
        n
    }
}
