pub fn is_armstrong_number(num: u32) -> bool {
    let mut no_of_digits: u32 = 1;
    let mut m = num;
    let mut sum = 0;
    match num {
        0 => (),
        _ => no_of_digits = ((num as f64).log10() + 1.0) as u32
    }
    while m>0 {
        let digit = m%10;
        sum = sum + (digit.pow(no_of_digits));
        m = m/10;
    }
    if sum==num {
        return true
    } else {
        return false
    }
}
