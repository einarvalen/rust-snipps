use std::env;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    let config = minigrep::Config::new(&args, ignore_case).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(2);
    };
}


