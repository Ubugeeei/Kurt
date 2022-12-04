//! This module includes some implementations on HTML.

use super::{
    super::super::dom::{chardata::Text, element::Element, node::Node},
    tag::{close_tag, open_tag},
};
use crate::render::dom::{document::Document, element::AttrMap};
use regex::Regex;

#[allow(unused_imports)]
use combine::{
    attempt, choice,
    error::ParseError,
    error::{StreamError, StringStreamError},
    many, many1, parser, satisfy, EasyParser, Parser, Stream,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HTMLParseError {
    #[error("failed to parse; {0}")]
    InvalidResourceError(StringStreamError),
}

pub fn parse_html(html_string: &str) -> Result<Document, HTMLParseError> {
    let html_string = skip_doctype(html_string);
    let _nodes = nodes::<&str>()
        .parse(html_string.as_ref())
        .map(|(nodes, _)| nodes)
        .map_err(HTMLParseError::InvalidResourceError);

    match _nodes {
        Ok(nodes) => {
            let document_element = if nodes.len() == 1 {
                nodes.into_iter().next().unwrap()
            } else {
                Element::new("html".to_string(), AttrMap::new(), nodes)
            };
            // TODO: set url
            // Ok(Document::new(
            //     response.url.to_string(),
            //     response.url.to_string(),
            //     document_element,
            // ))
            Ok(Document::new(
                "".to_string(),
                "".to_string(),
                document_element,
            ))
        }
        Err(e) => Err(e),
    }
}

// `nodes_` (and `nodes`) tries to parse input as Element or Text.
pub fn nodes_<Input>() -> impl Parser<Input, Output = Vec<Box<Node>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(many(choice((attempt(element()), attempt(text())))))
}

fn text<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(satisfy(|c: char| c != '<')).map(Text::new)
}

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

fn skip_doctype(html: &str) -> String {
    let re = Regex::new(r"<!(DOCTYPE|doctype)[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element() {
        assert_eq!(
            element().easy_parse("<p></p>"),
            Ok((Element::new("p".to_string(), AttrMap::new(), vec![]), ""))
        );

        assert!(element().easy_parse("<p>Hello World</div>").is_err());
    }

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

    #[test]
    fn test_skip_doctype() {
        assert_eq!(
            skip_doctype("<!DOCTYPE html><html></html>"),
            "<html></html>"
        );
        assert_eq!(
            skip_doctype("<!doctype html><html></html>"),
            "<html></html>"
        );
    }

    #[test]
    fn test_parse_example() {
        // TODO: parse
        let html = r#"
            <!doctype html>
            <html>
            <head>
                <title>Example Domain</title>
            
                <meta charset="utf-8" />
                <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <style type="text/css">
                body {
                    background-color: #f0f0f2;
                    margin: 0;
                    padding: 0;
                    font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
                    
                }
                div {
                    width: 600px;
                    margin: 5em auto;
                    padding: 2em;
                    background-color: #fdfdff;
                    border-radius: 0.5em;
                    box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
                }
                a:link, a:visited {
                    color: #38488f;
                    text-decoration: none;
                }
                @media (max-width: 700px) {
                    div {
                        margin: 0 auto;
                        width: auto;
                    }
                }
                </style>    
            </head>
            
            <body>
            <div>
                <h1>Example Domain</h1>
                <p>This domain is for use in illustrative examples in documents. You may use this
                domain in literature without prior coordination or asking for permission.</p>
                <p><a href="https://www.iana.org/domains/example">More information...</a></p>
            </div>
            </body>
            </html>
        "#;

        assert!(parse_html(html).is_ok());
    }
}