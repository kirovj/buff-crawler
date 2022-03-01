// constants
pub const UA: &str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36";
pub const API_BUFF :&str = "https://buff.163.com/api/market/goods?game=csgo&page_num=1&page_size=120&use_suggestion=0&trigger=undefined_trigger&_=";
pub const DEFAULT: &str = "无";

// Retain one decimal place
pub fn round(f: f32) -> f32 {
    (f * 10f32).round() / 10f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(2.0f32, round("1.919".parse::<f32>().unwrap()));
    }
}
