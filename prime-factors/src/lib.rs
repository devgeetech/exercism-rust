pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_facs = Vec::new();
    let mut m = n;
    let mut boot = 2;
    while boot<=m {
        while m%boot==0 {
            prime_facs.push(boot);
            m = m/boot;
        }
        boot = boot + 1;
    }
    return prime_facs
}
