# correctio
An extendable recognition tool for grammatical errors in text.
Still work in progress.

## Information
### Regex
correctio follows the syntax of [re2](https://github.com/google/re2) by Google, as the regex crate for Rust is based on
the engine of the latter.
### Grammatical Error Rule
A grammatical error rule might be defined in the rules.txt as follows:
```
Description (one line)
Rule 1 (new line)
Rule 2
Rule ...
finish block (new line)
```

## Used
### Dependencies/Crates
* [regex](https://github.com/rust-lang/regex)
### Other
* [gitignore.io](gitignore.io)
