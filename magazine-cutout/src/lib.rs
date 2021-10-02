// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();

    for word in magazine {
        let count = magazine_map.entry(word).or_insert(0);
        *count += 1;
    }

    for word2 in note {
        let count = magazine_map.entry(word2).or_insert(0);
        *count -= 1;
    }

    !magazine_map.iter().any(|(_, count)| *count<0)
}
