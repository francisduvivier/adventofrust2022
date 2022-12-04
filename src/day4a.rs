use std::env;
use std::fs;
use std::str::Split;

pub fn solve(input_lines: Vec<String>) -> String {
    println!("nb lines: {}", input_lines.len());
    let mut count = 0;
    for line in input_lines {
        // println!("line: {}", line);
        count += line_score(line);
    }
    return count.to_string();
}

fn line_score(line: String) -> i32 {
    let number_pairs = line.split(',').map(|numbers| numbers.split('-').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    // println!("sizes {}-{}", left.len(), right.len());
    let lower_left: i32 = number_pairs[0][0].parse().unwrap();
    let upper_left: i32 = number_pairs[0][1].parse().unwrap();
    let lower_right: i32 = number_pairs[1][0].parse().unwrap();
    let upper_right: i32 = number_pairs[1][1].parse().unwrap();
    if (lower_left <= lower_right && upper_left >= upper_right) {
        return 1;
    }
    if (lower_left >= lower_right && upper_left <= upper_right) {
        return 1;
    }
    return 0;
}
