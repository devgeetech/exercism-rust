pub fn reply(message: &str) -> &str {

    let is_space = message.contains(char::is_whitespace);
    let is_small = message.contains(char::is_lowercase);
    let is_cap = message.contains(char::is_uppercase);
    let is_esc = message.contains(char::is_control);
    let is_number = message.contains(char::is_numeric);
    let trim_message = message.trim();
    let is_ques = trim_message.ends_with("?");
    
    if message.len() == 0 {
        return "Fine. Be that way!"
    } else if (is_space || is_esc) && (!is_small && !is_cap && !is_ques && !is_number) {
        return "Fine. Be that way!"
    } else if is_cap && !is_small && !is_ques {
        return "Whoa, chill out!"
    } else if is_cap && !is_small && is_ques {
        return "Calm down, I know what I'm doing!"
    } else if is_ques {
        return "Sure."
    } else {
        return "Whatever."
    }
}
