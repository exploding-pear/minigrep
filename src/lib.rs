use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

pub struct Config<'a> {
    query : &'a str,
    filename : &'a str,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(input : &[String]) -> Result<Config, &str> {
        if input.len() < 3 {
            return Err("not enough arguments");
        }

        //if CASE_SENSITIVE doesn't exist, case_sensitive is false
        //if CASE_SENSITIVE does exist, case_sensitive is true
        let case_sensitive = ! env::var("CASE_SENSITIVE").is_err();

        Ok(Config{ query : &input[1], filename : &input[2], case_sensitive: case_sensitive })
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
    //creating a lowercase query for a case insensitive search
    let insensitive_query = config.query.to_lowercase();
    let mut insensitive_line;

    loop {
        val = reader.read_line(&mut line)?;
        if val == 0 {
            break
        }
        //case sensitive check. line being checked and query are both lowercased and compared
        else if config.case_sensitive == false {
            insensitive_line = line.to_lowercase();
            if insensitive_line.contains(& insensitive_query) {
                results.push(line.clone());
            }
            insensitive_line.clear();
        }
        //case sensitive check. compares line to query
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
        env::set_var("CASE_SENSITIVE", "true");
        assert_eq!(results("testfiles/test.txt", "Pick"), 1);
    }
    #[test]
    fn two_hits_insensitive() {
        env::remove_var("CASE_SENSITIVE");
        assert_eq!(results("testfiles/test.txt", "Pick"), 2);
    }
    #[test]
    fn three_hits() {
        env::set_var("CASE_SENSITIVE", "true");
        assert_eq!(results("testfiles/poem.txt", "body"), 3);
    }
    #[test]
    fn case_insensitive() {
        env::remove_var("CASE_SENSITIVE");
        assert_eq!(results("testfiles/test.txt", "rUsT"), 1);
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