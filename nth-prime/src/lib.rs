pub fn nth(n: u32) -> u32 {
    const MAX_SIZE:u32 = 1000009;
    let mut is_prime = vec![true; MAX_SIZE as usize];
    
    let mut p = 2;
    while p*p<MAX_SIZE {
        if is_prime[p as usize]==true {
            let mut i = p*p;
            while i<MAX_SIZE {
                is_prime[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut prime_vec = vec![];

    for j in 2..MAX_SIZE {
        if is_prime[j as usize]==true {
            prime_vec.push(j);
        }
    }

    return prime_vec[n as usize];
}
