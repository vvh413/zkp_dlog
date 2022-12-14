use rand::{thread_rng, Rng};

use super::modular::{gcd, pow};

pub fn get_prime(t: usize) -> u64 {
    let mut p = thread_rng().gen_range(3..=u64::MAX);
    while !lehmann(p, t) {
        p = thread_rng().gen_range(3..=u64::MAX);
    }
    p
}

pub fn get_coprime(b: u64, p: u64) -> u64 {
    let mut a = thread_rng().gen_range(2..p);
    while gcd(a, b) != 1 {
        a = thread_rng().gen_range(2..p);
    }
    a
}

pub fn lehmann(p: u64, t: usize) -> bool {
    if p % 2 == 0 {
        return false;
    }
    for _ in 0..t {
        let a = thread_rng().gen_range(2..p);
        let b = pow(a, (p - 1) >> 1, p);
        if b != 1 && b != (p - 1) {
            return false;
        }
    }
    true
}
