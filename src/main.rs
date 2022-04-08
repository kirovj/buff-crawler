mod crawl;
mod db;
mod http;
mod item;
mod utils;

use crate::{crawl::Target, db::DbHelper};
use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::{Mutex, Once},
};

#[derive(Deserialize)]
struct Search {
    target: String,
    typo: String,
    name: String,
    item_id: u32,
}

async fn find(Json(payload): Json<Search>) -> Json<Value> {
    match payload.typo.as_str() {
        "item" => get_items_by_name(payload).await,
        "price" => get_price_by_item_id(payload).await,
        _ => Json(json!({ "error": "typo error, use price or item" })),
    }
}

async fn get_items_by_name(payload: Search) -> Json<Value> {
    let target = Target::from(payload.target.as_str());
    let db = get_db_helper(target).lock().unwrap();
    let data = db.find_items_by_name(payload.name).unwrap();
    Json(serde_json::to_value(data).unwrap())
}

async fn get_price_by_item_id(payload: Search) -> Json<Value> {
    let target = Target::from(payload.target.as_str());
    let db = get_db_helper(target).lock().unwrap();
    let data = db.find_price_by_item_id(payload.item_id).unwrap();
    Json(serde_json::to_value(data).unwrap())
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

fn get_db_helper(target: Target) -> &'static Mutex<DbHelper> {
    let container = get_dbconnection_container();
    container.get(&target).unwrap()
}

#[tokio::main]
async fn main() {
    // let db_buff = get_dbconnection(Target::Yyyp);
    // let _ = tokio::spawn(crawl(Target::Buff, "./data/buff.db"));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/find", post(find));

    println!("server start");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
