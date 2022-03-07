use super::{PropertyMap, StyledNode};
use crate::core::{Node, Stylesheet};

fn create_styled_node<'a>(node: &'a Box<Node>, stylesheet: &Stylesheet) -> StyledNode<'a> {
    // prepare basic information of StyledNode
    let mut props = PropertyMap::new();
    let children = create_styled_nodes(&node.children, stylesheet);

    // match CSS rules
    for matched_rule in stylesheet.rules.iter().filter(|r| r.matches(node)) {
        for declaration in &matched_rule.declarations {
            props.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    // all set :-)
    StyledNode {
        node_type: &node.node_type,
        properties: props,
        children: children,
    }
}

pub fn create_styled_nodes<'a>(nodes: &'a Vec<Box<Node>>, stylesheet: &Stylesheet) -> Vec<StyledNode<'a>> {
    nodes
        .iter()
        .map(|x| create_styled_node(x, stylesheet))
        .collect()
}
