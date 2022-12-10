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
    let mut size_map: HashMap<String, i32> = HashMap::new();
    size_map.insert("/".to_string(), 0);
    let max_size = 100000;
    let mut mode = "wait";
    let mut cwd: Vec<&str> = vec![];
    let separator = "/";
    let lines_as_pieces: Vec<Vec<&str>> = input_lines.iter().map(|line| line.split(" ").collect()).collect();
    for line_pieces in lines_as_pieces {
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
                        let dir = line_pieces[2];
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
                        let mut curr_dir = separator.clone().to_string();
                        size_map.insert(curr_dir.clone(), *size_map.get(&curr_dir).unwrap_or(&0) + file_size);
                        println!("Adding file size {} to {}", file_size, curr_dir);
                        for sub_dir in cwd.clone() {
                            // let subby = format!("{}/{}", curr_dir, sub_dir);
                            // curr_dir = subby.clone().as_mut_str();
                            // curr_dir = format!("{}/{}", &curr_dir.clone(), sub_dir);

                            let addition = (sub_dir.to_string() + separator).to_string();
                            curr_dir += addition.clone().as_str();
                            println!("Adding file size {} to {}", file_size, curr_dir);
                            size_map.insert(curr_dir.clone(), *size_map.get(&curr_dir).unwrap_or(&0) + file_size);
                        }
                    }
                }
            }
            &_ => { assert!(false); }
        }
    }
    println!("total size: {}", size_map.get(separator).unwrap());
    return sum_smaller(&size_map, max_size);
}

fn sum_smaller(size_map: &HashMap<String, i32>, max_size: i32) -> i32 {
    let mut sum = 0;
    for (_key, val) in size_map {
        println!(" size_map[{}] is {}", *_key, *val);
        if *val <= max_size {
            sum += val;
        }
    }
    return sum;
}