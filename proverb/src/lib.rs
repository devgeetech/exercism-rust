pub fn build_proverb(list: &[&str]) -> String {
    let list_len:usize = list.len();
    let mut s = String::from("");
    if list.len()>0 {
        for i in 0..(list_len-1) {
            s = format!("{}For want of a {} the {} was lost.\n", s, list[i], list[i+1]);
        }
        s = format!("{}And all for the want of a {}.", s, list[0]);
    }
    return s.to_string();
}
