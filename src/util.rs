use std::{env, iter};
use std::fs;
use std::str::Split;

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
