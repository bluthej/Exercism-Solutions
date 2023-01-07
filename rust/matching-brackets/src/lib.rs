pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brack: String = string.chars().filter(|&c| "{[(}])".contains(c)).collect();
    let paren = ["()", "{}", "[]"];
    while paren.iter().any(|s| brack.contains(s)) {
        paren.iter().for_each(|s| brack = brack.replace(s, ""))
    }
    brack.is_empty()
}
