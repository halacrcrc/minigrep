use std::fs;
use std::env;
use std::error::Error;

pub static mut LINE:&str = "a";

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;

    println!("Search for: \"{}\"", config.query);
    println!("Open with: \"{}\"", config.filename);
    println!("Trim with: \"{}\"", config.trim_case);
    
    if config.case_insensitive {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{}",line);
        }
    } else {
        for line in search(&config.query, &contents, &config.trim_case) {
            println!("{}",line);
        }
    } 
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub trim_case: String,
    pub case_insensitive: bool
}

impl Config {
    pub fn new (mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() != 4 {
            return Err("wrong argumens!");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没见着字符啊")
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("没见着文件啊")
        };

        let trim_case = match args.next() {
            Some(trim_case) => trim_case,
            None => return Err("没见着要删的啊")
        };

        let case_insensitive = env::var("CASE_INSENSEITIVE").is_ok();
    
        Ok(Config { query, filename, trim_case,case_insensitive })
    }
}


pub fn search<'a>(query: &str, contents:&'a str, trim_case: &str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            let line = line.trim_start_matches(trim_case).trim();
            results.push(line);
        }
    }

    results
   
    // contents.lines()
    //     .filter(|line| line.contains(query))
    //     .collect()

}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let trim_case = "";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, trim_case));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["safe, fast, productive."], 
            search_case_insensitive(query, contents));
    }
}