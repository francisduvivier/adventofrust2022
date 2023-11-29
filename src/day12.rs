use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;
use crate::day11::Operator::{ADD, MULTIPLY};

#[derive(Debug, Clone, Copy)]
enum Operator {
    ADD,
    MULTIPLY,
}

#[derive(Debug, Clone, Copy)]
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

fn apply_operation(val: u128, op: &Operation) -> u128 {
    let operand = match op.operand {
        0 => val,
        _ => op.operand as u128
    };
    match op.operator {
        ADD => val + operand,
        MULTIPLY => val * operand
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u32,
    op: Operation,
    test_divider: u32,
    result_monkey_true: usize,
    result_monkey_false: usize,
    nb_inspections: u128,
    items: Vec<u128>,
}

impl FromStr for Monkey {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(description: &str) -> Result<Self, Self::Err> {
        let items = vec![1];
        // Example monkey 0:,  Starting items: 52, 60, 85, 69, 75, 75,  Operation: new = old * 17,  Test: divisible by 13,    If true: throw to monkey 6,    If false: throw to monkey 7,,
        let re = Regex::new(r"([0-9]+):[^:]+: ([ 0-9,]+),.*old ([+*]) ([0-9]+|old),.*by ([0-9]+),.*true:.*key ([0-9]+),.*false:.*key ([0-9]+)[, \n]*").unwrap();
        let caps = re.captures(description).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let items = caps.get(2).unwrap().as_str().split(", ").map(|number_string| number_string.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let operator = caps.get(3).unwrap().as_str();
        let operand = caps.get(4).unwrap().as_str();
        let test_divider = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
        let result_monkey_true = caps.get(6).unwrap().as_str().parse::<usize>().unwrap();
        let result_monkey_false = caps.get(7).unwrap().as_str().parse::<usize>().unwrap();
        Ok(Monkey {
            id,
            op: parse_operation(operator, operand),
            test_divider,
            result_monkey_true,
            result_monkey_false,
            nb_inspections: 0,
            items,
        })
    }
}

pub fn solve(input_lines: Vec<String>, cycles: usize, should_divide: bool) -> u128 {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input_lines);

    let mut modulo: u128 = 1;
    {
        &monkeys.iter().for_each(|m: &Monkey| modulo *= m.test_divider as u128);
    }
    for i in 0..cycles {
        for m in 0..monkeys.len() {
            while *(&monkeys[m].items.len()) > 0 as usize {
                let mut item: u128;
                {
                    item = monkeys[m].items.pop().unwrap()
                }
                let monkey_clone = monkeys[m].clone();
                let mut op_result = apply_operation(item, &monkey_clone.op);
                if should_divide {
                    op_result /= 3;
                }
                op_result = op_result % modulo;
                let test_result = op_result % monkey_clone.test_divider as u128 == 0;
                let true_monkey = monkey_clone.result_monkey_true;
                let false_monkey = monkey_clone.result_monkey_false;
                if test_result {
                    monkeys[true_monkey].items.push(op_result);
                } else {
                    monkeys[false_monkey].items.push(op_result);
                }
                monkeys[m].nb_inspections += 1;
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