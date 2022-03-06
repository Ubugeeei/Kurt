use super::{AttributeSelectorOp, Selector, SimpleSelector};
use crate::core::utils::whitespaces;
use combine::{
    choice,
    error::StreamError,
    many1, optional,
    parser::char::{self, char, letter, string},
    sep_by, ParseError, Parser, Stream,
};

pub fn selectors<Input>() -> impl Parser<Input, Output = Vec<Selector>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by(
        simple_selector().skip(whitespaces()),
        char(',').skip(whitespaces()),
    )
}

fn simple_selector<Input>() -> impl Parser<Input, Output = SimpleSelector>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let universal_selector = char('*').map(|_| SimpleSelector::UniversalSelector);

    let class_selector =
        (char('.'), many1(letter())).map(|(_, class_name)| SimpleSelector::ClassSelector {
            class_name: class_name,
        });

    let type_or_attribute_selector = (
        many1(letter()).skip(whitespaces()),
        optional((
            char('[').skip(whitespaces()),
            many1(letter()),
            choice((string("="), string("~="))),
            many1(letter()),
            char(']'),
        )),
    )
        .and_then(|(tag_name, opts)| match opts {
            Some((_, attribute, op, value, _)) => {
                let op = match op {
                    "=" => AttributeSelectorOp::Eq,
                    "~=" => AttributeSelectorOp::Contain,
                    _ => {
                        return Err(<Input::Error as combine::error::ParseError<
                            char,
                            Input::Range,
                            Input::Position,
                        >>::StreamError::message_static_message(
                            "invalid attribute selector op",
                        ))
                    }
                };
                Ok(SimpleSelector::AttributeSelector {
                    tag_name: tag_name,
                    attribute: attribute,
                    op: op,
                    value: value,
                })
            }
            None => Ok(SimpleSelector::TypeSelector { tag_name: tag_name }),
        });

    choice((
        universal_selector,
        class_selector,
        type_or_attribute_selector,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectors() {
        assert_eq!(
            selectors().parse("test [aa=bb], piyo[cc~=dd] {"),
            Ok((
                vec![
                    SimpleSelector::AttributeSelector {
                        tag_name: "test".to_string(),
                        attribute: "aa".to_string(),
                        op: AttributeSelectorOp::Eq,
                        value: "bb".to_string()
                    },
                    SimpleSelector::AttributeSelector {
                        tag_name: "piyo".to_string(),
                        attribute: "cc".to_string(),
                        op: AttributeSelectorOp::Contain,
                        value: "dd".to_string()
                    }
                ],
                "{"
            ))
        );
    }

    #[test]
    fn test_simple_selector() {
        assert_eq!(
            simple_selector().parse("* {"),
            Ok((SimpleSelector::UniversalSelector, " {"))
        );

        assert_eq!(
            simple_selector().parse("test{"),
            Ok((
                SimpleSelector::TypeSelector {
                    tag_name: "test".to_string(),
                },
                "{"
            ))
        );

        assert_eq!(
            simple_selector().parse("test [foo=bar] "),
            Ok((
                SimpleSelector::AttributeSelector {
                    tag_name: "test".to_string(),
                    attribute: "foo".to_string(),
                    op: AttributeSelectorOp::Eq,
                    value: "bar".to_string()
                },
                " "
            ))
        );

        assert_eq!(
            simple_selector().parse("test[foo~=bar]{"),
            Ok((
                SimpleSelector::AttributeSelector {
                    tag_name: "test".to_string(),
                    attribute: "foo".to_string(),
                    op: AttributeSelectorOp::Contain,
                    value: "bar".to_string()
                },
                "{"
            ))
        );
    }
}
