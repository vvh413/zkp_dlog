use anyhow::{ensure, Result};

pub fn add(x: u64, y: u64, p: u64) -> u64 {
    let r = p - x;
    if y < r {
        x + y
    } else {
        y - r
    }
}

pub fn sub(x: u64, y: u64, p: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        p - y + x
    }
}

pub fn mul(mut x: u64, mut y: u64, p: u64) -> u64 {
    x %= p;
    y %= p;
    if y > x {
        (x, y) = (y, x);
    }
    let mut result = 0;
    while y != 0 {
        if y & 1 != 0 {
            result = add(result, x, p);
        }
        x = add(x, x, p);
        y >>= 1;
    }
    result
}

pub fn pow(mut x: u64, mut y: u64, p: u64) -> u64 {
    x %= p;
    y %= p;
    let mut result = 1;
    while y > 0 {
        if y & 1 != 0 {
            result = mul(result, x, p);
        }
        x = mul(x, x, p);
        y >>= 1;
    }
    result
}

pub fn gcd(x: u64, y: u64) -> u64 {
    let (mut r, mut old_r): (u64, u64) = (y, x);

    while r != 0 {
        let q = old_r / r;
        let qr = q * r;
        old_r = if old_r > qr { old_r - qr } else { qr - old_r };
        (old_r, r) = (r, old_r);
    }
    old_r
}

pub fn inverse(x: u64, p: u64) -> Result<u64> {
    let (mut t, mut newt): (u64, u64) = (0, 1);
    let (mut r, mut newr): (u64, u64) = (p, x);
    let mut n: usize = 0;
    while newr != 0 {
        let q = r / newr;
        let qr = q * newr;
        r = if r > qr { r - qr } else { qr - r };
        (r, newr) = (newr, r);
        (t, newt) = (newt, t + q * newt);
        n += 1;
    }
    ensure!(r <= 1, "not inversable");

    if n & 1 == 0 {
        t = p - t;
    }

    Ok(t)
}
