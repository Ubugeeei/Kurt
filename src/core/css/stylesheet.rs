use combine::{error::ParseError, many, Parser, Stream};

use crate::core::utils::whitespaces;

use super::{rule, Rule, Stylesheet};

pub fn parse_css(raw: String) -> Stylesheet {
    rules()
        .parse(raw.as_str())
        .map(|(rules, _)| Stylesheet::new(rules))
        .unwrap()
}

fn rules<Input>() -> impl Parser<Input, Output = Vec<Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (whitespaces(), many(rule().skip(whitespaces()))).map(|(_, rules)| rules)
}
