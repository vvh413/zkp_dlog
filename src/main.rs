use std::env;

use anyhow::Result;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{prover::Prover, verifier::Verifier};

mod math;
mod prover;
mod verifier;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let t: usize = args.get(1).expect("expects param `t`").parse()?;

    let (tx1, rx1): (Sender<u64>, Receiver<u64>) = channel(1);
    let (tx2, rx2): (Sender<u64>, Receiver<u64>) = channel(1);

    tokio::try_join!(
        tokio::spawn(async move {
            let peggy = Prover::new(t);
            peggy.run(tx1, rx2).await.unwrap()
        }),
        tokio::spawn(async move {
            let victor = Verifier::new(t);
            victor.run(tx2, rx1).await.unwrap()
        })
    )?;

    Ok(())
}
