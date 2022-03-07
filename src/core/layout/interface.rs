use crate::core::{NodeType, PropertyMap};

#[derive(Debug, PartialEq)]
pub struct LayoutBox<'a> {
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum BoxType<'a> {
    BlockBox(BoxProps<'a>),
    InlineBox(BoxProps<'a>),
    AnonymousBox,
}

#[derive(Debug, PartialEq)]
pub struct BoxProps<'a> {
    pub node_type: &'a NodeType,
    pub properties: PropertyMap,
}
