mod day3a;
mod util;
mod day3b;
mod day4a;
mod day4b;

pub use util::*;

fn main() {
    println!("Hello, world!");
    // println!("Solution: {}", day3a::solve(read_input(3)));
    // println!("Solution: {}", day3b::solve(read_input(3)));
    // println!("Solution: {}", day4a::solve(read_input(4)));
    println!("Solution: {}", day4b::solve(read_input(4)));
}


