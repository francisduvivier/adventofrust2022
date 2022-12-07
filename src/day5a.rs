use std::{env, str};
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;

pub fn solve(input_lines: Vec<String>) -> String {
    let debug = false;
    println!("nb lines: {}", input_lines.len());
    let mut stacks: Vec<Vec<String>> = vec![vec![]; 9];
    let mut moves: Vec<Move> = vec![];
    for line in input_lines {
        if Regex::new(r"^(\[| {4})").unwrap().is_match(line.as_str()) {
            let chunks = line.as_bytes().chunks(4);
            let mut i = 0;
            for chunk in chunks {
                let chunk_string = from_utf8(chunk).unwrap();
                if chunk_string.starts_with("[") {
                    let letter = chunk_string.substring(1, 2);
                    stacks[i].push(letter.to_string());
                    if debug { println!("stacks[{}][{}].unwrap(): {}", i, (stacks[i].len() - 1), stacks[i].last().unwrap()); }
                }
                i += 1;
            }
        } else if line.starts_with("move") {
            let mv = parse_move(line);
            if debug { println!("Move {} from {} to {}", mv.amount, mv.from, mv.to); }
            moves.push(mv);
        } else if !line.is_empty() && !line.starts_with(" ") {
            print!("l {}", line);
            assert!(false);
        }
    }
    println!("moves {}, stacks {}", moves.len(), stacks.len());
    for mut i in 0..stacks.len() {
        if debug { println!("stacks last before [{}]", stacks[i].last().unwrap()); }
        stacks[i].reverse();
        if debug { println!("stacks first after [{}]", stacks[i].first().unwrap()); }
    }
    for mv in moves {
        for _ in 0..mv.amount {
            let popped = stacks[(mv.from - 1) as usize].pop().unwrap();
            if debug { println!("popped {} from {}", popped, mv.from as usize) }
            stacks[(mv.to - 1) as usize].push(popped);
        }
    }
    let mut chars: Vec<&str> = vec![];
    for mut i in 0..stacks.len() {
        chars.push(stacks[i].last().unwrap())
    }
    return chars.join("");
}

struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

fn parse_move(line: String) -> Move {
    //eg. move 3 from 5 to 4
    let matcher = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    for caps in matcher.captures_iter(&line) {
        let mv = Move { amount: caps[1].parse().unwrap(), from: caps[2].parse().unwrap(), to: caps[3].parse().unwrap() };
        return mv;
    }
    return Move { amount: 0, from: 0, to: 0 };
}

