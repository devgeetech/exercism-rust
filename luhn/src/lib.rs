/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for el in code.chars().filter(|c| !c.is_whitespace()).rev() {
        match el.to_digit(10) {
            Some(x) if count%2 != 0 => { 
                sum += if (x + x) > 9 { (x + x) - 9 } else { x + x };
            },
            Some(x) if count%2 == 0 => {
                sum += x;
            },
            _ => {
                return false
            }
        }
        count += 1;
    }
    
    sum%10 == 0 && count > 1
}
