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

pub fn solve(input_lines: Vec<String>) -> i32 {
    let visibles: HashSet<String> = HashSet::new();
    let mut map: Vec<Vec<(String, i32)>> = vec![];
    for x in 0..input_lines.len() {
        map.push(vec![]);
        for y in 0..input_lines[x].len() {
            let key = format!("{},{}", x, y).to_string();
            map[x].push((key, int_from_char_in_lines(&input_lines, x, y)))
        }
    }
    return 0;
}

fn int_from_char_in_lines(input_lines: &Vec<String>, x: usize, y: usize) -> i32 {
    let x1: Vec<char> = input_lines[x].chars().collect();
    let x2: i32 = x1[y].to_string().parse().unwrap();
    x2
}

