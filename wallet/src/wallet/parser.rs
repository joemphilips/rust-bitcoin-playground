use config;
use std::collections::HashMap;
use config::ConfigError;

pub fn parse_config<'a>(path: &'a str) -> Result<(HashMap<String, String>), ConfigError> {
  let mut settings = config::Config::default();
  settings
    .merge(config::File::with_name(path))
    .unwrap()
    .merge(config::Environment::with_prefix("RUSTY_WALLET"))
    .unwrap();
  settings.try_into::<HashMap<String, String>>()
}
