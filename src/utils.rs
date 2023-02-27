use chrono::Local;

// constants
pub const UA: &str = 
    "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36";
pub const DEFAULT: &str = "无";
pub const API_BUFF: &str = 
    "https://buff.163.com/api/market/goods?game=csgo&page_num=1&page_size=120&use_suggestion=0&trigger=undefined_trigger&_=";
pub const API_YYYP: &str = "https://api.youpin898.com/api/v2/commodity/template/GetCsGoNewOnShelf?Count=50";
pub const DB_FILE_BUFF: &str = "./data/buff.db";
pub const DB_FILE_YYYP: &str = "./data/yyyp.db";
pub const HTML: &str = include_str!("../index.html");
pub const ALERT: &str = include_str!("../alert.txt");

pub fn alert(message: &str) -> () {
    ureq::post(ALERT).send_form(&[("data", message)]).unwrap();
}

// Retain one decimal place
pub fn round(f: f32) -> f32 {
    (f * 10f32).round() / 10f32
}

pub fn current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn current_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn current_timestamp() -> String {
    Local::now().timestamp_millis().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(2.0f32, round("1.919".parse::<f32>().unwrap()));
    }
}
