#![allow(unused)]
//use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
    args[1..].iter().for_each(|file_name| dump_content(file_name));
	//for (_, file_name) in args[1..].iter().enumerate() { dump_content(&file_name); }
}

fn dump_content( file_name: &String) {
	let path = Path::new(&file_name);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.to_string()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
