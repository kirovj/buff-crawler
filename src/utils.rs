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
