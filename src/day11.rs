enum Operator {
    ADD,
    MULTIPLY,
}

struct Operation {
    operator: Operator,
    operand: u64,
}

struct Monkey {
    id: u32,
    op: Operation,
    test_divider: u32,
    result_monkey_true: u32,
    result_monkey_false: u32,
    nb_inspections: u32,
    items: Vec<u64>,
}

pub fn solve(input_lines: Vec<String>, cycles: usize) -> i32 {
    let monkeys: Vec<Monkey> = parse_monkeys(input_lines);
    for i in 0..cycles {
        for monkey in monkeys {
            // TODO execute action, increase inspections, do test and pass item on
        }
    }
    let sorted_scores = monkeys.map(|m| m.nb_inspections).collect().sort();
    return sorted_scores[0] * sorted_scores[1];
}