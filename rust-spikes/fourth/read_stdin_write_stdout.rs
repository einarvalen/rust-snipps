#![allow(unused)]

use std::io::{self, Write};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  for (index, line) in BufReader::new(io::stdin()).lines().enumerate() { println!("{}", line.unwrap()); }
}
