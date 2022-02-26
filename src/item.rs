pub struct Item {
    pub id: u32,
    pub name: String,
    pub class: String,
    pub typo: String,
    pub ware: String,
    pub quality: String,
    pub rarity: String,
    pub stat_trak: bool,
}

fn pref_value(name: String, class: String, typo: String) -> (String, String, String) {
    let (name, mut class, mut typo) = (name, class, typo);
    match class.as_str() {
        "手套" => {
            if let Some((_name, _)) = name.split_once("|") {
                typo = _name
                    .replace("（★）", "")
                    .replace("裹手", "手部束带")
                    .trim()
                    .to_string();
            }
        }
        "印花" => {
            let splits = name.split("|").collect::<Vec<&str>>();
            if splits.len() == 3 {
                typo = splits[2].trim().to_string();
            }
        }
        _ => {}
    }
    match typo.as_str() {
        "SSG 08" | "SCAR-20" | "AWP" | "G3SG1" => class = "狙击步枪".to_string(),
        _ => {}
    }

    (name, class, typo)
}

impl Item {
    pub fn new(
        name: String,
        class: String,
        typo: String,
        ware: String,
        quality: String,
        rarity: String,
        stat_trak: bool,
    ) -> Item {
        let (name, class, typo) = pref_value(name, class, typo);
        Item {
            id: 0,
            name,
            class,
            typo,
            ware,
            quality,
            rarity,
            stat_trak,
        }
    }
}

#[derive(Debug)]
pub struct PriceInfo {
    pub id: usize,
    pub item_id: u32,
    pub date: String,
    pub price: usize,
}

impl PriceInfo {
    pub fn new(item_id: u32, date: String, price: usize) -> Self {
        PriceInfo {
            id: 0,
            item_id,
            date,
            price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trans(n: &str, c: &str, t: &str) -> (String, String, String) {
        (n.to_string(), c.to_string(), t.to_string())
    }

    #[test]
    fn test_pref_value() {
        let (n1, c1, t1) = (
            String::from("SSG 08 | 鬼脸天蛾"),
            String::from("步枪"),
            String::from("SSG 08"),
        );
        assert_eq!(
            trans("SSG 08 | 鬼脸天蛾", "狙击步枪", "SSG 08"),
            pref_value(n1, c1, t1)
        );

        let (n1, c1, t1) = (
            String::from("G3SG1 | 黑暗豹纹"),
            String::from("步枪"),
            String::from("G3SG1"),
        );
        assert_eq!(
            trans("G3SG1 | 黑暗豹纹", "狙击步枪", "G3SG1"),
            pref_value(n1, c1, t1)
        );

        let (n1, c1, t1) = (
            String::from("裹手（★） | 森林色调"),
            String::from("手套"),
            String::from(""),
        );
        assert_eq!(
            trans("裹手（★） | 森林色调", "手套", "手部束带"),
            pref_value(n1, c1, t1)
        );

        let (n1, c1, t1) = (
            String::from("驾驶手套（★） | 雪豹"),
            String::from("手套"),
            String::from(""),
        );
        assert_eq!(
            trans("驾驶手套（★） | 雪豹", "手套", "驾驶手套"),
            pref_value(n1, c1, t1)
        );

        let (n1, c1, t1) = (
            String::from("印花 | Spirit（闪亮）| 2020 RMR"),
            String::from("印花"),
            String::from(""),
        );
        assert_eq!(
            trans("印花 | Spirit（闪亮）| 2020 RMR", "印花", "2020 RMR"),
            pref_value(n1, c1, t1)
        );

        let (n1, c1, t1) = (
            String::from("印花 | Vitality （全息） | 2021年斯德哥尔摩锦标赛"),
            String::from("印花"),
            String::from(""),
        );
        assert_eq!(
            trans(
                "印花 | Vitality （全息） | 2021年斯德哥尔摩锦标赛",
                "印花",
                "2021年斯德哥尔摩锦标赛"
            ),
            pref_value(n1, c1, t1)
        );
    }
}
