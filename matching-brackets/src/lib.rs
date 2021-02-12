pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brac_list: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' =>  {
                brac_list.push(c); 
            },
            ')' | ']' | '}' =>  {
                match brac_list.pop() {
                    Some(last_brac) => {
                        match last_brac {
                            '{' => if c != '}' { return false },
                            '[' => if c != ']' { return false },
                            '(' => if c != ')' { return false },
                            _ => ()
                        }
                    },
                    None => {
                        return false
                    }
                }
            }
            _ => ()  
        }
    }

    if brac_list.len() == 0 {
        return true
    } else {
        return false
    }
}
