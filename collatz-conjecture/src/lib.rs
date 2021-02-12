pub fn collatz(n: u64) -> Option<u64> {
    let mut i = 0;
    let mut m = n;
    match m {
        0 => return None,
        _ => {
            while m!=1 {
                if m%2==0 {
                    m=m/2;
                } else {
                    m=(3*m)+1;
                }
                i=i+1;
            }
            return Some(i)
        }
    }
}
