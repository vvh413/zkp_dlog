use anyhow::{ensure, Result};
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::math::modular::{inverse, mul, pow};

pub struct Verifier {
    t: usize,
}

impl Verifier {
    pub fn new(t: usize) -> Self {
        Verifier { t }
    }

    pub async fn run(&self, tx: Sender<u64>, mut rx: Receiver<u64>) -> Result<()> {
        let p = rx.recv().await.unwrap();
        let a = rx.recv().await.unwrap();
        let b = rx.recv().await.unwrap();
        log::info!("recieved p, A and B");

        let mut h: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            h.push(rx.recv().await.unwrap());
        }

        log::info!("generating random bits");
        let bits: Vec<bool> = (0..self.t).map(|_| thread_rng().gen_bool(0.5)).collect();
        let j = bits.iter().position(|b_i| *b_i).unwrap();
        let ih_j = inverse(h[j], p)?;

        log::info!("sending random bits");
        for b_i in bits.iter() {
            tx.send(*b_i as u64).await?;
        }

        let mut s: Vec<u64> = Vec::new();
        for _ in 0..self.t {
            s.push(rx.recv().await.unwrap());
        }
        log::info!("recieved s_i");

        log::info!("verifying A ^ s_i = h_i * (h_j ^ -b_i) (mod p)");
        self.verify_r_j(p, a, &h, &s, &bits)?;

        let z = rx.recv().await.unwrap();
        log::info!("recieved Z");

        log::info!("verifying A ^ Z = B * (h_j ^ -1) (mod p)");
        self.verify_z(p, a, b, ih_j, z)?;

        log::info!("ok");
        Ok(())
    }

    pub fn verify_r_j(
        &self,
        p: u64,
        a: u64,
        h: &Vec<u64>,
        s: &Vec<u64>,
        bits: &Vec<bool>,
    ) -> Result<()> {
        let j = bits.iter().position(|b_i| *b_i).unwrap();
        let ih_j = inverse(h[j], p)?;
        for (i, b_i) in bits.iter().enumerate() {
            if *b_i {
                log::debug!("{} ^ {:20} = {:20} * {}", a, s[i], h[i], ih_j);
                ensure!(pow(a, s[i], p) == mul(h[i], ih_j, p));
            } else {
                log::debug!("{} ^ {:20} = {:20}", a, s[i], h[i]);
                ensure!(pow(a, s[i], p) == h[i]);
            }
        }
        Ok(())
    }

    pub fn verify_z(&self, p: u64, a: u64, b: u64, ih_j: u64, z: u64) -> Result<()> {
        log::debug!("{} ^ {:20} = {:20} * {}", a, z, b, ih_j);
        ensure!(pow(a, z, p) == mul(b, ih_j, p));
        Ok(())
    }
}
