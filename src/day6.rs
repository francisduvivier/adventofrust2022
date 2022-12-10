use std::{env, str};
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::vec_to_set;

pub fn solve(input_lines: Vec<String>, amount: usize) -> i32 {
    let mut quad: Vec<char> = vec![];
    let mut i = 0;
    for curr_char in input_lines[0].chars() {
        quad.insert(0, curr_char);
        if (quad.len() > amount) {
            quad.pop();
        }
        i += 1;
        if vec_to_set(&quad).len() == amount {
            return i;
        }
    }
    return 0;
}