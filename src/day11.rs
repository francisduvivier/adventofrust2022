use std::str::FromStr;

enum Operator {
    ADD,
    MULTIPLY,
}

struct Operation {
    operator: Operator,
    operand: u32,
}

fn parse_operation(line: &str) -> Operation {
    return Operation {
        operator: Operator::ADD, // TODO implement
        operand: 5, // TODO implement
    };
}

struct Monkey {
    id: u32,
    op: Operation,
    test_divider: u32,
    result_monkey_true: u32,
    result_monkey_false: u32,
    nb_inspections: u64,
    items: Vec<u64>,
}

impl FromStr for Monkey {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(description: &str) -> Result<Self, Self::Err> {
        let items = vec![1];
        Ok(Monkey {
            id: 0,
            op: parse_operation(""),
            test_divider: 0,
            result_monkey_true: 0,
            result_monkey_false: 0,
            nb_inspections: 0,
            items,
        })
    }
}

pub fn solve(input_lines: Vec<String>, cycles: usize) -> u64 {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input_lines);
    for i in 0..cycles {
        for monkey in &mut monkeys {
            for item in &monkey.items {
                // TODO execute action, do test and pass item on
                monkey.nb_inspections += 1;
            }
        }
    }
    monkeys.sort_by(|m1, m2| m2.nb_inspections.cmp(&m1.nb_inspections));
    return monkeys[0].nb_inspections * monkeys[1].nb_inspections;
}

fn parse_monkeys(input: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let joined = input.join(",");
    let resplit = joined.split("Monkey ");
    for monkey_description in resplit {
        println!("monkey {}", monkey_description);
        monkeys.push(parse_monkey(monkey_description));
    }
    monkeys
}

fn parse_monkey(descriptions: &str) -> Monkey {
    Monkey::from_str(descriptions).unwrap()
}