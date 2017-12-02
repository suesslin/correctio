mod models { pub mod grammatical_error; }

extern crate regex;

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use regex::*;
use models::grammatical_error::{GrammaticalError};

fn main() {
    let grammatical_errors = GrammaticalError::read_rgx_file("rsrc/rules.txt");
    for error in grammatical_errors.unwrap() {
        println!("New block");
        for rule in error.regex_rules {
            println!("{}", rule);
        }
    }
}

trait RgxRuleFileReader {
    fn read_rgx_file(path: &str) -> Result<Vec<GrammaticalError>, Error>;
}

impl RgxRuleFileReader for GrammaticalError {
    fn read_rgx_file(path: &str) -> Result<Vec<GrammaticalError>, Error> {
        let mut contents = String::new();

        let res = File::open(path)
            .unwrap()
            .read_to_string(&mut contents);

        if res.is_err() { return Err(Error::new(ErrorKind::InvalidData, "couldn't load data.")) }

        Ok(contents.split("\n\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|block|{
                // Seperates the grammar rule blocks (syntax: New line)
                let pieces = block.split("\n").collect::<Vec<&str>>();
                let title = pieces[0];

                // Get all the rules for each individual block
                let mut rules: Vec<String> = Vec::new();
                for (i, r) in pieces.into_iter().enumerate() {
                    if i == 0 { continue } // Skip as the first item is a title
                    rules.push(r.to_string());
                }

                return GrammaticalError::new(rules, title.to_string()) })
            .collect::<Vec<GrammaticalError>>())
    }
}
