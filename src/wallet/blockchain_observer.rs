use bitcoin::network::listener::Listener;
use bitcoin::network::constants::Network;
use bitcoin::network::serialize::{RawEncoder, RawDecoder};
use bitcoin::blockdata::blockchain::Blockchain;
use bitcoin::network::encodable::{ConsensusEncodable, ConsensusDecodable};


use std::collections::BinaryHeap; // for state machine
use std::cmp::Ordering;
use std::default::Default;
use std::net::SocketAddr;
use std::thread;
use std::sync::mpsc;
use std::fs;
use std::sync::{RwLock, Arc};
use std::io::{BufReader, Read, BufWriter, Write};
use error::*;
use WALLET_CONFIG;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
  SyncBlock = 1, // 1st priority
  NotifyReceive = 2, // 2nd
  GetUTXO = 3, // 3rd
  SaveToDisk = 4, // 4th
}


pub struct BlockchainObserver<'a> {
  peer: &'a str,
  port: u16,
  network: Network,
  blockchain: Arc<RwLock<Blockchain>>,
}

impl<'a> BlockchainObserver<'a> {
  pub fn new() -> Result<Self> {

    let network = get_network(WALLET_CONFIG.get("network").unwrap())?;

    let port = WALLET_CONFIG
      .get("rpcaddress")
      .unwrap()
      .to_owned()
      .parse::<SocketAddr>()?
      .port();

    let peer = WALLET_CONFIG
      .get("rpcaddress")
      .unwrap()
      .split(":")
      .nth(0)
      .unwrap();

    // setup blockchain from file
    let fp = WALLET_CONFIG.get("blockchain_db").unwrap();
    let mut fh = fs::OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(fp)?;
    let mut fd = RawDecoder::new(BufReader::new(fh));
    let blockchain = match ConsensusDecodable::consensus_decode(&mut fd) {
      Ok(blockchain) => blockchain,
      Err(e) => {
        debug!("Failed to load blockchain: {:?}, starting from genesis.", e);
        Blockchain::new(network)
      }
    };

    let blockchain_p = Arc::new(RwLock::new(blockchain));

    Ok(BlockchainObserver {
      peer: peer,
      port: port,
      network: network,
      blockchain: blockchain_p,
    })
  }

  pub fn sync_and_listen(&self) -> Result<mpsc::Receiver<String>> {
    use bitcoin::network::listener::Listener;
    use bitcoin::network::message::SocketResponse::*;
    use bitcoin::network::message::NetworkMessage::*;
    use bitcoin::network::message_blockdata::{GetBlocksMessage, GetHeadersMessage};
    let (tx, rs) = mpsc::channel();
    let tx_for_thread = tx.clone();
    let (mut p2prx, mut p2psock) = self.start()?;
    let mut state_queue: BinaryHeap<Action> = BinaryHeap::new(); // priority-aware state queue


    let getheaders_msg = GetHeadersMessage::new(
      self.blockchain.read().unwrap().locator_hashes(),
      Default::default(),
    );
    thread::spawn(move || {
      // initial sync
      p2psock.send_message(GetHeaders(getheaders_msg));

      loop {
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
      }
    });
    Ok(rs)
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

#[cfg(test)]
mod tests {
  use super::Action::*;
  #[test]
  fn action_comp_test() {}
}
