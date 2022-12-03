use std::env;
use std::fs;
use std::str::Split;

pub fn solve(input_lines: Vec<String>) -> String {
    println!("Hello, day3a!");
    println!("nb lines: {}", input_lines.len());
    let mut count = 0;
    for line in input_lines {
        // println!("line: {}", line);
        count += line_score(line);
    }
    return count.to_string();
}

fn line_score(line: String) -> i32 {
    let (left, right) = line.split_at(line.len() / 2);
    // println!("sizes {}-{}", left.len(), right.len());
    for letter in left.chars() {
        if right.contains(letter) {
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

fn char_code(letter: char) -> u8 {
    let mut letter_number: &mut [u8] = &mut [0];
    letter.encode_utf8(letter_number);
    let value = letter_number[0];
    value
}
