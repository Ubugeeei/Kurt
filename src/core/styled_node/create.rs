use super::{PropertyMap, StyledDocument, StyledNode};
use crate::core::{Document, Node, Stylesheet};

pub fn create_styled_document<'a>(
    document: &'a Document,
    stylesheet: &Stylesheet,
) -> StyledDocument<'a> {
    let document_element = create_styled_node(&document.document_element, &stylesheet);
    StyledDocument {
        document_element: document_element,
    }
}

fn create_styled_node<'a>(node: &'a Box<Node>, stylesheet: &Stylesheet) -> StyledNode<'a> {
    let mut props = PropertyMap::new();
    let children = create_styled_nodes(&node.children, stylesheet);

    for matched_rule in stylesheet.rules.iter().filter(|r| r.matches(node)) {
        for declaration in &matched_rule.declarations {
            props.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    StyledNode {
        node_type: &node.node_type,
        properties: props,
        children: children,
    }
}

fn create_styled_nodes<'a>(
    nodes: &'a Vec<Box<Node>>,
    stylesheet: &Stylesheet,
) -> Vec<StyledNode<'a>> {
    nodes
        .iter()
        .map(|x| create_styled_node(x, stylesheet))
        .collect()
}
