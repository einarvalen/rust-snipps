use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    println!("Searthing for {} in the file {}", config.query, config.filename);
    let contents  = fs::read_to_string(&config.filename).expect("Something went wrong reading the file");
    let res = search(&config, &contents);
    let count = res.len();
    for line in res {
        println!("{}", line);
    }
    Ok(count)
}

//pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    if config.ignore_case {
        contents.lines().filter(|line| line.to_lowercase().contains(&config.query)).collect()
    } else {
        contents.lines().filter(|line| line.contains(&config.query)).collect()
    }
}

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String], ignore_case: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: minigrep query filename");
        }
        let query = if ignore_case { args[1].clone().to_lowercase() } else { args[1].clone() };
        let filename = args[2].clone();
        Ok(Config { query, filename, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_case() {
        let config = Config::new(&["minigrep".to_string(),"PROBLEM".to_string(), "src/main.rs".to_string()], true);
        assert_eq!(1, run(config.unwrap()).unwrap());
    }

    #[test]
    fn match_case() {
        let config = Config::new(&["minigrep".to_string(),"Problem".to_string(), "src/main.rs".to_string()], false);
        assert_eq!(1, run(config.unwrap()).unwrap());
    }
}
