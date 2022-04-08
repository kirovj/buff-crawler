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
        _ => Json(json!({ "error": "typo error" })),
    }
}

async fn get_items_by_name(payload: Search) -> Json<Value> {
    let target = Target::from(payload.target.as_str());
    let db = get_dbconnection(target).lock().unwrap();
    let data = db.find_items_by_name(payload.name).unwrap();
    Json(serde_json::to_value(data).unwrap())
}

async fn get_price_by_item_id(payload: Search) -> Json<Value> {
    let target = Target::from(payload.target.as_str());
    let db = get_dbconnection(target).lock().unwrap();
    let data = db.find_price_by_item_id(payload.item_id).unwrap();
    Json(serde_json::to_value(data).unwrap())
}

fn get_db_file(target: Target) -> &'static str {
    match target {
        Target::Buff => utils::DB_FILE_BUFF,
        Target::Yyyp => utils::DB_FILE_YYYP,
    }
}

fn get_dbconnection(target: Target) -> &'static Mutex<DbHelper> {
    let db_file = get_db_file(target);
    // 使用MaybeUninit延迟初始化
    static mut DB_CONNECTION: MaybeUninit<Mutex<DbHelper>> = MaybeUninit::uninit();
    // Once带锁保证只进行一次初始化
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        println!("Initializing DB connection to {}", db_file);
        DB_CONNECTION
            .as_mut_ptr()
            .write(Mutex::new(DbHelper::new(db_file)));
    });
    unsafe { &*DB_CONNECTION.as_ptr() }
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
