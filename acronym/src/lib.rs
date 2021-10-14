pub fn abbreviate(phrase: &str) -> String {
    let mut acronym: String = String::from("");
    let word_list: Vec<&str> = phrase
        .split(|x: char| {
            x=='-' || x==' ' || x=='_' 
        })
        .collect();
    for word in word_list {
        if word.len() > 0 {
            acronym.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
            let lower: Vec<&str> = word.matches(|x: char| x.is_ascii_lowercase()).collect();
            if !lower.is_empty() {
                let capitals = word.matches(|x: char| x.is_ascii_uppercase()).skip(1);
                capitals.for_each(|y| {
                    acronym.push(y.chars().nth(0).unwrap())
                })
            }
        }
    }
    acronym
}
