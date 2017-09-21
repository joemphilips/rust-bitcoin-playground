#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

mod wallet;
use wallet::spv_wallet::LocalBlockchainSupervisor;
use wallet::parser::parse_config;

extern crate bitcoin;
use bitcoin::network::constants::Network;
use bitcoin::network::listener::Listener;

use clap::{App, AppSettings};

mod error {
  error_chain!{
    foreign_links {
      Clap(::clap::Error);
      Bitcoin(::bitcoin::util::Error);
    }
  }
}

use error::*;
fn run<I, T>(args: I) -> Result<()>
where
  I: IntoIterator<Item = T>,
  T: Into<std::ffi::OsString> + Clone,
{
  // create default path for config and wallet file
  /* let datadir_path = &std::env::home_dir().unwrap();
  let default_config_path_buf = datadir_path.join("rustwallet.conf");
  let default_config_path_str = default_config_path_buf.to_str().unwrap();
  let default_wallet_path_buf = datadir_path.join("wallet.dat");
  let default_wallet_path_str = default_wallet_path_buf.to_str().unwrap(); */

  // main parse logic
  let yml = load_yaml!("cli_option.yaml");
  let app = App::from_yaml(yml);
  let matches = app.get_matches_from_safe(args)?;
  if let Some(c) = matches.value_of("config") {
    println!("going to parse config file {:?}", c);
    let _ = parse_config(c);
  }
  let spv = LocalBlockchainSupervisor::new(Network::Testnet);
  spv.start()?;
  Ok(())
}

fn main() {
  if let Err(ref e) = run(std::env::args()) {

    println!("error: {}", e);

    for e in e.iter().skip(1) {
      println!("caused by: {}", e);
    }
    // run with `RUST_BACKTRACE=1` if you one to see this.
    if let Some(backtrace) = e.backtrace() {
      println!("backtrace: {:?}", backtrace);
    }

    std::process::exit(1);
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn no_op_test() {
    assert!(::run(&["hello"]).is_ok())
  }
  #[test]
  #[should_panic]
  fn should_panic_test() {
    assert_eq!("Hello", "World")
  }
}
