// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient: i16 = dividend/divisor;
    let remainder: i16 = dividend%divisor;
    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter
        .enumerate()
        .filter(|&(i, _)| i%2==0)
        .map(|(_, val)| {
            val
        })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        ((0 - self.0).abs() + (0 - self.1).abs())
    }
}

