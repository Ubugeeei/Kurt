#[allow(unused_imports)]
use crate::html_parser::dom::{AttrMap, Element, Node, Text};
use combine::parser::char::char;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{between, many, many1, Parser, Stream};
use combine::{
    error::ParseError,
    parser::char::{letter, newline, space},
};

use super::parse_attributes::attributes;

pub fn open_tag<Input>() -> impl Parser<Input, Output = (String, AttrMap)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let open_tag_name = many1::<String, _, _>(letter());
    let open_tag_content = (
        open_tag_name,
        many::<String, _, _>(space().or(newline())),
        attributes(),
    )
        .map(|v: (String, _, AttrMap)| (v.0, v.2));
    between(char('<'), char('>'), open_tag_content)
}
