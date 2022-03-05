use super::Stylesheet;
use crate::core::rules;
use combine::Parser;

pub fn parse_css(raw: String) -> Stylesheet {
    rules()
        .parse(raw.as_str())
        .map(|(rules, _)| Stylesheet::new(rules))
        .unwrap()
}
