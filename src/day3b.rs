use std::env;
use std::fs;
use std::str::Split;
pub use crate::util::*;

pub fn solve(input_lines: Vec<String>) -> String {
    println!("Hello, day3a!");
    println!("nb lines: {}", input_lines.len());
    let mut count = 0;
    for chunk in input_lines.chunks(3) {
        // println!("line: {}", line);
        count += group_score(Vec::from(chunk));
    }
    return count.to_string();
}

fn group_score(lines: Vec<String>) -> i32 {
    // println!("sizes {}-{}", left.len(), right.len());
    for letter in lines[0].chars() {
        if lines[1].contains(letter) && lines[2].contains(letter) {
            return char_score(letter);
        }
    }
    return 0;
}

fn char_score(letter: char) -> i32 {
    let letter_code = char_code(letter);
    if (letter >= 'a' && letter <= 'z') {
        return (1 + letter_code - char_code('a')) as i32;
    }
    return (27 + letter_code - char_code('A')) as i32;
}
