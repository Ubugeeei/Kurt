#[allow(unused_imports)]
use crate::{
    core::dom::Text,
    core::dom::{AttrMap, Element},
};

// definition: https://dom.spec.whatwg.org/#interface-node
#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Box<Node>>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Element(super::element::Element),
    Text(super::chardata::Text),
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self.node_type {
            NodeType::Element(ref e) => {
                let attrs = e
                    .attributes
                    .iter()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| {
                        // TODO (security): do this securely! This might causes mXSS.
                        format!("{}=\"{}\"", k, v)
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let children = self
                    .children
                    .iter()
                    .clone()
                    .into_iter()
                    .map(|node| node.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                if attrs != "" {
                    format!("<{} {}>{}</{}>", e.tag_name, attrs, children, e.tag_name)
                } else {
                    format!("<{}>{}</{}>", e.tag_name, children, e.tag_name)
                }
            }
            NodeType::Text(ref t) => t.data.clone(),
        }
    }
}
