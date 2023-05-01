use axum::{
    http::HeaderMap,
    extract::Path,
    // handler::Handler,
    http::{StatusCode, Uri},
    routing::get,
    routing::post,
    Json,
    Router,
    extract::{
        Query,Form
    }
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
mod page;
use page::PageRequest;
mod domain;
use domain::TempUser;
use std::collections::HashMap;

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
        .route("/path/:id/:name", get(path_get_req2))
        .route("/page", get(page_get))
        .route("/form", post(form_req))
        .route("/headers", get(get_headers))
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
/**
 * 多个path 参数 用元祖方式获取
 */
async fn path_get_req2(Path((id,name)): Path<(i32, String )>) -> Json<ServiceResult<String>> {
    if  id == 0  {
     let result1 = ServiceResult::illegal_argument(name);
      return Json(result1);
    }
    if  id == 1  {
     let result1 = ServiceResult::ok(name);
      return Json(result1);
    }
    if  id == 2  {
     let result1 = ServiceResult::error("".to_string());
      return Json(result1);
    }
 
     let result = ServiceResult::new(200, "okMessage".to_string(), name);
 
     Json(result)
 }
 /**
  * get url 请求
  */
  async fn page_get(Query(args): Query<PageRequest>) -> Json<ServiceResult<i32>> {

    let current= args.current.unwrap_or(0);
    info!("current:{}",current);

   let paget_size=args.page_size.unwrap_or(10);

   let result = ServiceResult::new(200, "okMessage".to_string(), paget_size);
 
     Json(result)

  }
 /**
  * 表单请求
  */
  async fn form_req(Form(frm): Form<TempUser>) -> Json<TempUser> {
 
 
   let result = TempUser::new(
    frm.age, frm.name);
 
     Json(result)

  }
  async fn get_headers(headers:HeaderMap) ->Json<HashMap<String,String>> {
    let mut  result:HashMap<String,String>=HashMap::new();
    headers.keys().for_each(|key|{
     info!("key:{}",key);
   let value=  headers.get(key).unwrap().to_str().unwrap().to_string();
     info!("value:{}",value);
     result.insert(key.clone().to_string(), value);
    }   );
  
        Json(result)
  }