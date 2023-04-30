pub use crate::configs::constants;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResult<T> {
    pub status: usize,
    pub messsage: String,
    pub data: T,
    pub attache: String,
    pub response_time: String,
}

impl<T> ServiceResult<T> {
    pub fn new(status: usize, message: String, data: T) -> ServiceResult<T> {
        let now: DateTime<Local> = Local::now();
        let str_time = now.format("%Y-%m-%d %H:%M:%S.%s").to_string();
        ServiceResult {
            status,
            messsage: message,
            data: data,
            attache: "".to_string(),
            response_time: str_time,
        }
    }

    pub fn ok(data: T) -> ServiceResult<T> {
        let now: DateTime<Local> = Local::now();
        let str_time = now.format("%Y-%m-%d %H:%M:%S.%s").to_string();
        ServiceResult {
            status:constants::ILLEGAL_ARGUMENT_CODE,
            messsage: constants::ILLEGAL_ARGUMENT_MSG.to_string(),
            data: data,
            attache: "".to_string(),
            response_time: str_time,
        }
    }

        pub fn error(data: T) -> ServiceResult<T> {
        let now: DateTime<Local> = Local::now();
        let str_time = now.format("%Y-%m-%d %H:%M:%S.%s").to_string();
        ServiceResult {
            status:constants::SYS_ERROR_CODE,
            messsage: constants::SYS_ERROR__MSG.to_string(),
            data: data,
            attache: "".to_string(),
            response_time: str_time,
        }
    }


    pub fn illegal_argument(data:T) -> ServiceResult<T> {
        let now: DateTime<Local> = Local::now();
        let str_time: String = now.format("%Y-%m-%d %H:%M:%S.%s").to_string();
        ServiceResult {
            status: constants::ILLEGAL_ARGUMENT_CODE,
            messsage: constants::ILLEGAL_ARGUMENT_MSG.to_string(),
            data: data,
            attache: "".to_string(),
            response_time: str_time,
        }
    }
}
