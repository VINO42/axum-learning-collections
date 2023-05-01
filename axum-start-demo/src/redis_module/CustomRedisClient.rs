
use std::sync::Arc;


 
#[derive(Clone)]
pub struct DatabaseClient {
    pub dsn: String,
}
pub struct RedisClientCfg {
    pub host: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseClient,
    pub rdb: Arc<RedisClientCfg>,
}

 


