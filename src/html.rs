#[allow(unused_imports)]
use crate::dom::{AttrMap, Element, Node, Text};
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{between, many, many1, parser, Parser, Stream};
use combine::{
    error::ParseError,
    parser::char::{letter, newline, space},
    satisfy,
};
use combine::{parser::char::char, sep_by};

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
 * 属性パース(複数)
 * @return ('attr key', 'attr value')
 */
fn attributes<Input>() -> impl Parser<Input, Output = AttrMap>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by::<Vec<(String, String)>, _, _, _>(
        attribute(),
        many::<String, _, _>(space().or(newline())),
    )
    .map(|attrs: Vec<(String, String)>| attrs.into_iter().collect())
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
            attribute().easy_parse("class=\"header\""),
            Ok((("class".to_string(), "header".to_string()), ""))
        );

        assert_eq!(
            attribute().easy_parse("class = \"header\""),
            Ok((("class".to_string(), "header".to_string()), ""))
        );
    }

    #[test]
    fn test_parse_attributes() {
        let mut expected_map = AttrMap::new();
        expected_map.insert("class".to_string(), "foobar".to_string());
        expected_map.insert("id".to_string(), "def".to_string());
        assert_eq!(
            attributes().easy_parse("class=\"foobar\" id=\"def\""),
            Ok((expected_map, ""))
        );

        assert_eq!(attributes().easy_parse(""), Ok((AttrMap::new(), "")))
    }
}
