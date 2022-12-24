use crate::render::dom::{
    chardata::Text,
    document::Document,
    element::{AttrMap, Element},
    node::Node,
};

pub struct HTMLParser {
    input: String,
    position: usize,
    current_char: char,
    chars: Vec<char>,
}

impl HTMLParser {
    pub fn new(input: String) -> HTMLParser {
        let chars = input.chars().collect::<Vec<char>>();
        HTMLParser {
            input,
            position: 0,
            chars: chars.clone(),
            current_char: chars[0],
        }
    }

    fn next_char(&mut self) {
        self.position += 1;
        if self.position > self.input.len() - 1 {
            self.current_char = '\0';
        } else {
            self.current_char = self.chars[self.position];
        }
    }
}

// TODO: error handling
impl HTMLParser {
    pub fn parse(&mut self, url: String) -> Document {
        let nodes = self.parse_nodes();
        let document_element = Element::new("html".to_string(), AttrMap::new(), nodes);
        Document::new(url.clone(), url, document_element)
    }

    fn parse_nodes(&mut self) -> Vec<Box<Node>> {
        let mut nodes = Vec::new();
        while self.current_char != '\0' {
            match self.current_char {
                '<' => match self.peek_char() {
                    '/' => {
                        let _tag_name = self.parse_end_tag();
                        break;
                    }
                    '!' => {
                        let _ = self.parse_doc_type(); // NOTE: skip doc type
                    }
                    _ => {
                        let (tag_name, attributes) = self.parse_start_tag();
                        let children = self.parse_nodes();
                        let element = Element::new(tag_name, attributes, children);
                        nodes.push(element);
                    }
                },
                _ => {
                    let text = self.parse_text();
                    nodes.push(Text::new(text));
                }
            }
        }
        nodes
    }
}

/// parse tag
impl HTMLParser {
    fn parse_start_tag(&mut self) -> (String, AttrMap) {
        self.consume_char('<');
        let tag_name = self.parse_identifier();
        self.consume_whitespace();
        let attributes = self.parse_attributes();
        self.consume_char('>');
        (tag_name, attributes)
    }

    fn parse_end_tag(&mut self) -> String {
        self.consume_char('<');
        self.consume_char('/');
        let tag_name = self.parse_identifier();
        self.consume_char('>');
        tag_name
    }

    fn parse_doc_type(&mut self) -> String {
        self.consume_char('<');
        self.consume_char('!');
        self.consume_char('D');
        self.consume_char('O');
        self.consume_char('C');
        self.consume_char('T');
        self.consume_char('Y');
        self.consume_char('P');
        self.consume_char('E');
        self.consume_whitespace();
        let doc_type = self.parse_identifier();
        self.consume_char('>');
        self.consume_whitespace();
        doc_type
    }
}

/// parse attributes
impl HTMLParser {
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();
        while self.current_char != '>' {
            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
            self.consume_whitespace();
        }
        attributes
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_identifier();
        self.consume_whitespace();
        self.consume_char('=');
        self.consume_whitespace();
        let value = self.parse_string();
        (name, value)
    }
}

/// parse text
impl HTMLParser {
    fn parse_text(&mut self) -> String {
        let mut result = String::new();
        while self.current_char != '<' && self.current_char != '\0' {
            result.push(self.current_char);
            self.next_char();
        }
        result
    }
}

/// parser utils
impl HTMLParser {
    fn parse_identifier(&mut self) -> String {
        let mut result = String::new();
        while self.current_char.is_alphanumeric() {
            result.push(self.current_char);
            self.next_char();
        }
        result
    }

    fn parse_string(&mut self) -> String {
        let mut result = String::new();
        self.consume_char('"');
        while self.current_char != '"' {
            result.push(self.current_char);
            self.next_char();
        }
        self.consume_char('"');
        result
    }

    fn consume_char(&mut self, c: char) {
        if self.current_char == c {
            self.next_char();
        } else {
            panic!("Expected char: {}, got: {}", c, self.current_char);
        }
    }

