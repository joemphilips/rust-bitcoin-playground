#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

mod server;

extern crate bitcoin;

use clap::{App, AppSettings, Arg};

mod error {
  error_chain!{
    foreign_links {
      Clap(::clap::Error);
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
  let datadir_path = &std::env::home_dir().unwrap();
  let default_config_path_buf = datadir_path.join("rustwallet.conf");
  let default_config_path_str = default_config_path_buf.to_str().unwrap();
  let default_wallet_path_buf = datadir_path.join("wallet.dat");
  let default_wallet_path_str = default_wallet_path_buf.to_str().unwrap();

  // main parse logic
  let yml = load_yaml!("walletconf.yaml");
  let app = App::from_yaml(yml);
  let matches = app
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(
      env!("CARGO_PKG_DESCRIPTION"),
      " - ",
      env!("CARGO_PKG_HOMEPAGE")
    ))
    .setting(AppSettings::ColoredHelp)
    .get_matches_from_safe(args)?;
  println!("{:?}", matches);
  while let Some(o) = matches.value_of("config") {
    let spv = server::spv_listener::SPVListener::new(o);
    spv.run();
  }

  Ok(())
}

fn main() {
  if let Err(ref e) = run(std::env::args()) {

    println!("error: {}", e);

    for e in e.iter().skip(1) {
      println!("caused by: {}", e);
    }

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
