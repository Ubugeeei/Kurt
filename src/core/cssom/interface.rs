use crate::core::{Node, NodeType};

#[derive(Debug, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}
impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Stylesheet { rules }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>, // a comma-separated list of selectors
    pub declarations: Vec<Declaration>,
}
impl Rule {
    pub fn matches(&self, node: &Box<Node>) -> bool {
        self.selectors.iter().any(|s| s.matches(node))
    }
}

pub type Selector = SimpleSelector;

#[derive(Debug, PartialEq)]
pub enum SimpleSelector {
    UniversalSelector,
    TypeSelector {
        tag_name: String,
    },
    AttributeSelector {
        tag_name: String,
        op: AttributeSelectorOp,
        attribute: String,
        value: String,
    },
    ClassSelector {
        class_name: String,
    },
}
impl SimpleSelector {
    pub fn matches(&self, node: &Box<Node>) -> bool {
        match self {
            SimpleSelector::UniversalSelector => true,

            SimpleSelector::TypeSelector { tag_name } => match node.node_type {
                NodeType::Element(ref element) => element.tag_name.as_str() == tag_name,
                _ => false,
            },

            SimpleSelector::AttributeSelector {
                tag_name,
                op,
                attribute,
                value,
            } => match node.node_type {
                NodeType::Element(ref element) => {
                    element.tag_name.as_str() == tag_name
                        && match op {
                            AttributeSelectorOp::Eq => {
                                element.attributes.get(attribute) == Some(value)
                            }

                            AttributeSelectorOp::Contain => element
                                .attributes
                                .get(attribute)
                                .map(|value| {
                                    value
                                        .split_ascii_whitespace()
                                        .find(|v| v == value)
                                        .is_some()
                                })
                                .unwrap_or(false),
                        }
                }

                _ => false,
            },

            SimpleSelector::ClassSelector { class_name } => match node.node_type {
                NodeType::Element(ref element) => {
                    element.attributes.get("class") == Some(class_name)
                }
                _ => false,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AttributeSelectorOp {
    Eq,      // =
    Contain, // ~=
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: CSSValue,
    // TODO (enhancement): add a field for `!important`
}

#[derive(Debug, PartialEq, Clone)]
pub enum CSSValue {
    Keyword(String),
    Length((usize, Unit)),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Rem,
    Px,
    Percent,
}

#[cfg(test)]
mod tests {
    use crate::core::Element;

    use super::*;

    #[test]
    fn test_universal_selector_behaviour() {
        let element = &Element::new(
            "p".to_string(),
            [
                ("id".to_string(), "test".to_string()),
                ("class".to_string(), "testclass".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
            vec![],
        );
        assert!(SimpleSelector::UniversalSelector.matches(element));
    }
}
