use std::{fs, env, io, error::Error};
use std::io::Read;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // {{{

    let content = match config.input {
        Input::File(path) => fs::read_to_string(path)?,
        Input::Std => {
            let mut buf = String::new();
            match io::stdin().read_to_string(&mut buf) {
                Ok(_) => buf,
                Err(err) => {
                    return Err(Box::new(err));
                },
            }
        },
    };

    let results = if config.ignore_case {
        search_case_ins(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
} // }}}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> { // {{{
    let mut results = vec![];

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
} // }}}

pub fn search_case_ins<'a>(query: &str, content: &'a str) -> Vec<&'a str> { // {{{
    let query = query.to_lowercase();
    let mut results = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
} // }}}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Input<'a> {
    Std,
    File(&'a str),
}

#[derive(Debug)]
pub struct Config<'a> { // {{{
    pub query:       String,
    pub input:       Input<'a>,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        let input = if args.len() < 2 {
            return Err("Usage: minigrep substring [file]
                       If file is - or absent, read from stdin.");
        } else if args.len() < 3 {
            Input::Std
        } else {
            if args[2] == "-" {
                Input::Std
            } else {
                Input::File(&args[2])
            }
        };

        let query = args[1].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, input, ignore_case })
    }
} // }}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() { // {{{
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    } // }}}

    #[test]
    fn case_insensitive() { // {{{
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_ins(query, content));
    } // }}}

    #[test]
    fn config_sets_input_method() { // {{{

        let mut args: Vec<String> = vec![String::from("minigrep"), 
                                         String::from("foo")
                                        ];

        let config = Config::build(&args).unwrap();
        assert_eq!(config.input, Input::Std);

        args.push(String::from("-"));

        let config = Config::build(&args).unwrap();
        assert_eq!(config.input, Input::Std);

        args[2] = String::from("path");

        let config = Config::build(&args).unwrap();
        assert_eq!(config.input, Input::File(&"path"));
    } // }}}
}
