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
use crate::{coords_to_key, int_from_char_in_lines};

pub fn solve(input_lines: Vec<String>) -> i32 {
    let mut map: Vec<Vec<(String, i32)>> = vec![];
    let mut transposed_map: Vec<Vec<(String, i32)>> = vec![];
    for x in 0..input_lines.len() {
        map.push(vec![]);
        for y in 0..input_lines[x].len() {
            let key = coords_to_key((x as i32, y as i32));
            let new_pair = (key, int_from_char_in_lines(&input_lines, x, y));
            map[x].push(new_pair);
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
    let mut score = calc_score_1_dir(&map, x, y);
    score *= calc_score_1_dir(&transposed_map, y, x);
    score
}

fn calc_score_1_dir(map: &&Vec<Vec<(String, i32)>>, x: usize, y: usize) -> i32 {
    let vantage_height = map[x][y].1;
    let mut score = 1;
    let left = &map[x][..y];
    let right_slice = &map[x][y + 1..];
    score *= count_visibles(&right_slice.to_vec(), vantage_height);
    let mut left_side = left.to_vec();
    left_side.reverse();
    score *= count_visibles(&left_side, vantage_height);
    score
}

fn count_visibles(pairs: &Vec<(String, i32)>, max_size: i32) -> i32 {
    let mut count = 0;
    for pair in pairs {
        count += 1;
        if (pair.1 >= max_size) {
            break;
        }
    }
    return count;
}

