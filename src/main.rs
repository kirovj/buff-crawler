mod crawl;
mod db;
mod http;
mod item;
mod utils;

use crate::{crawl::Target, db::DbHelper};
use axum::{
    extract::Json,
    http::StatusCode,
    response::Html,
    routing::{get, get_service, post},
    Router,
};
use item::{Item, PriceInfo};
use rusqlite::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::{Mutex, Once},
};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct Request {
    target: String,
    name: String,
    item_id: u32,
}

#[derive(Serialize)]
struct Response<T> {
    status: u8,
    message: String,
    data: Option<Vec<T>>,
}

impl<T: Serialize> Response<T> {
    fn ok(data: Vec<T>) -> Self {
        Response {
            status: 0,
            message: String::from("ok"),
            data: Some(data),
        }
    }

    fn fail(message: String) -> Self {
        Response {
            status: 1,
            message,
            data: None,
        }
    }

    fn new(result: Result<Vec<T>, Error>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(e) => Self::fail(e.to_string()),
        }
    }
}

// index.html
async fn index() -> Html<&'static str> {
    Html(get_index_html())
}

async fn get_items_by_name(Json(request): Json<Request>) -> Json<Response<Item>> {
    let db = get_db_helper(request.target).lock().unwrap();
    let data = db.find_items_by_name(request.name);
    Json(Response::new(data))
}

async fn get_price_by_item_id(Json(request): Json<Request>) -> Json<Response<PriceInfo>> {
    let db = get_db_helper(request.target).lock().unwrap();
    let data = db.find_price_by_item_id(request.item_id);
    Json(Response::new(data))
}

fn get_index_html() -> &'static str {
    include_str!("../index.html")
}

fn get_dbconnection_container() -> &'static HashMap<Target, Mutex<DbHelper>> {
    // 使用 MaybeUninit 延迟初始化
    static mut DB_CONNECTION_CONTAINER: MaybeUninit<HashMap<Target, Mutex<DbHelper>>> =
        MaybeUninit::uninit();
    // Once 带锁保证只进行一次初始化
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        println!("Initializing DB connection container");
        let mut map = HashMap::new();
        map.insert(Target::Buff, Mutex::new(DbHelper::new(utils::DB_FILE_BUFF)));
        map.insert(Target::Yyyp, Mutex::new(DbHelper::new(utils::DB_FILE_YYYP)));
        DB_CONNECTION_CONTAINER.as_mut_ptr().write(map);
    });
    unsafe { &*DB_CONNECTION_CONTAINER.as_ptr() }
}

fn get_db_helper(target: String) -> &'static Mutex<DbHelper> {
    let target = Target::from(target.as_str());
    let container = get_dbconnection_container();
    container.get(&target).unwrap()
}

#[tokio::main]
async fn main() {
    // let db_buff = get_dbconnection(Target::Yyyp);
    // let _ = tokio::spawn(crawl(Target::Buff, "./data/buff.db"));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/index", get(index))
        .route("/find_item", post(get_items_by_name))
        .route("/find_price", post(get_price_by_item_id))
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        );

    println!("server start");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
