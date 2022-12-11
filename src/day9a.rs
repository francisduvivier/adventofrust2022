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

pub fn solve(input_lines: Vec<String>) -> usize {
    // Logic: take place of head unless currently still attached.
    let mut visited_by_tail: HashSet<String> = HashSet::new();
    let mut previous_head_pos = (0 as usize, 0 as usize);
    let mut tail_pos = (0 as usize, 0 as usize);

    for motion in input_lines {
        let split: Vec<&str> = motion.split(" ").collect();
        let (dir, times_string) = (split[0].to_string(), split[1].to_string());
        let times: u32 = times_string.parse().unwrap();
        for _i in 0..times {
            let new_head_pos = update_pos(previous_head_pos, &dir);
            if pos_dist(new_head_pos, tail_pos) > 1 {
                tail_pos = previous_head_pos;
                visited_by_tail.insert(coords_to_key(previous_head_pos));
            }
            previous_head_pos = new_head_pos
        }
    }

    return visited_by_tail.len();
}

fn update_pos(pos: (usize, usize), dir: &str) -> (usize, usize) {
    match dir {
        "U" => (pos.0, pos.1 + 1),
        _ => (pos.0, pos.1)
    }
}

fn pos_dist(p0: (usize, usize), p1: (usize, usize)) -> usize {
    let horizontal_dist: i32 = (p0.0 as i32) - (p1.0 as i32);
    let vertical_dist = (p0.1 as i32) - (p1.1 as i32);
    max(vertical_dist.abs() as usize, horizontal_dist.abs() as usize)
}
