use anyhow::Result;
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::math::{
    modular::{pow, sub},
    prime::{get_coprime, get_prime},
};

pub struct Prover {
    pub p: u64,
    pub a: u64,
    x: u64,
    t: usize,
}

impl Prover {
    pub fn new(t: usize) -> Self {
        let p = get_prime(t);
        let x = get_coprime(p - 1, p);
        let a = thread_rng().gen_range(0..p);
        log::info!("p = {}", p);
        log::info!("A = {}", a);
        Prover { p, x, a, t }
    }

    pub async fn run(&self, tx: Sender<u64>, mut rx: Receiver<u64>) -> Result<()> {
        let b = self.b();
        log::info!("B = A ^ x mod p = {}", b);
        log::info!("sending p, A and B");
        tx.send(self.p).await?;
        tx.send(self.a).await?;
        tx.send(b).await?;

        log::info!("generating r_i");
        let r: Vec<u64> = (0..self.t)
            .map(|_| thread_rng().gen_range(0..self.p - 1))
            .collect();

        log::info!("sending h_i = A ^ r_i mod p");
        for h_i in self.h(&r).iter() {
            tx.send(*h_i).await?;
        }

        let mut bits: Vec<bool> = Vec::new();
        for _ in 0..self.t {
            bits.push(rx.recv().await.unwrap() != 0);
        }
        log::info!("received random bits");
        let j = bits.iter().position(|b_i| *b_i).unwrap();
        log::info!("j = {}", j);

        log::info!("sending s_i = (r_i - r_j * b_i) mod (p - 1)");
        for s_i in self.s(&bits, &r).iter() {
            tx.send(*s_i).await?
        }

        let z = self.z(r[j]);
        log::info!("Z = (x - r_j) mod (p - 1) = {}", z);
        log::info!("sending Z");
        tx.send(z).await?;

        Ok(())
    }

    pub fn b(&self) -> u64 {
        pow(self.a, self.x, self.p)
    }

    pub fn h(&self, r: &Vec<u64>) -> Vec<u64> {
        r.iter().map(|r_i| pow(self.a, *r_i, self.p)).collect()
    }

    pub fn s(&self, bits: &Vec<bool>, r: &Vec<u64>) -> Vec<u64> {
        let j = bits.iter().position(|b_i| *b_i).unwrap();
        let r_j = r[j];
        bits.iter()
            .enumerate()
            .map(|(i, b_i)| {
                if *b_i {
                    sub(r[i], r_j, self.p - 1)
                } else {
                    r[i]
                }
            })
            .collect()
    }

    pub fn z(&self, r_j: u64) -> u64 {
        sub(self.x, r_j, self.p - 1)
    }
}
