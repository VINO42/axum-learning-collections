use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Server {
  pub addr: String,
}