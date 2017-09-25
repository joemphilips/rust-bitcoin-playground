#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey, ExtendedPrivKey};
use rand::random;
use secp256k1::Secp256k1;

use wallet::txbuilder::{Builder, BuilderCommand};
use wallet::blockchain_observer::BlockchainObserver;


fn init(network: Network) -> Result<ExtendedPrivKey> {
  let secp = Secp256k1::new();
  let seed: u8 = random();
  let sk = ExtendedPrivKey::new_master(&secp, network, &[seed])?;
  Ok(sk)
}

/// struct responsible for keychain info, balances and covert those formats.
pub struct Wallet<'a> {
  blockchain_observers: Vec<Box<BlockchainObserver<'a>>>,
  txbuilder: Builder,
}


impl<'a> Wallet<'a> {
  pub fn new(config: &'a HashMap<String, String>) -> Self {
    Wallet {
      blockchain_observers: vec![Box::new(BlockchainObserver::new(config).unwrap())],
      txbuilder: Builder {},
    }
  }

  pub fn start(&self) -> Result<()> {
    use bitcoin::network::listener::Listener;
    for obs_box in self.blockchain_observers.iter() {
      let obs = &*obs_box;
      obs.start();
    }
    Ok(())
  }
  pub fn show_balance(&self) {}
}

#[cfg(test)]
mod tests {
  fn run_tests() {
    pringln!("not implemented yet!")
  }
}
