use crate::core::AttrMap;
use combine::{
    between,
    error::ParseError,
    many, many1,
    parser::char::{char, letter, newline, space},
    satisfy, sep_by, Parser, Stream,
};

/// `attributes` consumes `name1="value1" name2="value2" ... name="value"`
pub fn attributes<Input>() -> impl Parser<Input, Output = AttrMap>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by::<Vec<(String, String)>, _, _, _>(
        attribute(),
        many::<String, _, _>(space().or(newline())),
    )
    .map(|attrs: Vec<(String, String)>| {
        let m: AttrMap = attrs.into_iter().collect();
        m
    })
}

/// `attribute` consumes `name="value"`.
fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let attribute_name = many::<String, _, _>(letter());
    let attribute_inner_value =
        many1::<String, _, _>(satisfy(|c: char| c != '"')).map(|x| x.replace("&quot;", "\""));
    let attribute_value = between(char('"'), char('"'), attribute_inner_value);
    (
        attribute_name,
        many::<String, _, _>(space().or(newline())),
        char('='),
        many::<String, _, _>(space().or(newline())),
        attribute_value,
    )
        .map(|v| (v.0, v.4))
}

#[cfg(test)]
mod tests {
    use combine::EasyParser;

    use super::*;
    // parsing tests of attributes
    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            attribute().easy_parse("test=\"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );

        assert_eq!(
            attribute().easy_parse("test = \"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );

        assert_eq!(
            attribute().easy_parse("test = \"&quot;&quot;\""),
            Ok((("test".to_string(), "\"\"".to_string()), ""))
        )
    }

    #[test]
    fn test_parse_attributes() {
        let mut expected_map = AttrMap::new();
        expected_map.insert("test".to_string(), "foobar".to_string());
        assert_eq!(
            attributes().easy_parse("test=\"foobar\""),
            Ok((expected_map, ""))
        );

        assert_eq!(attributes().easy_parse(""), Ok((AttrMap::new(), "")))
    }
}
