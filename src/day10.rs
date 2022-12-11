use std::{env, str};
use std::cmp::max;
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::format;
use std::iter::FromIterator;

use crate::{coords_to_key, int_from_char_in_lines};

const OP_ADDX: &'static str = "addx";
const ADDX_DURATION: i32 = 2;
const OP_NOOP: &'static str = "noop";


pub fn solve(input_lines: Vec<String>, sum_cycles: &Vec<usize>) -> i32 {
    let mut sum_of_wanted_cycles = 0;
    let mut reg_val = 1;
    let lines_as_pieces: Vec<Vec<&str>> = input_lines.iter().map(|line| line.split(" ").collect()).collect();
    let mut i = 0;
    let mut cycle = 0;
    loop {
        let line = &input_lines[i];
        if !line.contains(OP_NOOP) {
            let command = lines_as_pieces[i][0];
            match command {
                OP_ADDX => {
                    let val: i32 = lines_as_pieces[i][1].parse().unwrap();
                    for _j in 0..ADDX_DURATION {
                        // println!("line: {:?}", lines_as_pieces[i]);
                        cycle += 1;
                        if sum_cycles.contains(&cycle) {
                            println!("multiplying with {} for cycle {}", reg_val, cycle);
                            sum_of_wanted_cycles += reg_val;
                        }
                    }
                    reg_val += val;
                }
                _ => {
                    println!("line: {}", line);
                    assert!(false)
                }
            }
        } else {
            cycle += 1;
            if sum_cycles.contains(&cycle) {
                println!("multiplying with {} for cycle {}", reg_val, cycle);
                sum_of_wanted_cycles *= reg_val;
            }
        }
        i += 1;

        if (i >= lines_as_pieces.len()) {
            println!("reg_val {}", reg_val);
            return sum_of_wanted_cycles;
        }
    }
}

fn sign(p0: i32) -> i32 {
    if p0 > 0 { return 1; }
    if p0 < 0 { return -1; }
    return 0;
}

fn update_pos(pos: (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "U" => (pos.0, pos.1 + 1),
        "D" => (pos.0, pos.1 - 1),
        "L" => (pos.0 - 1, pos.1),
        "R" => (pos.0 + 1, pos.1),
        _ => (pos.0, pos.1)
    }
}

fn max_pos_dist(p0: (i32, i32), p1: (i32, i32)) -> usize {
    let (horizontal_dist, vertical_dist) = post_dists(p0, p1);
    max(horizontal_dist.abs() as usize, vertical_dist.abs() as usize)
}

fn post_dists(p0: (i32, i32), p1: (i32, i32)) -> (i32, i32) {
    let horizontal_dist: i32 = (p0.0 as i32) - (p1.0 as i32);
    let vertical_dist = (p0.1 as i32) - (p1.1 as i32);
    (horizontal_dist, vertical_dist)
}
