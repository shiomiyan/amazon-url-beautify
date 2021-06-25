pub mod parser {
    use regex::Regex;

    pub fn get_asin(url: &str) -> &str {
        let re = Regex::new(r"/dp/([A-Z0-9]+)/|\?").unwrap();
        re.captures(url).unwrap().get(1).unwrap().as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::url::parser::get_asin;

    #[test]
    fn test_parser_amazon() {
        let before: &str = "https://www.amazon.co.jp/%E3%82%B5%E3%83%B3%E3%83%88%E3%83%AA%E3%83%BC-%E4%BC%8A%E5%8F%B3%E8%A1%9B%E9%96%80-%E3%83%A9%E3%83%99%E3%83%AB%E3%83%AC%E3%82%B9%EF%BC%88%E6%97%A8%E3%81%BF%E8%8C%B6%E8%91%89%E5%85%A5%E3%82%8A%EF%BC%89%E3%81%8A%E8%8C%B6-525ml-%E3%83%9A%E3%83%83%E3%83%88%E3%83%9C%E3%83%88%E3%83%AB%E9%A3%B2%E6%96%99/dp/B08KT2QRL4/ref=sr_1_5?__mk_ja_JP=%E3%82%AB%E3%82%BF%E3%82%AB%E3%83%8A&dchild=1&keywords=%E3%81%8A%E8%8C%B6&qid=1624200629&sr=8-5";
        let after = "https://www.amazon.co.jp/dp/B08KT2QRL4".to_string();
        let result = "https://www.amazon.co.jp/dp/".to_string() + get_asin(before);
        assert_eq!(result, after);
    }
}
