mod crawl;
mod db;
mod http;
mod item;
mod utils;

use crate::{
    crawl::{BuffCrawler, Crawl, Target, YyypCrawler},
    db::DbHelper,
};
use axum::{
    extract::Json,
    http::StatusCode,
    response::Html,
    routing::{get, get_service, post},
    Router,
};
use chrono::{Local, Timelike};
use item::{Item, PriceInfo};
use serde_json::{Error as JsonError, Value};
use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::{Mutex, Once},
};
use tower_http::services::ServeDir;

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

fn get_db_helper_by_string(target: String) -> &'static Mutex<DbHelper> {
    get_db_helper(Target::from(target.as_str()))
}

fn get_db_helper(target: Target) -> &'static Mutex<DbHelper> {
    let container = get_dbconnection_container();
    container.get(&target).unwrap()
}

fn get_watch_list() -> &'static Vec<Item> {
    static mut WATCH_LIST: MaybeUninit<Vec<Item>> = MaybeUninit::uninit();
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        println!("Initializing watch list");
        let watch_json = std::fs::read_to_string("watch.json").unwrap();
        let watch_list_value: Result<Value, JsonError> = serde_json::from_str(watch_json.as_str());
        match watch_list_value {
            Ok(values) => {
                let watch_list: Vec<Item> = values
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter_map(|value| {
                        let target = value["target"].as_str().unwrap().to_string();
                        let name = value["name"].as_str().unwrap().to_string();
                        let class = value["class"].as_str().unwrap().to_string();
                        let typo = value["typo"].as_str().unwrap().to_string();
                        let ware = value["ware"].as_str().unwrap().to_string();
                        let stat_trak = value["stat_trak"].as_bool().unwrap();
                        let db = get_db_helper_by_string(target).lock().unwrap();
                        let item = db.find_item(name, class, typo, ware, stat_trak);
                        item.ok()
                    })
                    .collect();
                WATCH_LIST.as_mut_ptr().write(watch_list);
            }
            _ => println!("read watch list failed"),
        }
    });
    unsafe { &*WATCH_LIST.as_ptr() }
}

// index.html
async fn index() -> Html<&'static str> {
    Html(utils::HTML)
}

async fn find_watch_list() -> Json<http::Response<Item>> {
    Json(http::Response::ok(get_watch_list().to_vec()))
}

async fn find_items_by_name(Json(request): Json<http::Request>) -> Json<http::Response<Item>> {
    let db = get_db_helper_by_string(request.target).lock().unwrap();
    let data = db.find_items_by_name(request.name);
    Json(http::Response::new(data))
}

async fn find_price_by_item_id(
    Json(request): Json<http::Request>,
) -> Json<http::Response<PriceInfo>> {
    let db = get_db_helper_by_string(request.target).lock().unwrap();
    let data = db.find_price_by_item_id(request.item_id);
    Json(http::Response::new(data))
}

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        let db_helper = DbHelper::new(utils::DB_FILE_BUFF);
        let crawler = BuffCrawler::new(db_helper);
        crawler.run();
    });

    let _ = tokio::spawn(async {
        loop {
            if Local::now().hour() == 23 {
                let db_helper = DbHelper::new(utils::DB_FILE_YYYP);
                let crawler = YyypCrawler::new(db_helper);
                crawler.run();
            }
            std::thread::sleep(std::time::Duration::from_secs(600));
        }
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/find_watch_list", get(find_watch_list))
        .route("/find_item", post(find_items_by_name))
        .route("/find_price", post(find_price_by_item_id))
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
