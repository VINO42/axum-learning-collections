extern crate config;
use super::server::Server;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ApplicationConfig {
  pub server: Server
}