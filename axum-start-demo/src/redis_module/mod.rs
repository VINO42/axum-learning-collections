pub mod CustomRedisClient;

pub use CustomRedisClient::DatabaseClient;
pub use CustomRedisClient::AppState;
pub use CustomRedisClient::RedisClientCfg;

use serde::de::DeserializeOwned;
use redis::{AsyncCommands,Client,Connection};
use serde::{ Serialize};
use serde_json::{from_str, json};
const REDIS_DSN: &str = "redis://127.0.0.1:6379/";




pub async fn client() -> Client {
    let client = Client::open( REDIS_DSN).unwrap();

    client
 }

 pub async fn   set<T:DeserializeOwned+Serialize>(key: &str, value: T) -> Result<&'static str, String> {
    
     let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let json_value=serde_json::to_string(&value).unwrap();
     let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    conn.set(key,json_value )
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully set")
}

pub async fn get ( key: &str) -> Result<String, String >{
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;

    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    let value = conn.get(key).await.map_err(|err| err.to_string())?;

    // let result=serde_json::from_str(&value).unwrap();

    Ok(value)
}


/**
 * ttl
 */
pub async fn expire(key: &str, time: usize) -> Result<&'static str, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    conn.expire(key, time).await.map_err(|err| err.to_string())?;
    Ok("OK")

}