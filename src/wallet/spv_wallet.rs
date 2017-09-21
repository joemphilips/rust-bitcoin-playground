#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;

use bitcoin::blockdata::blockchain::Blockchain;
use bitcoin::network::constants::Network;
use bitcoin::network::listener::Listener;
use bitcoin::util::bip32;

pub struct LocalBlockchainSupervisor {
  blockchain: Blockchain,
  network: Network,
}

impl Listener for LocalBlockchainSupervisor {
  fn peer(&self) -> &str {
    "127.0.0.1"
  }

  fn port(&self) -> u16 {
    18332
  }

  fn network(&self) -> Network {
    self.network
  }
}

fn get_network(network: &str) -> Option<Network> {
  if network == "testnet" {
    return Some(Network::Testnet);
  } else if network == "bitcoin" {
    return Some(Network::Bitcoin);
  } else {
    return None;
  }
}

impl LocalBlockchainSupervisor {
  pub fn new(config: HashMap<String, String>) -> Self {
    let network = get_network(config.get("network").unwrap()).unwrap();
    LocalBlockchainSupervisor {
      blockchain: Blockchain::new(network),
      network: network,
    }
  }
}
