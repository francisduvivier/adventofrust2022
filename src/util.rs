use std::{env, iter};
use std::fs;
use std::str::Split;
use std::collections::HashSet;

pub fn read_input(day: i8) -> Vec<String> {
    let file_path = format!("./input/day{}.txt", day);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("input size [{}]", contents.len());
    return contents.lines().map(|s| s.to_string()).collect();
}

pub fn char_code(letter: char) -> u8 {
    let mut letter_number: &mut [u8] = &mut [0];
    letter.encode_utf8(letter_number);
    let value = letter_number[0];
    value
}


pub fn int_from_char_in_lines(input_lines: &Vec<String>, x: usize, y: usize) -> i32 {
    let x1: Vec<char> = input_lines[x].chars().collect();
    let x2: i32 = x1[y].to_string().parse().unwrap();
    x2
}

pub fn vec_to_set(vec: &Vec<char>) -> HashSet<&char> {
    HashSet::from_iter(vec)
}