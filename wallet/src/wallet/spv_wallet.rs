#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;
use std::sync::mpsc;

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey, ExtendedPrivKey};
use rand::random;
use secp256k1::Secp256k1;

use wallet::txbuilder::{Builder, BuilderCommand};
use wallet::blockchain_observer::BlockchainObserver;

use WALLET_CONFIG;

fn init(network: Network) -> Result<ExtendedPrivKey> {
  let secp = Secp256k1::new();
  let seed: u8 = random();
  let sk = ExtendedPrivKey::new_master(&secp, network, &[seed])?;
  Ok(sk)
}

/// struct responsible for keychain info, balances and covert those formats.
pub struct Wallet<'a> {
  blockchain_observers: BlockchainObserver<'a>,
  txbuilder: Builder,
}


impl<'a> Wallet<'a> {
  pub fn new() -> Self {
    Wallet {
      blockchain_observers: BlockchainObserver::new().unwrap(),
      txbuilder: Builder {},
    }
  }

  pub fn start(&self) -> Result<mpsc::Receiver<String>> {
    let recv = self.blockchain_observers.sync_and_listen()?;
    loop {
      let msg = recv.recv()?;
      println!("{:?}", msg)
    }
  }
  pub fn show_balance(&self) {}
}

#[cfg(test)]
mod tests {
  fn run_tests() {
    pringln!("not implemented yet!")
  }
}
