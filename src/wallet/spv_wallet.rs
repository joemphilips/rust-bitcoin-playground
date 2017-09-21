#!/usr/bin/env/ run-cargo-script

use error::*;
use bitcoin::blockdata::blockchain::Blockchain;
use bitcoin::network::constants::Network;
use bitcoin::network::listener::Listener;

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
  pub fn new(config: Network) -> Self {
    LocalBlockchainSupervisor { blockchain: Blockchain::new(config) }
  }
}

/*
pub fn server_start() -> Result<()> {
  let supervisor = LocalBlockchainSupervisor { blockchain: Blockchain::new(Network::Testnet) };
  let (recv, sock) = supervisor.start()?;
  println!("{:?}", recv);
  Ok(())
}
*/
