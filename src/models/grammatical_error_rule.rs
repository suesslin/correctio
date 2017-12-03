pub struct GrammaticalErrorRule {
    pub regex_rules: Vec<String>,
    desc: String
} impl GrammaticalErrorRule {
    pub fn new(r: Vec<String>, d: String) -> GrammaticalErrorRule {
        GrammaticalErrorRule{regex_rules: r, desc: d}
    }
}
