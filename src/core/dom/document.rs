use super::Node;

#[derive(Debug, PartialEq)]
pub struct Document {
    pub url: String,
    pub document_uri: String,
    pub document_element: Box<Node>,
}

impl Document {
    pub fn new(url: String, document_uri: String, document_element: Box<Node>) -> Document {
        Document {
            url: url,
            document_uri: document_uri,
            document_element: document_element,
        }
    }
}
