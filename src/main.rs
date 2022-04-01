#[macro_use]
extern crate lazy_static;

mod crawl;
mod db;
mod http;
mod item;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{crawl::Target, db::DbHelper};

async fn crawl(target: Target, db_file: &str) {
    let c = crawl::build_crawler(target, db_file).unwrap();
    c.run();
}

async fn find_data(db: &DbHelper) -> String {
    todo!()
}

#[tokio::main]
async fn main() {
    let db_buff = DbHelper::new(utils::DB_FILE_BUFF);
    let db_yyyp = DbHelper::new(utils::DB_FILE_YYYP);

    // let _ = tokio::spawn(crawl(Target::Buff, "./data/buff.db"));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/find", post(find_data));

    println!("server start");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
