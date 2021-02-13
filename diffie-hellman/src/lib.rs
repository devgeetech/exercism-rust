use rand::Rng;

fn mod_exp (base: u64, exp: u64, m: u64) -> u64 {
    let mut b = base;
    let mut e = exp;
    if m == 1 {
        return 0
    }
    let mut r = 1;
    b = b % m;
    while e > 0 {
        if (e % 2) == 1 {
            r = (r*b) % m;
        }
        e = e >> 1; 
        b = (b*b) % m;
    }
    return r
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}
