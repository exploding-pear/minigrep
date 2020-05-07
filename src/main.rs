use std::{env,fs};

struct Config<'a> {
    query : &'a str,
    filename : &'a str,
}

impl<'a> Config<'a> {
    fn new(input : &[String]) -> Config {
        if input.len() < 3 {
            panic!("not enough arguments");
        }

        Config{ query : &input[1], filename : &input[2] }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let configs = Config::new(&args);

    println!("searching for: {}", configs.query);
    println!("in the file: {}", configs.filename);

    let contents = fs::read_to_string(configs.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn too_few_args() {

        for itr in 0..1 {
            match itr {
                0 => {
                    let strings : Vec<String> = vec![String::new()];
                    let _configs = Config::new(&strings);

                }
                1 => {
                    let strings : Vec<String> = vec![String::from("hello")];
                    let _configs = Config::new(&strings);
                }
                _ => {
                    println!("only 3 tests supposed to run in this function");
                }
            }            
        }
    }
}