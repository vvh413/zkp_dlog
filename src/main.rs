use anyhow::Result;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{prover::Prover, verifier::Verifier};

mod modular;
mod prime;
mod prover;
mod verifier;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<()> {
    let t = 1000;

    let peggy = Prover::new(t);
    let victor = Verifier::new(t);
    println!("p = {}", peggy.p);
    println!("A = {}", peggy.a);
    println!("A^x = B (mod p) = {}", peggy.b);

    let (tx1, rx1): (Sender<u64>, Receiver<u64>) = channel(1);
    let (tx2, rx2): (Sender<u64>, Receiver<u64>) = channel(1);
    tokio::join!(peggy.run(tx1, rx2), victor.run(tx2, rx1));

    victor.verify(peggy)?;

    Ok(())
}
