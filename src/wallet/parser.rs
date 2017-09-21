use config;
use std::collections::HashMap;
use std::io;
use config::ConfigError;

pub fn parse_config<'a>(path: &'a str) -> Result<(HashMap<String, String>), ConfigError> {
  let mut settings = config::Config::default();
  settings
    .merge(config::File::with_name("walletconf"))
    .unwrap()
    .merge(config::Environment::with_prefix("RUSTY_WALLET"))
    .unwrap();
  settings.deserialize::<HashMap<String, String>>()
}
