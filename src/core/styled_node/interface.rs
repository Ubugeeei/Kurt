use std::collections::HashMap;

use crate::core::{CSSValue, NodeType};

#[derive(Debug, PartialEq)]
pub struct StyledNode<'a> {
    pub node_type: &'a NodeType,
    pub children: Vec<StyledNode<'a>>,
    pub properties: HashMap<String, CSSValue>,
}
