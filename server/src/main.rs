fn main() {
    let c = crawler::build_crawler("buff", "buff.db").unwrap();
    c.alert("test");
}
