use bitcoin::network::listener::Listener;
use bitcoin::network::constants::Network;
use std::collections::HashMap;
use std::net::SocketAddr;
use error::*;

pub struct BlockchainObserver<'a> {
  peer: &'a str,
  port: u16,
  network: Network,
}

impl<'a> BlockchainObserver<'a> {
  pub fn new(config: &'a HashMap<String, String>) -> Result<Self> {
    let network = get_network(config.get("network").unwrap())?;
    let port = config
      .get("rpcaddress")
      .unwrap()
      .to_owned()
      .parse::<SocketAddr>()?
      .port();
    let peer = config.get("rpcaddress").unwrap().split(":").nth(0).unwrap();
    Ok(BlockchainObserver {
      peer: peer,
      port: port,
      network: network,
    })
  }

  pub fn notify() {
    println!("not implemented yet!")
  }
}

impl<'a> Listener for BlockchainObserver<'a> {
  fn peer(&self) -> &str {
    self.peer
  }

  fn port(&self) -> u16 {
    self.port
  }

  fn network(&self) -> Network {
    self.network
  }
}


fn get_network(network: &str) -> Result<Network> {
  match network {
    "testnet" => return Ok(Network::Testnet),
    "bitcoin" => return Ok(Network::Bitcoin),
    _ => unreachable!(),
  }
}
