use std::{env, str};
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve(input_lines: Vec<String>) -> i32 {
    let mut quad: Vec<char> = vec![];
    let mut i = 0;
    for curr_char in input_lines[0].chars() {
        quad.insert(0, curr_char);
        if (quad.len() > 4) {
            quad.pop();
        }
        i += 1;
        if vec_to_set(&quad).len() == 4 {
            return i;
        }
    }
    return 0;
}

fn vec_to_set(vec: &Vec<char>) -> HashSet<&char> {
    HashSet::from_iter(vec)
}