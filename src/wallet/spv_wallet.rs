#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;
use std::thread;

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
    use bitcoin::network::message::NetworkMessage::*;
    for obs in self.blockchain_observers.iter() {
      let (mut recv, mut sock) = obs.start()?;

      thread::spawn(move || loop {
        match sock.receive_message() {
          Err(e) => println!("err! {:?}", e),
          Ok(o) => {
            match o {
              Version(v) => println!("version is {:?}", v),
              Verack => println!("received version ack"),
              Addr(addr_vec) => println!("received addressess are {:?}", addr_vec),
              Inv(inv_vec) => println!("received Invs are {:?}", inv_vec),
              GetData(data_vec) => println!("received data are {:?}", data_vec),
              NotFound(data_vec) => println!("received Inv notfound messages are {:?}", data_vec),
              GetBlocks(msg) => println!("received get block message is {:?}", msg),
              Ping(ver) | Pong(ver) => println!("ping pong {}", ver),
              x => println!("received {:?}", x),
            }
          }
        }
      });
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
