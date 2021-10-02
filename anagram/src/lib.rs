use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {

    let mut word_letter_set: HashSet<char> = HashSet::new();
    let mut anagram_list: HashSet<&str> = HashSet::new();

    

    for candidate in possible_anagrams {
        if word.len() != candidate.len() {
            continue;
        }
        if word.eq_ignore_ascii_case(candidate) {
            continue;
        }
        
        for c in word.chars() {
            for c_l in c.to_lowercase() {
                
            }
        }


        for d in candidate.chars() {
            for d_l in candidate.chars() {
                
            }
        }

        for i in 0..word.len() {

        }

    }
    
    anagram_list
}
