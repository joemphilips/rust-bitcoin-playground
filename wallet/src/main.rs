#![recursion_limit = "1024"]
#![feature(box_patterns)]

extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate config;
#[feature(rand)]
extern crate rand;
extern crate secp256k1;
extern crate bitcoin;
#[macro_use]
extern crate lazy_static;

// std library
use std::collections::HashMap;

// internal
mod wallet;
use wallet::spv_wallet::Wallet;
use wallet::parser::parse_config;

use clap::App;

mod error {
  error_chain!{
    foreign_links {
      Clap(::clap::Error);
      Bitcoin(::bitcoin::util::Error);
      Bip32(::bitcoin::util::bip32::Error);
      Config(::config::ConfigError);
      Addr(::std::net::AddrParseError);
      MPSC(::std::sync::mpsc::RecvError);
      IO(::std::io::Error);
    }
  }
}

lazy_static! {
  static ref WALLET_CONFIG: HashMap<String, String> = parse_config("walletconf.toml").unwrap();
}


use error::*;
fn run<I, T>(args: I) -> Result<()>
where
  I: IntoIterator<Item = T>,
  T: Into<std::ffi::OsString> + Clone,
{
  env_logger::init();
  warn!("WALLET_CONFIG is {:?}", *WALLET_CONFIG);
  /* let yml = load_yaml!("cli_option.yaml");
  let app = App::from_yaml(yml);
  let matches = app.get_matches_from_safe(args)?; */
  let spv = Wallet::new();
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
