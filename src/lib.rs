use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Config<'a> {
    query : &'a str,
    filename : &'a str,
}

impl<'a> Config<'a> {
    pub fn new(input : &[String]) -> Result<Config, &str> {
        if input.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config{ query : &input[1], filename : &input[2] })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(&config.filename)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut val : usize;

    loop {
        val = reader.read_line(&mut line)?;
        if val == 0 {
            break
        }
        else if line.contains(&config.query) {
            println!("{}", &line);
        }
        line.clear();
    }    
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    #[should_panic(expected = "failure: \"not enough arguments\"")]
    fn no_args() {
        let strings = vec![String::new()];
        Config::new(&strings).expect("failure");
    }
    #[test]
    #[should_panic(expected = "failure: \"not enough arguments\"")]
    fn one_arg() {
        let strings = vec![String::from("minigrep")];
        Config::new(&strings).expect("failure");
    }
    #[test]
    #[should_panic(expected = "failure: \"not enough arguments\"")]
    fn two_args() {
        let strings = vec![String::from("minigrep"), String::from("hello")];
        Config::new(&strings).expect("failure");
    }
}