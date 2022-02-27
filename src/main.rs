#[macro_use]
extern crate lazy_static;
extern crate clap;

mod constant;
mod crawler;
mod db;
mod http;
mod item;

use clap::{crate_version, App, Arg};

fn main() {
    // let help_desc = r#"aaaaaaaaa"#;
    let _ = App::new("CS:GO item price crawler")
        .version(crate_version!())
        .author("Kirovj. <wuyitingtz3@gmail.com>")
        .about("Please don't use it illegally, I don't take any responsibility.")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .help("crawl target")
                // .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db")
                .short("d")
                .long("database")
                .help("database file name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("proxy")
                .short("p")
                .long("proxy")
                .help("Use proxy or not")
                .takes_value(false),
        )
        .get_matches();
    match crawler::build_crawler("buff", constant::DB_FILE) {
        Some(crawler) => crawler.run(),
        _ => println!("so such target"),
    }
}
