pub struct GrammaticalError {
    regexRules: Vec<String>,
    desc: String
} impl GrammaticalError {
    pub fn new(r: Vec<String>, d: String) -> GrammaticalError {
        GrammaticalError{regexRules: r, desc: d}
    }
}
