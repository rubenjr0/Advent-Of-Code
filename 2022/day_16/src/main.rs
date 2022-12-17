use core::time;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    thread,
    time::Duration,
};

use pathfinding::{
    num_traits::Pow,
    prelude::{directions::S, *},
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Clone)]
enum ValveStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: isize,
    connections: Vec<String>,
    status: ValveStatus,
}

impl Valve {
    fn new(id: &str, flow_rate: isize, connections: Vec<&str>) -> Self {
        Self {
            id: String::from(id),
            flow_rate,
            connections: connections.iter().map(|s| s.trim().to_string()).collect(),
            status: ValveStatus::Closed,
        }
    }

    fn parse(input: &str) -> Self {
        let id = input.split_whitespace().nth(1).unwrap();
        let flow_rate = input
            .split('=')
            .nth(1)
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let connections = input
            .split("to valve")
            .nth(1)
            .unwrap()
            .chars()
            .skip(1)
            .collect::<String>();
        let connections = connections.split(',').collect();
        Self::new(id, flow_rate, connections)
    }

    fn open_valve(&mut self) {
        self.status = ValveStatus::Open;
    }
}

#[derive(Debug)]
struct Maze {
    valves: Vec<Valve>,
    current_valve_id: String,
    minutes_left: isize,
    released_pressure: usize,
    _parents: HashMap<String, String>,
    _vertices: Vec<String>,
}

impl Maze {
    fn parse(input: &str) -> Maze {
        let valves = input.lines().map(|l| Valve::parse(l)).collect();
        let parents = Self::build_parent_set(&valves);

        Maze {
            valves,
            current_valve_id: "AA".to_string(),
            minutes_left: 30,
            released_pressure: 0,
            _parents: parents,
            _vertices: vec![],
        }
    }

    fn build_parent_set(valves: &Vec<Valve>) -> HashMap<String, String> {
        let mut parents = HashMap::new();
        let mut queue: VecDeque<&String> = VecDeque::new();
        queue.push_back(&valves.first().unwrap().id);
        let mut visited: HashSet<&String> = HashSet::new();
        while let Some(current) = queue.pop_front() {
            visited.insert(current);
            for child in &valves
                .iter()
                .find(|v| &v.id == current)
                .unwrap()
                .connections
            {
                if visited.contains(child) {
                    continue;
                } else {
                    queue.push_back(child);
                    parents.insert(child.to_string(), current.to_string());
                }
            }
        }
        parents
    }

    fn get_valve(&self, id: &str) -> &Valve {
        self.valves.iter().find(|v| v.id == id).unwrap()
    }

    fn get_valve_mut(&mut self, id: &str) -> &mut Valve {
        self.valves.iter_mut().find(|v| v.id == id).unwrap()
    }

    fn move_to(&mut self, destination_id: String) {
        self.tick();
        println!("You move to valve {destination_id}");
        self.current_valve_id = destination_id.clone();
    }

    fn open(&mut self) {
        self.tick();
        println!("Opening {}", self.current_valve_id);
        let destination = self.get_valve_mut(&self.current_valve_id.clone());
        destination.open_valve();
    }

    fn tick(&mut self) {
        eprintln!(
            "\n== Minute {} @ {}",
            31 - self.minutes_left,
            self.current_valve_id
        );
        let mut rel = 0;
        self.valves.iter().for_each(|v| {
            if v.status == ValveStatus::Open {
                rel += v.flow_rate as usize;
            }
        });
        self.released_pressure += rel;
        eprint!("Valves ");
        self.valves
            .iter()
            .filter(|v| v.status == ValveStatus::Open)
            .for_each(|v| {
                eprint!("{} ", v.id);
            });
        eprintln!("are open, releasing {rel} pressure");
        self.minutes_left -= 1;
    }

    fn solve(&mut self) -> Solution {
        let solution = self.backtrack(
            Solution::new(30, self.valves.iter().filter(|v| v.flow_rate > 0).count()),
            0,
        );
        eprintln!("Solution: {solution:?}");
        solution
    }

