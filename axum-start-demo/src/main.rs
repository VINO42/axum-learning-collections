use axum::{
    extract::Path,
    // handler::Handler,
    http::{StatusCode, Uri},
    routing::get,
    Json,
    Router,
};
extern crate config;
extern crate lazy_static;
extern crate serde;
// use chrono::prelude::*;
use log::info;
use log4rs;
mod response;
use response::ServiceResult;
mod configs;
use self::config::*;
use configs::ApplicationConfig;

#[tokio::main]
async fn main() {
    log4rs::init_file("src/resource/log.yml", Default::default()).unwrap();

    let cfg: Config = configs::get_config();

    let configs: ApplicationConfig = cfg.try_into().unwrap();
    let addr = configs.server.addr;
    info!("Starting Server Address : {}", addr);

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/path/:id", get(path_get_req))
        .route("/test", get(test_get));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> String {
    info!("Hello world!");

    String::from("Hello world!")
}

async fn test_get() -> String {
    info!("Test :");

    String::from("Test")
}

async fn global_fall_back(url: Uri) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, "未知路由：".to_string())
}

/**
 * 从path中获取参数 响应自定义json
 */
async fn path_get_req(Path(id): Path<i32>) -> Json<ServiceResult<i32>> {
   if  id == 0  {
    let result1 = ServiceResult::illegal_argument(id);
     return Json(result1);
   }
   if  id == 1  {
    let result1 = ServiceResult::ok(id);
     return Json(result1);
   }
   if  id == 2  {
    let result1 = ServiceResult::error(-1);
     return Json(result1);
   }

    let result = ServiceResult::new(200, "okMessage".to_string(), id);

    Json(result)
}
