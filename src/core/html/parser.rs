//! This module includes some implementations on HTML.

use super::super::dom::{Element, Node, Text};
use super::tag::{close_tag, open_tag};

use combine::satisfy;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{
    attempt,
    error::{StreamError, StringStreamError},
    many,
};
use combine::{choice, error::ParseError};
use combine::{many1, parser, Parser, Stream};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HTMLParseError {
    #[error("failed to parse; {0}")]
    InvalidResourceError(StringStreamError),
}

// [NOTE] Specification on HTML parsing: https://html.spec.whatwg.org/multipage/parsing.html#parsing
//
// The specification defines parsing algorithm of HTML, which takes input stream as argument and emits DOM.
// It consists of the following two stages:
// 1. tokenization stage
// 2. tree construction stage
// The first one, tokenization stage, generates tokens from input stream.
// The latter one, tree construction stage, constructs a DOM while handling scripts inside <script> tags.
//
// This implementation omits details of those two stages for simplicity.
// Please check the following if you'd like to know about the parsing process more deeply:
// - html5ever crate by Serve project https://github.com/servo/html5ever
// - HTMLDocumentParser, HTMLTokenizer, HTMLTreeBuilder of Chromium (src/third_party/blink/renderer/core/html/parser/*)

// TODO: 未実装
/// This functions parses `response` as HTML in non-standard manner.
// pub fn parse(response: Response) -> Result<Document, HTMLParseError> {
//     // NOTE: Here we assume the resource is HTML and encoded by UTF-8.
//     // We should determine character encoding as follows:
//     // https://html.spec.whatwg.org/multipage/parsing.html#the-input-byte-streama
//     let nodes = parse_without_normalziation(response.data);
//     match nodes {
//         Ok(nodes) => {
//             let document_element = if nodes.len() == 1 {
//                 nodes.into_iter().nth(0).unwrap()
//             } else {
//                 Element::new("html".to_string(), AttrMap::new(), nodes)
//             };
//             Ok(Document::new(
//                 response.url.to_string(),
//                 response.url.to_string(),
//                 document_element,
//             ))
//         }
//         Err(e) => Err(e),
//     }
// }
// FIXME: 仮置
pub fn parse_nodes(html_string: &str) -> Result<Vec<Box<Node>>, HTMLParseError> {
    nodes()
        .parse(html_string)
        .map(|(nodes, _)| nodes)
        .map_err(|e| HTMLParseError::InvalidResourceError(e))
}

pub fn parse_without_normalziation(data: Vec<u8>) -> Result<Vec<Box<Node>>, HTMLParseError> {
    // NOTE: Here we assume the resource is HTML and encoded by UTF-8.
    // We should determine character encoding as follows:
    // https://html.spec.whatwg.org/multipage/parsing.html#the-input-byte-streama
    let body = String::from_utf8(data).unwrap();

    nodes()
        .parse(&body as &str)
        .map(|(nodes, _)| nodes)
        .map_err(|e| HTMLParseError::InvalidResourceError(e))
}

// `nodes_` (and `nodes`) tries to parse input as Element or Text.
pub fn nodes_<Input>() -> impl Parser<Input, Output = Vec<Box<Node>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(many(choice((attempt(element()), attempt(text())))))
}

/// `text` consumes input until `<` comes.
fn text<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(satisfy(|c: char| c != '<')).map(|t| Text::new(t))
}

/// `element` consumes `<tag_name attr_name="attr_value" ...>(children)</tag_name>`.
fn element<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (open_tag(), nodes(), close_tag()).and_then(
        |((open_tag_name, attributes), children, close_tag_name)| {
            if open_tag_name == close_tag_name {
                Ok(Element::new(open_tag_name, attributes, children))
            } else {
                Err(<Input::Error as combine::error::ParseError<
                    char,
                    Input::Range,
                    Input::Position,
                >>::StreamError::message_static_message(
                    "tag name of open tag and close tag mismatched",
                ))
            }
        },
    )
}

parser! {
    fn nodes[Input]()(Input) -> Vec<Box<Node>>
    where [Input: Stream<Token = char>]
    {
        nodes_()
    }
}

#[cfg(test)]
mod tests {

    use crate::core::AttrMap;

    use super::*;

    // parsing tests of an element
    #[test]
    fn test_parse_element() {
        assert_eq!(
            element().easy_parse("<p></p>"),
            Ok((Element::new("p".to_string(), AttrMap::new(), vec![]), ""))
        );

        assert_eq!(
            element().easy_parse("<p>Hello World</p>"),
            Ok((
                Element::new(
                    "p".to_string(),
                    AttrMap::new(),
                    vec![Text::new("Hello World".to_string())]
                ),
                ""
            ))
        );

        assert!(element().easy_parse("<p>Hello World</div>").is_err());
    }

    // parsing tests of a tag
    #[test]
    fn test_parse_text() {
        {
            assert_eq!(
                text().easy_parse("Hello World"),
                Ok((Text::new("Hello World".to_string()), ""))
            );
        }
        {
            assert_eq!(
                text().easy_parse("Hello World<"),
                Ok((Text::new("Hello World".to_string()), "<"))
            );
        }
    }
}
