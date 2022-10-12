use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    math::modular::{add, pow},
    math::prime::{get_coprime, get_prime},
};

pub struct Prover {
    pub p: u64,
    x: u64,
    pub a: u64,

    r: Vec<u64>,
    t: usize,
}

impl Prover {
    pub fn new(t: usize) -> Self {
        let p = get_prime(1000);
        let x = get_coprime(p - 1, p);
        let a = thread_rng().gen_range(0..p);
        log::info!("p = {}", p);
        log::info!("A = {}", a);
        Prover {
            p,
            x,
            a,
            r: (0..t).map(|_| thread_rng().gen_range(0..p - 1)).collect(),
            t,
        }
    }

    pub async fn run(&self, tx: Sender<u64>, mut rx: Receiver<u64>) {
        let b = self.b();
        log::info!("B = A ^ x mod p = {}", b);
        log::info!("sending p, A and B");
        tx.send(self.p).await.unwrap();
        tx.send(self.a).await.unwrap();
        tx.send(b).await.unwrap();

        log::info!("sending h_i = A ^ r_i mod p");
        for h_i in self.h().iter() {
            tx.send(*h_i).await.unwrap();
        }

        let mut b: Vec<bool> = Vec::new();
        for _ in 0..self.t {
            b.push(rx.recv().await.unwrap() != 0);
        }
        log::info!("received random bits");

        log::info!("sending s_i = (r_i + b_i * x) mod p");
        for s_i in self.s(&b).iter() {
            tx.send(*s_i).await.unwrap()
        }
    }

    pub fn b(&self) -> u64 {
        pow(self.a, self.x, self.p)
    }

    pub fn h(&self) -> Vec<u64> {
        self.r.iter().map(|r_i| pow(self.a, *r_i, self.p)).collect()
    }

    pub fn s(&self, b: &Vec<bool>) -> Vec<u64> {
        b.iter()
            .enumerate()
            .map(|(i, b_i)| {
                if *b_i {
                    add(self.r[i], self.x, self.p - 1)
                } else {
                    self.r[i]
                }
            })
            .collect()
    }
}
