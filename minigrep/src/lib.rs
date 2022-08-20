// #![allow(dead_code, unused)]

use std::{env, error::Error, fs, iter};

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&args.filename)?;

    for line in search(&args.query, &text, &args.case_sensitive)? {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(
    query: &str,
    text: &'a str,
    case_sensitive: &bool,
) -> Result<Vec<&'a str>, &'static str> {
    match if !case_sensitive {
        text.lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect::<Vec<&str>>()
    } else {
        text.lines()
            .filter(|line| line.contains(&query))
            .collect::<Vec<&str>>()
    } {
        lines if lines.is_empty() => Err("nothing found, boss"),
        lines => Ok(lines),
    }
}

pub struct Args {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Args {
    pub fn new(args: &mut iter::Skip<env::Args>) -> Result<Self, &str> {
        let case_sensitive = args.len() > 2;

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query supplied"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no filename supplied"),
        };

        Ok(Args {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_sensitive() {
        let query = "here";
        let text = "\
my name is Alice
here is a test
no tests Here
why are you here?";

        assert_eq!(
            vec!["here is a test", "why are you here?"],
            search(query, text, &true).unwrap()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "here";
        let text = "\
my name is Alice
here is a test
no tests Here
why are you here?";

        assert_eq!(
            vec!["here is a test", "no tests Here", "why are you here?"],
            search(query, text, &false).unwrap()
        )
    }
}
