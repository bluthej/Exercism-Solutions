pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    let contains_letters = ('A'..='z').any(|l| message.contains(l));
    if message.trim().ends_with("?") {
        if contains_letters && message.to_uppercase() == message {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    }
    if contains_letters && message.to_uppercase() == message {
        return "Whoa, chill out!";
    }
    "Whatever."
}
