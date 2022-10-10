use anyhow::Result;
use rand::{thread_rng, Rng};

use crate::{
    modular::{gcd, inverse, mul},
    prime::{get_coprime, get_prime},
};

#[test]
fn test_gcd() {
    let mut rng = thread_rng();
    for _ in 0..1000 {
        let p = rng.gen_range(3..=u64::MAX);
        let a = rng.gen_range(3..=u64::MAX);
        assert_eq!(gcd(a, p), num::integer::gcd(a, p));
    }
}

#[test]
fn test_coprime() {
    let mut rng = thread_rng();
    let p = get_prime(1000);
    for _ in 0..1000 {
        let b = rng.gen_range(2..p);
        let a = get_coprime(b, p);
        assert_eq!(gcd(a % p, b), 1);
    }
}

#[test]
fn test_inverse() -> Result<()> {
    let mut rng = thread_rng();
    let p = get_prime(1000);
    for _ in 0..1000 {
        let a = rng.gen_range(2..p);
        let inv_a = inverse(a, p)?;
        assert_eq!(mul(a, inv_a, p), 1);
    }
    Ok(())
}

#[test]
#[should_panic]
fn test_inverse_failed() {
    inverse(2, 6).unwrap();
}
