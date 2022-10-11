use anyhow::{ensure, Result};
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    modular::{mul, pow},
    prover::Prover,
};

pub struct Verifier {
    pub b: Vec<bool>,
    pub t: usize,
}

impl Verifier {
    pub fn new(t: usize) -> Self {
        let mut rng = thread_rng();
        Verifier {
            b: (0..t).map(|_| rng.gen_bool(0.5)).collect(),
            t,
        }
    }

    pub async fn run(&self, tx: Sender<u64>, mut rx: Receiver<u64>) {
        let p = rx.recv().await.unwrap();
        let a = rx.recv().await.unwrap();
        let b = rx.recv().await.unwrap();

        let mut h: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            h.push(rx.recv().await.unwrap());
        }

        for b_i in self.b.iter() {
            tx.send(*b_i as u64).await.unwrap();
        }

        let mut s: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            s.push(rx.recv().await.unwrap());
        }
        for (i, b_i) in self.b.iter().enumerate() {
            if *b_i {
                assert!(pow(a, s[i], p) == mul(h[i], b, p));
            } else {
                assert!(pow(a, s[i], p) == h[i]);
            }
        }

        log::info!("async ok");
    }

    pub fn verify(&self, peggy: Prover) -> Result<()> {
        let h = peggy.h();
        let s = peggy.s(&self.b);
        for (i, b_i) in self.b.iter().enumerate() {
            if *b_i {
                ensure!(pow(peggy.a, s[i], peggy.p) == mul(h[i], peggy.b, peggy.p));
            } else {
                ensure!(pow(peggy.a, s[i], peggy.p) == h[i]);
            }
        }

        log::info!("sync ok");
        Ok(())
    }
}
