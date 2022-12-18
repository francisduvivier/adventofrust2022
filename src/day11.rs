use std::{env, str};
use std::cmp::max;
use std::fs;
use std::str::Split;
use std::str::from_utf8;
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::format;
use std::iter::FromIterator;

use crate::{coords_to_key, int_from_char_in_lines};

const OP_ADD: &'static str = "+";
const OP_MULTI: &'static str = "+";
const OP_DIV: &'static str = "+";
struct Monkey {
    // Operation: TODO
}

pub fn solve(input_lines: Vec<String>, sum_cycles: usize) -> i32 {
    0
}