#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;

use bitcoin::network::constants::Network;
use bitcoin::util::bip32;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey, ExtendedPrivKey};
use bitcoin::util::bip32::Error;
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
pub struct Wallet {
  blockchain_observers: Vec<Box<BlockchainObserver>>,
  txbuilder: Builder,
}


impl Wallet {
  pub fn new(config: HashMap<String, String>) -> Self {
    Wallet {
      blockchain_observers: vec![],
      txbuilder: Builder {},
    }
  }
  pub fn show_balance(&self) -> () {
    println!("not implemented yet!")
  }
}

#[cfg(test)]
mod tests {
  fn run_tests() {
    pringln!("not implemented yet!")
  }
}