    fn consume_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.next_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.position > self.input.len() - 1 {
            '\0'
        } else {
            self.chars[self.position + 1]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::render::dom::node::NodeType;

    use super::*;

    #[test]
    fn test_parse_nodes() {
        assert_eq!(
            HTMLParser::new("<div>hello</div>".to_string()).parse_nodes(),
            vec![Box::new(Node {
                node_type: NodeType::Element(Element {
                    tag_name: String::from("div"),
                    attributes: AttrMap::new(),
                }),
                children: vec![Box::new(Node {
                    node_type: NodeType::Text(Text {
                        data: String::from("hello")
                    }),
                    children: vec![]
                })],
            })],
        );

        // skip doc type
        assert_eq!(
            HTMLParser::new(
                r#"<!DOCTYPE html>
<div>hello</div>"#
                    .to_string()
            )
            .parse_nodes(),
            vec![Box::new(Node {
                node_type: NodeType::Element(Element {
                    tag_name: String::from("div"),
                    attributes: AttrMap::new(),
                }),
                children: vec![Box::new(Node {
                    node_type: NodeType::Text(Text {
                        data: String::from("hello")
                    }),
                    children: vec![]
                })],
            })],
        );

        // complex html
        assert_eq!(
            HTMLParser::new(
                r#"<!DOCTYPE html>
<html>
<head>
  <title>hello</title>
</head>
<body>
  <div class="content pa-2">hello</div>
</body>
<script>
  console.log("hello");
</script>
</html>"#
                    .to_string()
            )
            .parse_nodes(),
            vec![Box::new(Node {
                node_type: NodeType::Element(Element {
                    tag_name: String::from("html"),
                    attributes: AttrMap::new(),
                }),
                children: vec![
                    Box::new(Node {
                        node_type: NodeType::Text(Text {
                            data: String::from("\n")
                        }),
                        children: vec![]
                    }),
                    Box::new(Node {
                        node_type: NodeType::Element(Element {
                            tag_name: String::from("head"),
                            attributes: AttrMap::new(),
                        }),
                        children: vec![
                            Box::new(Node {
                                node_type: NodeType::Text(Text {
                                    data: String::from("\n  ")
                                }),
                                children: vec![]
                            }),
                            Box::new(Node {
                                node_type: NodeType::Element(Element {
                                    tag_name: String::from("title"),
                                    attributes: AttrMap::new(),
                                }),
                                children: vec![Box::new(Node {
                                    node_type: NodeType::Text(Text {
                                        data: String::from("hello")
                                    }),
                                    children: vec![]
                                })],
                            }),
                            Box::new(Node {
                                node_type: NodeType::Text(Text {
                                    data: String::from("\n")
                                }),
                                children: vec![]
                            }),
                        ],
                    }),
                    Box::new(Node {
                        node_type: NodeType::Text(Text {
                            data: String::from("\n")
                        }),
                        children: vec![]
                    }),
                    Box::new(Node {
                        node_type: NodeType::Element(Element {
                            tag_name: String::from("body"),
                            attributes: AttrMap::new(),
                        }),
                        children: vec![
                            Box::new(Node {
                                node_type: NodeType::Text(Text {
                                    data: String::from("\n  ")
                                }),
                                children: vec![]
                            }),
                            Box::new(Node {
                                node_type: NodeType::Element(Element {
                                    tag_name: String::from("div"),
                                    attributes: {
                                        let mut attr = AttrMap::new();
                                        attr.insert(
                                            String::from("class"),
                                            String::from("content pa-2"),
                                        );
                                        attr
                                    },
                                }),
                                children: vec![Box::new(Node {
                                    node_type: NodeType::Text(Text {
                                        data: String::from("hello")
                                    }),
                                    children: vec![]
                                })],
                            }),
                            Box::new(Node {
                                node_type: NodeType::Text(Text {
                                    data: String::from("\n")
                                }),
                                children: vec![]
                            }),
                        ],
                    }),
                    Box::new(Node {
                        node_type: NodeType::Text(Text {
                            data: String::from("\n")
                        }),
                        children: vec![]
                    }),
                    Box::new(Node {
                        node_type: NodeType::Element(Element {
                            tag_name: String::from("script"),
                            attributes: AttrMap::new(),
                        }),
                        children: vec![Box::new(Node {
                            node_type: NodeType::Text(Text {
                                data: String::from("\n  console.log(\"hello\");\n")
                            }),
                            children: vec![]
                        })],
                    }),
                    Box::new(Node {
                        node_type: NodeType::Text(Text {
                            data: String::from("\n")
                        }),
                        children: vec![]
                    }),
                ],
            })],
        );
    }

    #[test]
    fn test_parse_start_tag() {
        assert_eq!(
            HTMLParser::new("<div id=\"main\" class=\"mt-1 pa-2 text-input\">".to_string())
                .parse_start_tag(),
            (String::from("div"), {
                let mut attributes = AttrMap::new();
                attributes.insert(String::from("id"), String::from("main"));
                attributes.insert(String::from("class"), String::from("mt-1 pa-2 text-input"));
                attributes
            }),
        );

        // includes whitespace
        assert_eq!(
            HTMLParser::new(
                r#"<div
    id="main"
    class="mt-1 pa-2 text-input"
>"#
                .to_string()
            )
            .parse_start_tag(),
            (String::from("div"), {
                let mut attributes = AttrMap::new();
                attributes.insert(String::from("id"), String::from("main"));
                attributes.insert(String::from("class"), String::from("mt-1 pa-2 text-input"));
                attributes
            }),
        );
    }

    #[test]
    fn test_parse_end_tag() {
        assert_eq!(
            HTMLParser::new("</div>".to_string()).parse_end_tag(),
            String::from("div"),
        );
    }

    #[test]
    fn test_parse_doc_type() {
        assert_eq!(
            HTMLParser::new("<!DOCTYPE html>".to_string()).parse_doc_type(),
            String::from("html"),
        );
    }

    #[test]
    fn test_parse_attributes() {
        assert_eq!(
            HTMLParser::new("id=\"main\" class=\"mt-1 pa-2 text-input\">".to_string())
                .parse_attributes(),
            {
                let mut attributes = AttrMap::new();
                attributes.insert(String::from("id"), String::from("main"));
                attributes.insert(String::from("class"), String::from("mt-1 pa-2 text-input"));
                attributes
            },
        );

        // includes whitespace
        assert_eq!(
            HTMLParser::new(
                r#"id="main"
class="mt-1 pa-2 text-input"
>"#
                .to_string()
            )
            .parse_attributes(),
            {
                let mut attributes = AttrMap::new();
                attributes.insert(String::from("id"), String::from("main"));
                attributes.insert(String::from("class"), String::from("mt-1 pa-2 text-input"));
                attributes
            },
        );
    }

    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            HTMLParser::new("id=\"main\"".to_string()).parse_attribute(),
            (String::from("id"), String::from("main")),
        );

        // many values
        assert_eq!(
            HTMLParser::new("class=\"mt-1 pa-2 text-input\"".to_string()).parse_attribute(),
            (String::from("class"), String::from("mt-1 pa-2 text-input")),
        );

        // includes whitespace
        assert_eq!(
            HTMLParser::new(
                r#"class =
"mt-1""#
                    .to_string()
            )
            .parse_attribute(),
            (String::from("class"), String::from("mt-1")),
        );
    }
}
