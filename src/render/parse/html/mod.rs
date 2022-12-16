use std::collections::HashMap;

use crate::render::dom::{chardata::Text, element::Element, node::Node};

mod lex;
mod token;

pub struct Parser {
    lexer: lex::Lexer,
    tokens: Vec<token::HTMLToken>,
    position: usize,
    current_token: token::HTMLToken,
    read_position: usize,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lexer = lex::Lexer::new(input);
        let tokens = lexer.lex();
        Parser {
            lexer,
            tokens,
            position: 0,
            read_position: 0,
            current_token: token::HTMLToken::EOF,
        }
    }

    pub fn parse(&mut self) -> Vec<Box<Node>> {
        let mut nodes = Vec::new();

        while self.peek_non_white_token() != token::HTMLToken::EOF {
            nodes.push(self.parse_node());
        }

        nodes
    }

    fn parse_node(&mut self) -> Box<Node> {
        let token = self.peek_non_white_token();
        match token {
            token::HTMLToken::Lt => {
                self.next_non_white_token(); // skip '<'
                self.next_non_white_token();

                let token = self.peek_non_white_token();
                match token {
                    token::HTMLToken::Word(tag_name) => self.parse_element(tag_name),
                    _ => panic!(),
                }
            }
            token::HTMLToken::Word(_) => self.parse_text(),
            _ => panic!(),
        }
    }

    fn parse_element(&mut self, tag_name: String) -> Box<Node> {
        let mut attributes = HashMap::new();
        let mut children = Vec::new();

        // TODO:
        Element::new(tag_name, attributes, children)
    }

    fn parse_text(&mut self) -> Box<Node> {
        let mut text = String::new();

        // TODO:
        Text::new(text)
    }

    fn next(&mut self) {
        self.current_token = self.tokens[self.position].clone();
        self.position += 1;
    }

    fn next_non_white_token(&mut self) {
        loop {
            self.next();
            if self.current_token != token::HTMLToken::Space
                && self.current_token != token::HTMLToken::Tab
                && self.current_token != token::HTMLToken::Newline
            {
                break;
            }
        }
    }

    fn peek(&self, idx: usize) -> token::HTMLToken {
        self.tokens[self.position + idx].clone()
    }

    fn peek_non_white_token(&self) -> token::HTMLToken {
        let mut idx = 0;
        loop {
            let token = self.tokens[self.position + idx].clone();
            if token != token::HTMLToken::Space
                && token != token::HTMLToken::Tab
                && token != token::HTMLToken::Newline
            {
                return token;
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::dom::{chardata::Text, element::Element};
    use std::collections::HashMap;

    #[test]
    fn test_parse() {
        let input = String::from(
            r#"<html>
  <body>
    <p class="my-class my-class2">Hello, world!</p>
    <p class="my-class2">Hello, world!</p>
  </body>
</html>
        "#,
        );
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(
            nodes,
            vec![Element::new(
                String::from("html"),
                HashMap::new(),
                vec![Element::new(
                    String::from("body"),
                    HashMap::new(),
                    vec![
                        Element::new(
                            String::from("p"),
                            {
                                let mut attr = HashMap::new();
                                attr.insert(
                                    String::from("class"),
                                    String::from("my-class my-class2"),
                                );
                                attr
                            },
                            vec![Text::new(String::from("Hello, world!"))]
                        ),
                        Element::new(
                            String::from("p"),
                            {
                                let mut attr = HashMap::new();
                                attr.insert(String::from("class"), String::from("my-class2"));
                                attr
                            },
                            vec![Text::new(String::from("Hello, world!"))],
                        )
                    ]
                )]
            )]
        );
    }
}
