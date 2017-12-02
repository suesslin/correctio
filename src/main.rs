mod models { pub mod grammatical_error; }

extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::*;
use models::grammatical_error::{GrammaticalError};

fn main() {
    let mut contents = String::new();
    File::open("rsrc/rules.txt")
        .unwrap()
        .read_to_string(&mut contents);

    let s = contents.split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|block|{
            let pieces = block.split("\n").collect::<Vec<&str>>();
            let title = pieces[0];

            let mut rules: Vec<String> = Vec::new();
            for (i, r) in pieces.into_iter().enumerate() {
                if i == 0 { continue } // Skip as the first item is a title
                rules.push(r.to_string());
            }

            return GrammaticalError::new(rules, title.to_string()) })
        .collect::<Vec<GrammaticalError>>();
}
