
#![allow(unused)]
	use std::io::{self, Write};
	fn read_input() -> io::Result<()> {
	    let mut input = String::new();
	    io::stdin().read_line(&mut input)?;
	    println!("You typed: {}", input.trim());
	    Ok(())
	}
fn main() {
	print!("Type something: ");
	io::stdout().flush().unwrap();
	read_input();
}
