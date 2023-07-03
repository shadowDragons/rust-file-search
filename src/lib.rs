use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(query: String, file: String, ignore_case: bool) -> Result<Config, &'static str> {
        Ok(Config {
            query,
            file,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file).unwrap_or_else(|_| {
        eprintln!("wenjian cuowu");
        process::exit(1);
    });
    let rs = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in rs {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
