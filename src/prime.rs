use rand::{thread_rng, Rng};

use crate::modular::{gcd, pow};

pub fn get_prime(t: usize) -> u64 {
    let mut rng = thread_rng();
    let mut p = rng.gen_range(3..=u64::MAX);
    while !lehmann(p, t) {
        p = rng.gen_range(3..=u64::MAX);
    }
    p
}

pub fn get_coprime(b: u64, p: u64) -> u64 {
    let mut rng = thread_rng();
    let mut a = rng.gen_range(2..p);
    while gcd(a, b) != 1 {
        a = rng.gen_range(2..p);
    }
    a
}

pub fn lehmann(p: u64, t: usize) -> bool {
    if p % 2 == 0 {
        return false;
    }
    let mut rng = thread_rng();
    for _ in 0..t {
        let a = rng.gen_range(2..p);
        let b = pow(a, (p - 1) >> 1, p);
        if b != 1 && b != (p - 1) {
            return false;
        }
    }
    true
}
