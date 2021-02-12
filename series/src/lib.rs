use std::iter;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series_vec: Vec<String> = Vec::new();
    if digits.len() < len {
        return [].to_vec();
    } else if len==0 {
        series_vec.extend(iter::repeat("".to_string()).take(digits.len()+1));
        return series_vec;
    }
    let no_of_items: i32 = (digits.len() as i32) - ((len - 1) as i32);
    for i in 0..=(no_of_items-1) {
        let pos: usize = i as usize;
        let new_s = digits[pos..(pos+len)].to_string();
        series_vec.push(new_s);
    }
    return series_vec;
}
