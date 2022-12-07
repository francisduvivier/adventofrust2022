use std::{env, str};
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;

pub fn solve(input_lines: Vec<String>) -> String {
    let debug_parse = false;
    println!("nb lines: {}", input_lines.len());
    let (stacks, moves) = parse_input(input_lines, debug_parse);
    println!("moves {}, stacks {}", moves.len(), stacks.len());
    return "TODO".to_string();
}

fn parse_input(input_lines: Vec<String>, debug_parse: bool) -> (Vec<Vec<String>>, Vec<Move>) {
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
                    if debug_parse { println!("stacks[{}][{}].unwrap(): {}", i, (stacks[i].len() - 1), stacks[i].last().unwrap()); }
                }
                i += 1;
            }
        } else if line.starts_with("move") {
            let mv = parse_move(line);
            if debug_parse { println!("Move {} from {} to {}", mv.amount, mv.from, mv.to); }
            moves.push(mv)
        } else if !line.is_empty() && !line.starts_with(" ") {
            print!("l {}", line);
            assert!(false);
        }
    }
    (stacks, moves)
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

