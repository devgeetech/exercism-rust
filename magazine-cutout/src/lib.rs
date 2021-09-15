// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();
    let mut note_map = HashMap::new();

    for word in magazine {
        let count = magazine_map.entry(word).or_insert(0);
        *count = *count + 1;
    }

    for word2 in note {
        let count = note_map.entry(word2).or_insert(0);
        *count = *count + 1;
    }

    for (key, value) in &note_map {
        let count = magazine_map.get(key);
        match count {
            None => return false,
            Some(i) => {
                if value > i {
                    return false
                }
            }
        }
    }

    true
}
