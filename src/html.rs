#[allow(unused_imports)]
use crate::dom::{AttrMap, Element, Node, Text};
use combine::{error::ParseError, parser::char::{letter, newline, space}, satisfy};
use combine::parser::char::char;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{parser, Parser, Stream, many1, many, between};


/**
 * 属性パース
 * @return ('attr key', 'attr value')
 */
fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        // まずは属性の名前を何文字か読む
        many1::<String, _, _>(letter()),
        // 空白と改行を読み飛ばす
        many::<String, _, _>(space().or(newline())),
        // = を読む
        char('='),
        // 空白と改行を読み飛ばす
        many::<String, _, _>(space().or(newline())),
        // 引用符の間の、引用符を含まない文字を読む
        between(
            char('"'),
            char('"'),
            many1::<String, _, _>(satisfy(|c: char| c != '"')),
        ),
    )
        .map(|v| (v.0, v.4))
}

/**
 * ====================================================
 *  unit tests
 * ====================================================
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            attribute().easy_parse("test=\"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );

        assert_eq!(
            attribute().easy_parse("test = \"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        )
    }
}
