
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct PageRequest {
    pub current:  Option<i32>,
    pub page_size:Option<i32>,
}