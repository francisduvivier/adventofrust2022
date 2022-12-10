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
    let mut visibles: HashSet<String> = HashSet::new();
    let mut map: Vec<Vec<(String, i32)>> = vec![];
    let mut transposed_map: Vec<Vec<(String, i32)>> = vec![];
    for x in 0..input_lines.len() {
        map.push(vec![]);
        for y in 0..input_lines[x].len() {
            let key = format!("{},{}", x, y).to_string();
            let new_pair = (key, int_from_char_in_lines(&input_lines, x, y));
            map[x].push(new_pair);
            // let new_transposed_pair = (key.clone(), int_from_char_in_lines(&input_lines, x, y));
        }
        println!("{:?}", map[x]);
    }

    for horizontal_line in map.clone() {
        add_visibles(horizontal_line, &mut visibles);
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
        add_visibles(vertical_line, &mut visibles);
    }
    return visibles.len() as i32;
}

fn add_visibles(pairs: Vec<(String, i32)>, visibles: &mut HashSet<String>) {
    // todo!()
    for pair in pairs {
        visibles.insert(pair.0); //TODO
    }
}

fn int_from_char_in_lines(input_lines: &Vec<String>, x: usize, y: usize) -> i32 {
    let x1: Vec<char> = input_lines[x].chars().collect();
    let x2: i32 = x1[y].to_string().parse().unwrap();
    x2
}

