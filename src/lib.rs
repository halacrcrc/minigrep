use std::fs;
<<<<<<< HEAD
use std::env;
=======
>>>>>>> 671073eba19e6a70c4aaa608221ee036830f71e0
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;

    println!("Search for: \"{}\"", config.query);
<<<<<<< HEAD
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
=======
    println!("open with: \"{}\"", config.filename);
    println!("\nopen with:\n{}",contents);

>>>>>>> 671073eba19e6a70c4aaa608221ee036830f71e0
    Ok(())
}

pub struct Config {
    pub query: String,
<<<<<<< HEAD
    pub filename: String,
    pub trim_case: String,
    pub case_insensitive: bool
=======
    pub filename: String
>>>>>>> 671073eba19e6a70c4aaa608221ee036830f71e0
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
<<<<<<< HEAD
        if args.len() != 4 {
=======
        if args.len() != 3 {
>>>>>>> 671073eba19e6a70c4aaa608221ee036830f71e0
            return Err("wrong argumens!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
<<<<<<< HEAD
        let trim_case = args[3].clone();
        let case_insensitive = env::var("CASE_INSENSEITIVE").is_ok();
    
        Ok(Config { query, filename, trim_case,case_insensitive })
    }
}

pub fn search<'a>(query: &str, contents:&'a str, trim_case: &str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in contents.lines() {
        let line1 = line.trim_start_matches(trim_case).trim();
        if line.contains(query) {
            results.push(line1);
        }
    }

    results
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
=======
    
        Ok(Config { query, filename })
>>>>>>> 671073eba19e6a70c4aaa608221ee036830f71e0
    }
}