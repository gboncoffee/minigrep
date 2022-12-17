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
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
} // }}}

pub fn search_case_ins<'a>(query: &str, content: &'a str) -> Vec<&'a str> { // {{{
    let query = query.to_lowercase();

    content.lines()
        .filter(|line| line
            .to_lowercase()
            .contains(&query))
        .collect()
} // }}}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Input {
    Std,
    File(String),
}

#[derive(Debug)]
pub struct Config { // {{{
    pub query:       String,
    pub input:       Input,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ) -> Result<Config, &'static str> {

        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("\
Error parsing arguments: Usage: minigrep substring [file]
If file is - or absent, read from stdin."),
        };

        let input = match args.next() {
            Some(path) => {
                if path == "-" {
                    Input::Std
                } else {
                    Input::File(path)
                }
            },
            None => Input::Std,
        };

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

        let mut args = Vec::new();
        args.push(String::from("minigrep"));
        args.push(String::from("foo"));

        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.input, Input::Std);

        let mut args = Vec::new();
        args.push(String::from("minigrep"));
        args.push(String::from("foo"));
        args.push(String::from("-"));

        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.input, Input::Std);

        let mut args = Vec::new();
        args.push(String::from("minigrep"));
        args.push(String::from("foo"));
        args.push(String::from("path"));

        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.input, Input::File(String::from("path")));
    } // }}}
}
