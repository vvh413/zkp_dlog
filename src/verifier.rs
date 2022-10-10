use anyhow::{ensure, Result};
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    modular::{inverse, mul, pow},
    prover::Prover,
};

pub struct Verifier {
    pub b: Vec<bool>,
    pub j: usize,
    pub t: usize,
}

impl Verifier {
    pub fn new(t: usize) -> Self {
        let mut rng = thread_rng();
        let b: Vec<bool> = (0..t).map(|_| rng.gen_bool(0.5)).collect();
        Verifier {
            b: b.clone(),
            j: b.iter().position(|b_i| *b_i).unwrap(),
            t,
        }
    }

    pub async fn run(&self, tx: Sender<u64>, mut rx: Receiver<u64>) {
        let p = rx.recv().await.unwrap();
        let a = rx.recv().await.unwrap();

        let mut h: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            h.push(rx.recv().await.unwrap());
        }

        let inv_h_j = dbg!(inverse(h[self.j], p).unwrap());

        for b_i in self.b.iter() {
            tx.send(*b_i as u64).await.unwrap();
        }
        tx.send(self.j as u64).await.unwrap();

        let mut s: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            s.push(rx.recv().await.unwrap());
        }
        for (i, b_i) in self.b.iter().enumerate() {
            if *b_i {
                assert!(pow(a, s[i], p) == mul(h[i], inv_h_j, p));
            } else {
                assert!(pow(a, s[i], p) == h[i]);
            }
        }

        let z = rx.recv().await.unwrap();
        dbg!(z);
        let b = rx.recv().await.unwrap();
        dbg!(pow(a, z, p));
        dbg!(mul(b, inv_h_j, p));
        assert!(pow(a, z, p) == mul(b, inv_h_j, p));

        println!("ok");
    }

    pub fn verify(&self, peggy: Prover) -> Result<()> {
        let h = peggy.h();
        let inv_h_j = inverse(h[self.j], peggy.p)?;
        let s = peggy.s(&self.b, self.j);
        for (i, b_i) in self.b.iter().enumerate() {
            if *b_i {
                ensure!(pow(peggy.a, s[i], peggy.p) == mul(h[i], inv_h_j, peggy.p));
            } else {
                ensure!(pow(peggy.a, s[i], peggy.p) == h[i]);
            }
        }

        let z = peggy.z(self.j);
        ensure!(pow(peggy.a, z, peggy.p) == mul(peggy.b, inv_h_j, peggy.p));
        Ok(())
    }
}
