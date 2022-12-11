pub type Item = u128;
type Id = usize;

type Operation = Box<dyn Fn(Item) -> Item>;
type Test = Box<dyn Fn(Item) -> bool>;
type Decision = Box<dyn Fn(bool) -> Id>;

pub struct Monkey {
    starting_items: Vec<Item>,
    operation: Operation,
    test: Test,
    throw_decision: Decision,
}

impl Monkey {
    pub fn parse(input: &str) -> Monkey {
        let mut lines = input.lines().skip(1);
        let starting_items = parse_items(lines.next().unwrap());
        let operation = parse_operation(lines.next().unwrap());
        let test = parse_test(lines.next().unwrap());
        let throw_decision = parse_throw_decision(lines.take(2).collect());

        Monkey {
            starting_items,
            operation,
            test,
            throw_decision,
        }
    }

    pub fn num_items(&self) -> usize {
        self.starting_items.len()
    }

    pub fn play_round(&mut self, relief: bool, _id: Id) -> Vec<(Id, Item)> {
        #[cfg(feature = "debug")]
        println!("Monkey {} is playing", _id);
        let out = self
            .starting_items
            .iter()
            .map(|item| {
                let worry = (self.operation)(*item);
                let new_worry = if relief { worry / 3 } else { worry };
                let test = (self.test)(new_worry);
                let throw_to = (self.throw_decision)(test);
                #[cfg(feature = "debug")]
                {
                    println!("    Worry level goes to {worry}");
                    println!("    Worry level goes to {new_worry}");
                    if test {
                        println!("    Item passes the test");
                    } else {
                        println!("    Item doesn't pass the test");
                    }
                    println!("  Monkey inspects an item with a worry level of {item}");
                    println!("    Item with worry level {new_worry} thrown to monkey {throw_to}");
                }
                (throw_to, new_worry)
            })
            .collect();
        self.starting_items.clear();
        out
    }

    pub fn catch_item(&mut self, item: Item) {
        self.starting_items.push(item);
    }
}

fn parse_items(line: &str) -> Vec<Item> {
    line.split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(", ")
        .flat_map(|n| n.parse())
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    let parts = line
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .skip(1);
    let mut parts = parts.into_iter();
    let op = parts.next().unwrap();
    let right = parts.next().unwrap().parse::<Item>();
    let closure = match op {
        "*" => |x, y| x * y,
        "+" => |x, y| x + y,
        _ => panic!("Not supported: {op}"),
    };
    Box::new(move |x| closure(x, if let Ok(y) = right { y } else { x }))
}

fn parse_test(line: &str) -> Test {
    let n: Item = line.split_whitespace().last().unwrap().parse().unwrap();
    Box::new(move |x| x % n == 0)
}

fn parse_throw_decision(lines: Vec<&str>) -> Decision {
    let if_true = lines[0].split_whitespace().last().unwrap().parse().unwrap();
    let if_false = lines[1].split_whitespace().last().unwrap().parse().unwrap();

    Box::new(move |x| if x { if_true } else { if_false })
}

#[cfg(test)]
mod tests {
    use crate::monkey::Monkey;

    #[test]
    fn monkey_parsing() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let parsed_monkey = Monkey::parse(input);
        let monkey = Monkey {
            starting_items: vec![79, 98],
            operation: Box::new(|x| x * 19),
            test: Box::new(|x| x % 23 == 0),
            throw_decision: Box::new(|x| if x { 2 } else { 3 }),
        };

        assert_eq!(parsed_monkey.starting_items, monkey.starting_items);

        let parsed_operations: Vec<_> = parsed_monkey
            .starting_items
            .into_iter()
            .map(parsed_monkey.operation)
            .collect();
        let operations: Vec<_> = monkey
            .starting_items
            .into_iter()
            .map(monkey.operation)
            .collect();

        assert_eq!(parsed_operations, operations);

        let parsed_tests: Vec<_> = parsed_operations
            .into_iter()
            .map(parsed_monkey.test)
            .collect();
        let tests: Vec<_> = operations.into_iter().map(monkey.test).collect();

        assert_eq!(parsed_tests, tests);

        let parsed_throw_decisions: Vec<_> = parsed_tests
            .into_iter()
            .map(parsed_monkey.throw_decision)
            .collect();
        let throw_decisions: Vec<_> = tests.into_iter().map(monkey.throw_decision).collect();

        assert_eq!(parsed_throw_decisions, throw_decisions);
    }
}
