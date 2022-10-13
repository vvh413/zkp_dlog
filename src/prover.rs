use anyhow::Result;
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    math::modular::{add, pow},
    math::prime::{get_coprime, get_prime},
};

pub struct Prover {
    pub p: u64,
    pub a: u64,
    x: u64,
    t: usize,
}

impl Prover {
    pub fn new(t: usize) -> Self {
        let p = get_prime(1000);
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

        let mut b: Vec<bool> = Vec::new();
        for _ in 0..self.t {
            b.push(rx.recv().await.unwrap() != 0);
        }
        log::info!("received random bits");

        log::info!("sending s_i = (r_i + b_i * x) mod p");
        for s_i in self.s(&b, &r).iter() {
            tx.send(*s_i).await?
        }
        Ok(())
    }

    pub fn b(&self) -> u64 {
        pow(self.a, self.x, self.p)
    }

    pub fn h(&self, r: &Vec<u64>) -> Vec<u64> {
        r.iter().map(|r_i| pow(self.a, *r_i, self.p)).collect()
    }

    pub fn s(&self, b: &Vec<bool>, r: &Vec<u64>) -> Vec<u64> {
        b.iter()
            .enumerate()
            .map(|(i, b_i)| {
                if *b_i {
                    add(r[i], self.x, self.p - 1)
                } else {
                    r[i]
                }
            })
            .collect()
    }
}
