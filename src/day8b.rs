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


    let mut x = 0;
    for horizontal_line in &map {
        let mut y = 0;
        for val in horizontal_line {
            if x == 0 {
                transposed_map.push(vec![]);
            }
            transposed_map[y].push((val.clone().0, val.1));
            y += 1;
        }
        x += 1;
    }
    let mut max_score = 0;
    for x in 0..input_lines.len() {
        for y in 0..input_lines[x].len() {
            let score = calc_score(&map, &transposed_map, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    return max_score;
}

fn calc_score(map: &Vec<Vec<(String, i32)>>, transposed_map: &Vec<Vec<(String, i32)>>, x: usize, y: usize) -> i32 {
    let mut score = 1;
    let vec = map[x].clone();
    let (left, right_slice) = vec.split_at(y);
    score *= count_visibles(&right_slice.to_vec()); //TODO remove first elem or slice different
    let mut left_side = left.to_vec();
    left_side.reverse();
    score *= count_visibles(&left_side);

    score
}

fn count_visibles(pairs: &Vec<(String, i32)>) -> i32 {
    let mut highest = -1;
    let mut count = 0;
    for pair in pairs {
        if (pair.1 > highest) {
            highest = pair.1;
            count += 1;
        }
    }
    return count;
}

fn int_from_char_in_lines(input_lines: &Vec<String>, x: usize, y: usize) -> i32 {
    let x1: Vec<char> = input_lines[x].chars().collect();
    let x2: i32 = x1[y].to_string().parse().unwrap();
    x2
}