    fn backtrack<'a>(&'a self, current: Solution, depth: usize) -> Solution {
        let is_target = current.actions
            == vec![
                Action::Move("DD".to_string()),
                Action::Open("DD".to_string()),
                Action::Move("CC".to_string()),
                /*
                Action::Move("BB".to_string()),
                Action::Open("BB".to_string()),
                Action::Move("AA".to_string()),
                Action::Move("II".to_string()),
                Action::Move("JJ".to_string()),
                Action::Open("JJ".to_string()), */
            ];
        if is_target {
            eprintln!(
                "{:?}, open? [{:?} -> {}]",
                current,
                current.actions.last().unwrap(),
                current.actions.last().unwrap().is_open(),
            );
            thread::sleep(Duration::from_secs(1));
        }

        if current.remaining_minutes == 0 {
            return current;
        }

        let mut best = current.clone();
        let position = &current._position;
        let valve = self.get_valve(position).clone();
        let remaining_minutes = current.remaining_minutes - 1;
        let released = current.released + current.rate;
        let is_open = current
            .actions
            .iter()
            .any(|a| a.contains(&current._position) && a.is_open());
        if is_target {
            dbg!(is_open);
            thread::sleep(Duration::from_secs(20));
        }
        if !is_open && valve.flow_rate > 0 {
            let mut open_action = current.actions.clone();
            open_action.push(Action::Open(current._position.clone()));
            let open_solution = Solution {
                actions: open_action,
                released,
                rate: current.rate + valve.flow_rate as usize,
                remaining_minutes,
                openable: current.openable - 1,
                _position: current._position.clone(),
            };
            let open_solution = self.backtrack(open_solution, depth + 1);
            if open_solution.released > best.released {
                best = open_solution;
                // eprintln!("New best: {best:?} (depth {depth})");
            }
        }
        let mut best_candidates: Vec<_> = valve
            .connections
            .iter()
            .map(|id| self.get_valve(id))
            .collect();
        best_candidates.sort_by(|a, b| b.flow_rate.cmp(&a.flow_rate));
        let best_move = best_candidates
            .par_iter()
            .map(|child| {
                let mut move_action = current.actions.clone();
                move_action.push(Action::Move(child.id.clone()));
                let move_solution = Solution {
                    actions: move_action,
                    released,
                    remaining_minutes,
                    rate: current.rate,
                    openable: current.openable,
                    _position: child.id.clone(),
                };
                self.backtrack(move_solution, depth + 1)
            })
            .max_by_key(|s| s.released);
        if let Some(best_move) = best_move {
            if best_move.released > best.released {
                // eprintln!("New best: {best:?} (depth {depth})");
                best = best_move
            }
        }
        best
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Move(String),
    Open(String),
}

impl Action {
    fn is_open(&self) -> bool {
        match self {
            Self::Move(_) => false,
            _ => true,
        }
    }
    fn contains(&self, id: &String) -> bool {
        match self {
            Self::Move(id2) => id == id2,
            Self::Open(id2) => id == id2,
        }
    }
}

#[derive(Debug, Clone)]
struct Solution {
    actions: Vec<Action>,
    released: usize,
    rate: usize,
    remaining_minutes: usize,
    openable: usize,
    _position: String,
}

impl Solution {
    fn new(remaining_minutes: usize, openable: usize) -> Self {
        Self {
            actions: vec![],
            released: 0,
            rate: 0,
            remaining_minutes,
            openable,
            _position: "AA".to_string(),
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{:?}] - Released: {}, Rate: {}, Minutes left: {}, openable: {}, position: {}",
            self.actions.last(),
            self.released,
            self.rate,
            self.remaining_minutes,
            self.openable,
            self._position
        )
    }
}

fn main() {
    let input = include_str!("../test.txt");
    let mut maze = Maze::parse(input);
    'timer: loop {
        /*  let mut best_path = maze.best_path();
        while let Some(step) = best_path.pop() {
            maze.move_to(step.clone());
            if maze.minutes_left == 0 {
                println!("You escaped with {} pressure", maze.released_pressure);
                break 'timer;
            }
            if best_path.len() == 0 && maze.get_valve(&step).flow_rate > 0 {
                maze.open();
            }
            if maze.minutes_left == 0 {
                println!("You escaped with {} pressure", maze.released_pressure);
                break 'timer;
            }
        } */
        maze.solve();
        break;
    }
    /*   if maze.released_pressure <= 1897 {
        panic!("Too low")
    } else {
        println!("{} pressure", maze.released_pressure);
    } */
}
