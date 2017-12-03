mod models { pub mod grammatical_error_rule; }

extern crate regex;

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use regex::*;
use models::grammatical_error_rule::{GrammaticalErrorRule};

fn main() {

    // let txt = contents_of_file("rsrc/mistakes-1.txt").unwrap();
    // println!("{}", txt);
    let rgx = Regex::new(r"(\.|,|!|\?)\s{2,}").unwrap();

    // match contents_of_file("rsrc/mistakes-1.txt") {
    //     Err(_) => println!("Couldn't use the file {}", "File name"),
    //     Ok(f) => {
    //         let grammatical_error_rules = GrammaticalErrorRule::read_rgx_file("rsrc/rules.txt");
    //         let errors = grammatical_error_rules.map(|error_rule|{
    //             error_rule.regex_rules(|{rgx_rule}|{
    //
    //             });
    //         });
    //     }
    // };
}

trait RgxRuleFileReader {
    fn read_rgx_file(path: &str) -> Result<Vec<GrammaticalErrorRule>, Error>;
}

impl RgxRuleFileReader for GrammaticalErrorRule {
    fn read_rgx_file(path: &str) -> Result<Vec<GrammaticalErrorRule>, Error> {
        let mut contents = contents_of_file(path);

        // Get error from contents_of_file if there is any
        if contents.is_err() { return Err(contents.err().unwrap()) };

        Ok(contents.unwrap().split("\n\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|block|{
                // Seperates the grammar rule blocks (syntax: New line)
                let pieces = block.split("\n").collect::<Vec<&str>>();
                let title = pieces[0];

                // Get all the rules for each individual block
                let mut rules: Vec<String> = Vec::new();
                for (i, r) in pieces.into_iter().enumerate() {
                    // Get everthing bigger than 0 (0 = block txt)
                    if i > 0 { rules.push(r.to_string()) }
                }

                return GrammaticalErrorRule::new(rules, title.to_string()) })
            .collect::<Vec<GrammaticalErrorRule>>())
    }
}

fn contents_of_file(path: &str) -> Result<String, Error> {
    let mut contents = String::new();

    let res = File::open(path)
        .unwrap()
        .read_to_string(&mut contents);

    if res.is_err() { return Err(Error::new(ErrorKind::InvalidData, "couldn't load data."))}

    Ok(contents)
}
