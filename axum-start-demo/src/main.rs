use axum::{
    extract::Path,
    extract::{Form, Query},
    http::HeaderMap,
    // handler::Handler,
    http::{StatusCode, Uri},
    routing::get,
    routing::post,
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
mod page;
use page::PageRequest;
mod domain;
use domain::TempUser;
use std::collections::HashMap;

const COOKIE_NAME: &'static str = "token";

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
        .route("/vec", get(resp_vec))
        .route("/resp_status_headers", get(resp_status_headers))
        .route("/resp_str", get(resp_str))
        .route("/resp_text", get(resp_text))
        .route("/resp_status", get(resp_status))
        .route("/handler_cookie", post(handler_cookie))
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
    if id == 0 {
        let result1 = ServiceResult::illegal_argument(id);
        return Json(result1);
    }
    if id == 1 {
        let result1 = ServiceResult::ok(id);
        return Json(result1);
    }
    if id == 2 {
        let result1 = ServiceResult::error(-1);
        return Json(result1);
    }

    let result = ServiceResult::new(200, "okMessage".to_string(), id);

    Json(result)
}
/**
 * 多个path 参数 用元祖方式获取
 */
async fn path_get_req2(Path((id, name)): Path<(i32, String)>) -> Json<ServiceResult<String>> {
    if id == 0 {
        let result1 = ServiceResult::illegal_argument(name);
        return Json(result1);
    }
    if id == 1 {
        let result1 = ServiceResult::ok(name);
        return Json(result1);
    }
    if id == 2 {
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
    let current = args.current.unwrap_or(0);
    info!("current:{}", current);

    let paget_size = args.page_size.unwrap_or(10);

    let result = ServiceResult::new(200, "okMessage".to_string(), paget_size);

    Json(result)
}
/**
 * 表单请求
 */
async fn form_req(Form(frm): Form<TempUser>) -> Json<TempUser> {
    let result = TempUser::new(frm.age, frm.name);

    Json(result)
}
/**
 * 获取请求头
 */
async fn get_headers(headers: HeaderMap) -> Json<HashMap<String, String>> {
    let mut result: HashMap<String, String> = HashMap::new();
    headers.keys().for_each(|key| {
        info!("key:{}", key);
        let value = headers.get(key).unwrap().to_str().unwrap().to_string();
        info!("value:{}", value);
        result.insert(key.clone().to_string(), value);
    });

    Json(result)
}
/**
 * 响应数组
 */
async fn resp_vec(headers: HeaderMap) -> Json<Vec<String>> {
    let mut resp = Vec::new();
    headers.keys().for_each(|key| {
        resp.push(key.clone().to_string());
    });
    Json(resp)
}
/**
 * 响应 字符串切片
 */
async fn resp_str() -> &'static str {
    "str"
}
/**
 * 纯文本
 */
async fn resp_text() -> String {
    "String".to_string()
}
/**
 * 返回单纯的状态码
 */
async fn resp_status() -> StatusCode {
    StatusCode::OK
}
/**
 * 状态码和请求头返回
 */
async fn resp_status_headers() -> (StatusCode, HeaderMap) {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Token-Header", "axum".parse().unwrap());
    headers.insert("X-Custom-Header2", "axum2".parse().unwrap());

    (StatusCode::OK, headers)
}
/**
 * 中间件示例
 */
async fn middleware_example() -> &'static str {
    info!("this is a middleware example");
    "middleware_example"
}
/**
 * 处理 cookie
 */
async fn handler_cookie(headers: HeaderMap) -> ( HeaderMap,&'static str) {
    info!("handler cookie");
    let mut token: Option<String> = None;

    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|cookie| cookie.to_string())
        .unwrap_or("cookie is none".to_string());

    info!("cookies:{}", cookies);

    let cookiess: Vec<&str> = cookies.split(";").collect();

    for cookie in cookiess {
        let kv: Vec<&str> = cookie.split("=").collect();
        let name = kv[0].trim();
        info!("name:{}", name);
        if kv.len() == 2 {
            let value = kv[1].trim();
            info!("value:{}", value);

            if name == COOKIE_NAME && !value.is_empty() {
                token = Some(String::from(value));
                break;
            }
        }
    }

    if token.is_none() {
        return (headers,"not ok");
        // 没有我们需要的cookie
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        format!("{}={}", COOKIE_NAME, token.unwrap())
            .as_str()
            .parse()
            .unwrap(),
    );
    return (headers,"ok");

 } 
