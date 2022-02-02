use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct Config<'a, W: io::Write> {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub output: &'a mut W,
}

impl<'a, W: io::Write> Config<'a, W> {
    pub fn new(args: &[String], writer: &'a mut W) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: Config::<W>::is_case_sensitive(args.get(3)),
                output: writer,
            })
        }
    }

    fn is_case_sensitive(command_line_arg: Option<&String>) -> bool {
        match command_line_arg {
            Some(s) => match s.as_str() {
                "case_sensitive" => true,
                "case_insensitive" => false,
                _ => env::var("CASE_INSENSITIVE").is_err(),
            },
            None => env::var("CASE_INSENSITIVE").is_err(),
        }
    }
}

pub fn run<W: io::Write>(config: Config<W>) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        writeln!(config.output, "{}", line)?;
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|&line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|&line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = r#"
Rust:
safe, fast, productive
Pick three.
Duct tape.
        "#;

        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }

    #[test]
    fn multiple_results() {
        let query = "st";
        let contents = r#"
Rust:
safe, fast, productive
Pick three.
        "#;

        assert_eq!(
            vec!["Rust:", "safe, fast, productive"],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = r#"
Rust:
safe, fast, productive
Pick three.
Trust me.
        "#;

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
