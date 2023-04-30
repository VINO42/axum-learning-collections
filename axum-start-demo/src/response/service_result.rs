pub use crate::configs::constants;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResult<'a, T> {
    pub status: usize,
    pub messsage: &'a str,
    pub data: T,
    pub attache: &'a str,
    pub response_time: String,
}

impl<'a, T> ServiceResult<'a, T> {
    pub fn new(status: usize, message: &str, data: T) -> ServiceResult<T> {
        let now: DateTime<Local> = Local::now();
        let str_time = now.format("%Y-%m-%d %H:%M:%S.%s").to_string();
        ServiceResult {
            status,
            messsage: message,
            data: data,
            attache: "",
            response_time: str_time,
        }
    }

    pub fn illegal_argument() -> ServiceResult<'a, ()> {
        let now: DateTime<Local> = Local::now();
        let str_time: String = now.format("%Y-%m-%d %H:%M:%S.%s").to_string();
        ServiceResult {
            status: constants::ILLEGAL_ARGUMENT_CODE,
            messsage: constants::ILLEGAL_ARGUMENT_MSG,
            data: (),
            attache: "",
            response_time: str_time,
        }
    }
}
