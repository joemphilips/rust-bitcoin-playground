
use bitcoin::network::listener::Listener;
// use bitcoin::network::socket::Socket;
use bitcoin::network::constants::Network;
use std::fmt::Debug;

pub struct SPVListener {
  config: String,
}

impl Listener for SPVListener {
  fn peer(&self) -> &str {
    return "192.168.1.1";
  }

  fn port(&self) -> u16 {
    8332u16
  }

  fn network(&self) -> Network {
    Network::Testnet
  }
}

impl SPVListener {
  pub fn new<T: Debug>(argmatches: T) -> Self {
    println!("{:?}", argmatches);
    SPVListener { config: "hoge".to_owned() }
  }

  pub fn run(&self) {
    println!("running!");
    println!("config is {}", &self.config)
  }
}
