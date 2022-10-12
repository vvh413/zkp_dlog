use anyhow::{ensure, Result};
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::math::modular::{mul, pow};


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
        log::info!("recieved p, A and B");

        let mut h: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            h.push(rx.recv().await.unwrap());
        }

        log::info!("sending random bits");
        for b_i in self.b.iter() {
            tx.send(*b_i as u64).await.unwrap();
        }

        let mut s: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            s.push(rx.recv().await.unwrap());
        }
        log::info!("recieved s_i");

        log::info!("verifying");
        self.verify(p, a, b, &s, &h).unwrap();
        log::info!("ok");
    }

    pub fn verify(&self, p: u64, a: u64, b: u64, s: &Vec<u64>, h: &Vec<u64>) -> Result<()> {
        for (i, b_i) in self.b.iter().enumerate() {
            if *b_i {
                log::debug!("{} ^ {:20} = {:20} * {}", a, s[i], h[i], b);
                ensure!(pow(a, s[i], p) == mul(h[i], b, p));
            } else {
                log::debug!("{} ^ {:20} = {:20}", a, s[i], h[i]);
                ensure!(pow(a, s[i], p) == h[i]);
            }
        }
        Ok(())
    }
}
