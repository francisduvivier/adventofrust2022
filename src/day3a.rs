use std::env;
use std::fs;
use std::str::Split;

pub fn solve(inputLines: Vec<String>) -> String {
    println!("Hello, day3a!");
    println!("nb lines: {}", inputLines.len());
    let mut count = 0;
    for line in inputLines {
        // println!("line: {}", line);
        count += get_score(line);
    }
    return count.to_string();
}

fn get_score(line: String) -> i32 {
    let (left, right) = line.split_at(line.len() / 2);
    println!("sizes {}-{}", left.len(), right.len());

    return 0;
}
