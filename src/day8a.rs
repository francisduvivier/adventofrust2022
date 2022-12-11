use std::{env, str};
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::format;
use std::iter::FromIterator;
use crate::int_from_char_in_lines;

pub fn solve(input_lines: Vec<String>) -> i32 {
    let mut visibles: HashSet<String> = HashSet::new();
    let mut map: Vec<Vec<(String, i32)>> = vec![];
    let mut transposed_map: Vec<Vec<(String, i32)>> = vec![];
    for x in 0..input_lines.len() {
        map.push(vec![]);
        for y in 0..input_lines[x].len() {
            let key = format!("{},{}", x, y).to_string();
            let new_pair = (key, int_from_char_in_lines(&input_lines, x, y));
            map[x].push(new_pair);
        }
        println!("{:?}", map[x]);
    }

    for horizontal_line in map.clone() {
        add_all_visibles(horizontal_line, &mut visibles);
    }
    let mut x = 0;
    for horizontal_line in map {
        let mut y = 0;
        for val in horizontal_line {
            if x == 0 {
                transposed_map.push(vec![]);
            }
            transposed_map[y].push(val);
            y += 1;
        }
        x += 1;
    }

    for vertical_line in transposed_map {
        add_all_visibles(vertical_line, &mut visibles);
    }
    return visibles.len() as i32;
}

fn add_all_visibles(pairs: Vec<(String, i32)>, visibles: &mut HashSet<String>) {
    let mut cloned = pairs.clone();
    add_visibles(pairs, visibles);
    cloned.reverse();
    add_visibles(cloned, visibles);
}

fn add_visibles(pairs: Vec<(String, i32)>, visibles: &mut HashSet<String>) {
    let mut highest = -1;
    for pair in pairs {
        if (pair.1 > highest) {
            highest = pair.1;
            visibles.insert(pair.0);
        }
    }
}
