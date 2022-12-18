use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;
use crate::day11::Operator::{ADD, MULTIPLY};

enum Operator {
    ADD,
    MULTIPLY,
}

struct Operation {
    operator: Operator,
    operand: u32,
}

fn parse_operation(operator: &str, operand: &str) -> Operation {
    let operator: Operator = match operator {
        "+" => ADD,
        "*" => MULTIPLY,
        &_ => {
            assert!(false);
            MULTIPLY
        }
    };
    let operand = match operand {
        "old" => 0 as u32,
        _ => {
            operand.parse::<u32>().unwrap()
        }
    };
    return Operation {
        operator,
        operand,
    };
}

fn apply_operation(val: u64, op: Operation) -> u64 {
    let operand = match op.operand {
        0 => val,
        _ => op.operand as u64
    };
    match op.operator {
        ADD => val + operand,
        MULTIPLY => val * operand
    }
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
        // Example monkey 0:,  Starting items: 52, 60, 85, 69, 75, 75,  Operation: new = old * 17,  Test: divisible by 13,    If true: throw to monkey 6,    If false: throw to monkey 7,,
        let re = Regex::new(r"([0-9]+):[^:]+: ([ 0-9,]+),.*old ([+*]) ([0-9]+|old),.*").unwrap();
        let caps = re.captures(description).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let items = caps.get(2).unwrap().as_str().split(", ").map(|number_string| number_string.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let operator = caps.get(3).unwrap().as_str();
        let operand = caps.get(4).unwrap().as_str();
        Ok(Monkey {
            id,
            op: parse_operation(operator, operand),
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
    let resplit = joined.split("Monkey ").filter(|element| !element.is_empty());
    for monkey_description in resplit {
        println!("monkey {}", monkey_description);
        monkeys.push(parse_monkey(monkey_description));
    }
    monkeys
}

fn parse_monkey(descriptions: &str) -> Monkey {
    Monkey::from_str(descriptions).unwrap()
}