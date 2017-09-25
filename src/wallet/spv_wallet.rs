#!/usr/bin/env/ run-cargo-script

use error::*;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

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

  pub fn start(&self) -> Result<mpsc::Receiver<String>> {
    use bitcoin::network::listener::Listener;
    use bitcoin::network::message::SocketResponse::*;
    use bitcoin::network::message::NetworkMessage::*;
    let (tx, rs) = mpsc::channel();
    for obs in self.blockchain_observers.iter() {
      let tx_for_thread = tx.clone();
      let (mut p2prx, mut p2psock) = obs.start()?;

      thread::spawn(move || loop {
        match p2prx.recv().unwrap() {
          ConnectionFailed(e, socket) => {
            println!("err! {:?}", e);
            break;
          }
          MessageReceived(o) => {
            let msg: String = match o {
              Version(v) => {
                p2psock.send_message(Verack);
                format!("Received version is {:?}", v)
              }
              Verack => format!("received version ack"),
              Addr(addr_vec) => format!("received addressess are {:?}", addr_vec),
              Inv(inv_vec) => format!("received Invs are {:?}", inv_vec),
              GetData(data_vec) => format!("received data are {:?}", data_vec),
              NotFound(data_vec) => format!("received Inv notfound messages are {:?}", data_vec),
              GetBlocks(msg) => format!("received get block message is {:?}", msg),
              Ping(ver) | Pong(ver) => format!("ping pong {}", ver),
              x => format!("received {:?}", x),
            };
            tx_for_thread.send(msg).unwrap();
          }
        }
      });
    }
    Ok(rs)
  }
  pub fn show_balance(&self) {}
}

#[cfg(test)]
mod tests {
  fn run_tests() {
    pringln!("not implemented yet!")
  }
}
