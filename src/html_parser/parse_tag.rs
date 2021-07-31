use crate::html_parser::dom::AttrMap;
use combine::parser::char::char;

use combine::{between, many, many1, Parser, Stream};
use combine::{
    error::ParseError,
    parser::char::{letter, newline, space},
};

#[allow(unused_imports)]
use combine::EasyParser;

use super::parse_attributes::parse_attributes;

/**
 * 開始タグ パース
 */
pub fn parse_start_tag<Input>() -> impl Parser<Input, Output = (String, AttrMap)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // タグ名を読む
    let start_tag_name = many1::<String, _, _>(letter());
    // (名前, 属性) で格納
    let start_tag_content = (
        start_tag_name,
        // 空白/改行は読み飛ばす
        many::<String, _, _>(space().or(newline())),
        // 属性を読む
        parse_attributes(),
    )
        .map(|v: (String, _, AttrMap)| (v.0, v.2));

    // パース
    between(char('<'), char('>'), start_tag_content)
}

/**
 * 終了タグ パース
 */
pub fn parse_end_tag<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // タグ名を読む
    let end_tag_name = many1::<String, _, _>(letter());
    let end_tag_content = (char('/'), end_tag_name).map(|v| v.1);

    // パース
    between(char('<'), char('>'), end_tag_content)
}

/** ====================================================
 *   unit tests
 * ====================================================
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_start_tag() {
        {
            assert_eq!(
                parse_start_tag().easy_parse("<p>aaaa"),
                Ok((("p".to_string(), AttrMap::new()), "aaaa"))
            );
        }
        {
            let mut attributes = AttrMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            assert_eq!(
                parse_start_tag().easy_parse("<p id=\"test\">"),
                Ok((("p".to_string(), attributes), ""))
            )
        }
        {
            let result = parse_start_tag().easy_parse("<p id=\"test\" class=\"sample\">");
            let mut attributes = AttrMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            attributes.insert("class".to_string(), "sample".to_string());
            assert_eq!(result, Ok((("p".to_string(), attributes), "")));
        }

        {
            assert!(parse_start_tag().easy_parse("<p id>").is_err());
        }
    }

    #[test]
    fn test_parse_close_tag() {
        let result = parse_end_tag().easy_parse("</p>");
        assert_eq!(result, Ok(("p".to_string(), "")))
    }
}
