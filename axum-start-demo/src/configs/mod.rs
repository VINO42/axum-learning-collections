pub mod constants;
mod application_config;
mod server;
extern crate config;

use self::config::*;



pub fn get_config() -> Config {
  let mut c = Config::default();
  c.merge(File::new(constants::CONFIG_PATH, FileFormat::Yaml))
    .unwrap();
  c
}

pub use application_config::ApplicationConfig;
pub use server::Server;
  