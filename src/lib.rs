use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Qrery: {}", config.query);
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(())
}
