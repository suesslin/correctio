pub struct GrammaticalError {
    pub regex_rules: Vec<String>,
    desc: String
} impl GrammaticalError {
    pub fn new(r: Vec<String>, d: String) -> GrammaticalError {
        GrammaticalError{regex_rules: r, desc: d}
    }
}
