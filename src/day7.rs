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
    let mut size_map: HashMap<&str, i32> = HashMap::new();
    size_map.insert("/", 10);
    let max_size = 100000;
    let mut mode = "wait";
    let mut cwd: Vec<&str> = vec![];
    for line in input_lines {
        let line_clone = line.clone();
        let split = line_clone.split(" ");
        let line_pieces: Vec<&str> = split.clone().collect();
        if line_pieces[0] == "$" {
            mode = "wait";
        }
        match mode {
            "wait" => {
                let command = line_pieces[1];
                match command {
                    "ls" => {
                        mode = "ls";
                    }
                    "cd" => {
                        let dir = line_pieces[1];
                        match dir {
                            "/" => { cwd.clear(); }
                            ".." => { cwd.pop(); }
                            &_ => { cwd.push(dir); }
                        }
                    }
                    &_ => { assert!(false); }
                }
            }
            "ls" => {
                match line_pieces[0] {
                    "dir" => {
                        //do nothing
                    }
                    &_ => {
                        let file_size: i32 = line_pieces[0].parse().unwrap();
                        let mut curr_dir = String::new();
                        size_map.insert(curr_dir.as_str(), size_map.get("/").unwrap_or(&0) + file_size);
                        for sub_dir in cwd.clone() {
                            // let subby = format!("{}/{}", curr_dir, sub_dir);
                            // curr_dir = subby.clone().as_mut_str();
                            curr_dir = format!("{}/{}", curr_dir, sub_dir);
                            size_map.insert(curr_dir.as_str(), size_map.get("/").unwrap_or(&0) + file_size);
                        }
                    }
                }
            }
            &_ => { assert!(false); }
        }
    }
    sum_smaller(&size_map, max_size);
    return 0;
}

fn sum_smaller(siz_mMap: &HashMap<&str, i32>, max_size: i32) -> i32 {
    let mut sum = 0;
    for (_key, val) in siz_mMap {
        if val <= &max_size {
            sum += val;
        }
    }
    return sum;
}