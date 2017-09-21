#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;

use bitcoin::blockdata::blockchain::Blockchain;
use bitcoin::network::constants::Network;
use bitcoin::network::listener::Listener;
use bitcoin::util::bip32;

pub struct LocalBlockchainSupervisor {
  blockchain: Blockchain,
}

impl Listener for LocalBlockchainSupervisor {
  fn peer(&self) -> &str {
    "127.0.0.1"
  }

  fn port(&self) -> u16 {
    18332
  }

  fn network(&self) -> Network {
    Network::Testnet
  }
}

impl LocalBlockchainSupervisor {
  pub fn new(config: HashMap<String, String>) -> Self {
    let network;
    if config.get("network").unwrap() == "testnet" {
      network = Network::Testnet;
    } else if config.get("network").unwrap() == "bitcoin" {
      network = Network::Bitcoin;
    } else {
      panic!("network in config is not good!")
    }
    LocalBlockchainSupervisor { blockchain: Blockchain::new(network) }
  }
}
