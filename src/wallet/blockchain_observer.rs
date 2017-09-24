use bitcoin::network::listener::Listener;
use bitcoin::network::address::Address as P2PAddress;
use bitcoin::network::constants::Network;
use bitcoin::blockdata::blockchain::Blockchain;
use std::collections::HashMap;

pub struct BlockchainObserver {
  blockchain: Blockchain,
  network: Network,
  address: P2PAddress,
}

impl BlockchainObserver {
  pub fn new(config: HashMap<String, String>) -> Self {
    let network = get_network(config.get("network").unwrap()).unwrap();
    let address = get_address(config.get("address").unwrap());
    BlockchainObserver {
      blockchain: Blockchain::new(network),
      network: network,
      address: address,
    }
  }

  pub fn notify() {
    println!("not implemented yet!")
  }
}

impl Listener for BlockchainObserver {
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

// convert address to consensus_serializable format from string.
fn get_address(address: &str) -> Result<P2PAddress> {
  let address: i32 = address.parse();
  Address
}

fn get_network(network: &str) -> Option<Network> {
  match network {
    "testnet" => return Some(Network::Testnet),
    "bitcoin" => return Some(Network::Bitcoin),
    _ => return None,
  }
}
