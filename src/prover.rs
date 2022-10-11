use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    modular::{add, pow},
    prime::{get_coprime, get_prime},
};

pub struct Prover {
    pub p: u64,
    x: u64,

    pub a: u64,
    pub b: u64,

    pub r: Vec<u64>,
    pub t: usize,
}

impl Prover {
    pub fn new(t: usize) -> Self {
        let mut rng = thread_rng();
        let p = get_prime(1000);
        let x = get_coprime(p - 1, p);
        let a = rng.gen_range(2..p);
        Prover {
            p,
            x,
            a,
            b: pow(a, x, p),
            r: (0..t).map(|_| rng.gen_range(2..p - 1)).collect(),
            t,
        }
    }

    pub async fn run(&self, tx: Sender<u64>, mut rx: Receiver<u64>) {
        tx.send(self.p).await.unwrap();
        tx.send(self.a).await.unwrap();
        tx.send(self.b).await.unwrap();

        for h_i in self.h().iter() {
            tx.send(*h_i).await.unwrap();
        }

        let mut b: Vec<bool> = Vec::new();
        for _ in 0..self.t {
            b.push(rx.recv().await.unwrap() != 0);
        }

        for s_i in self.s(&b).iter() {
            tx.send(*s_i).await.unwrap()
        }
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