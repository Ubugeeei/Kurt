use crate::render::cssom::Stylesheet;
use crate::render::parser::rules;
use combine::{error::StringStreamError, Parser};

pub fn parse_css(raw: String) -> Result<Stylesheet, StringStreamError> {
    rules()
        .parse(raw.as_str())
        .map(|(rules, _)| Stylesheet::new(rules))
}
