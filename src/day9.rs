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

pub fn solve(input_lines: Vec<String>, knots: usize) -> usize {
    // Logic: take place of head unless currently still attached.
    let mut visited_by_tail: HashSet<String> = HashSet::new();
    let mut previous_knot_pos = vec![(0 as i32, 0 as i32); knots];
    let mut knot_pos = vec![(0 as i32, 0 as i32); knots];
    visited_by_tail.insert(coords_to_key(knot_pos.last().unwrap()));

    for motion in input_lines {
        let split: Vec<&str> = motion.split(" ").collect();
        let (dir, times_string) = (split[0].to_string(), split[1].to_string());
        let times: u32 = times_string.parse().unwrap();
        for _i in 0..times {
            previous_knot_pos[0] = knot_pos[0];
            knot_pos[0] = update_pos(knot_pos[0], &dir);
            for k in 1..knots {
                if pos_dist(knot_pos[k], knot_pos[k - 1]) > 1 {
                    previous_knot_pos[k] = knot_pos[k];
                    knot_pos[k] = previous_knot_pos[k - 1];
                    // println!("updating tail pos, head {:?}", knot_pos[k - 1]);
                } else {
                    // println!("not updating tail pos, head {:?}", knot_pos[k - 1]);
                }
            }

            visited_by_tail.insert(coords_to_key(knot_pos.last().unwrap()));
        }
    }

    return visited_by_tail.len();
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

fn pos_dist(p0: (i32, i32), p1: (i32, i32)) -> usize {
    let horizontal_dist: i32 = (p0.0 as i32) - (p1.0 as i32);
    let vertical_dist = (p0.1 as i32) - (p1.1 as i32);
    max(horizontal_dist.abs() as usize, vertical_dist.abs() as usize)
}
