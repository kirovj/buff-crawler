#[macro_use]
extern crate lazy_static;

mod crawler;
mod constant;
mod db;
mod http;
mod item;

use constant::DB_FILE;
use crawler::Crawler;


fn main() {
    let crawler = Crawler::new(DB_FILE);
    crawler.run_without_login();
}
