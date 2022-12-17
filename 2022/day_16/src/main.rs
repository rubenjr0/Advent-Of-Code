use std::collections::{HashMap, HashSet, VecDeque};

use pathfinding::prelude::*;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum ValveStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: usize,
    connections: Vec<String>,
    status: ValveStatus,
}

impl Valve {
    fn new(id: &str, flow_rate: usize, connections: Vec<&str>) -> Self {
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

    fn is_open(&self) -> bool {
        self.status == ValveStatus::Open
    }

    fn works(&self) -> bool {
        self.flow_rate > 0
    }

    fn open_valve(&mut self) {
        self.status = ValveStatus::Open;
    }
}

#[derive(Debug)]
struct Maze {
    valves: Vec<Valve>,
    current_valve_id: String,
    minutes_left: usize,
    released_pressure: usize,
    _parents: HashMap<String, String>,
    _vertices: HashSet<String>,
}

impl Maze {
    fn parse(input: &str) -> Maze {
        let valves: Vec<Valve> = input.lines().map(|l| Valve::parse(l)).collect();
        let vertices = valves.iter().map(|v| v.id.clone()).collect();
        let parents = Self::build_parent_set(&valves);

        Maze {
            valves,
            current_valve_id: "AA".to_string(),
            minutes_left: 30,
            released_pressure: 0,
            _parents: parents,
            _vertices: vertices,
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

    fn tick(&mut self) {
        eprintln!(
            "\n== Minute {} @ {}",
            31 - self.minutes_left,
            self.current_valve_id
        );
        let mut rel = 0;
        self.valves.iter().for_each(|v| {
            if v.is_open() {
                rel += v.flow_rate as usize;
            }
        });
        self.released_pressure += rel;
        eprint!("Valves ");
        self.valves.iter().filter(|v| v.is_open()).for_each(|v| {
            eprint!("{} ", v.id);
        });
        eprintln!("are open, releasing {rel} pressure");
        self.minutes_left -= 1;
    }

    fn shortest_path_from_to(&self, from: &String, to: &String) -> Option<Vec<String>> {
        if let Some((mut path, _)) = dijkstra(
            from,
            |id| {
                self.get_valve(id)
                    .connections
                    .iter()
                    .map(|s| (s.to_string(), 1))
            },
            |id| id == to,
        ) {
            path.remove(0);
            Some(path.iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }

    fn best_candidate(&self) -> Option<(String, f32, Vec<String>)> {
        self._vertices
            .par_iter()
            .map(|id| self.get_valve(id))
            .filter(|v| !v.is_open() && v.works() && v.id != self.current_valve_id)
            .flat_map(|v| {
                let shortest_path = self.shortest_path_from_to(&self.current_valve_id, &v.id);
                if let Some(shortest_path) = shortest_path {
                    if shortest_path.len() >= self.minutes_left {
                        return None;
                    }
                    let candidate = (
                        v.id.clone(),
                        (v.flow_rate * (self.minutes_left - shortest_path.len() - 1)) as f32
                            / ((shortest_path.len() + {
                                let distances: Vec<_> = self
                                    ._vertices
                                    .par_iter()
                                    .filter(|s| {
                                        let c = self.get_valve(s);
                                        c.is_open()
                                            && c.works()
                                            && c.id != v.id
                                            && c.id != self.current_valve_id
                                    })
                                    .flat_map(|c| {
                                        self.shortest_path_from_to(&v.id, c)
                                            .and_then(|p| Some(p.len()))
                                    })
                                    .collect();
                                distances.iter().sum::<usize>()
                            }) as f32)
                                .powf(
                                    2. * self
                                        ._vertices
                                        .iter()
                                        .filter(|v| !self.get_valve(v).is_open())
                                        .count() as f32
                                        / self._vertices.len() as f32,
                                ),
                        shortest_path,
                    );
                    eprintln!(
                        " - [{}] Candidate: {:?}, rate: {}",
                        self.current_valve_id, candidate, v.flow_rate
                    );
                    Some(candidate)
                } else {
                    None
                }
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn solve(&mut self) {
        loop {
            if self.minutes_left == 0 {
                break;
            }
            if let Some((best, _, path)) = self.best_candidate() {
                // println!("Moving to {} with score {}", best, score);
                path.iter().for_each(|step| {
                    self.tick();
                    println!("Moving to {step}");
                });
                self.tick();
                println!("Opened {best}");
                self.get_valve_mut(&best).open_valve();
                self.current_valve_id = best;
            } else {
                self.tick();
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut maze = Maze::parse(input);
    maze.solve();
    loop {
        break;
    }
    dbg!(maze.released_pressure);
    if maze.released_pressure <= 1897 {
        panic!("Too low")
    } else {
        println!("{} pressure", maze.released_pressure);
    }
}
