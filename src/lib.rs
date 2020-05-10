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
    let results = search(&config)?;

    for itr in results.iter() {
        print!("{}", itr);
    }

    Ok(())
}

//searches the given file for the query, returns a vector of strings
fn search(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open(config.filename)?;
    let mut results = Vec::new();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut val : usize;

    loop {
        val = reader.read_line(&mut line)?;
        if val == 0 {
            break
        }
        else if line.contains(config.query) {
            results.push(line.clone());
        }
        line.clear();
    }
    Ok(results)
}

#[cfg(test)]
mod test{
    use super::*;

    fn results(filename: &str, keyword: &str) -> usize {
        let cmd = vec!["minigrep".to_string(), keyword.to_string(), filename.to_string()];
        let config = Config::new(&cmd).expect("unexpected failure");
        let results = search(&config).expect("error");
        results.len()
    }
    #[test]
    fn one_hit() {
        assert_eq!(results("testfiles/test.txt", "Pick"), 1);
    }
    #[test]
    fn three_hits() {
        assert_eq!(results("testfiles/poem.txt", "body"), 3);
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