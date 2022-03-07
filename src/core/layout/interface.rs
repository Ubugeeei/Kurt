use crate::core::{NodeType, PropertyMap};

#[derive(Debug, PartialEq)]
pub struct LayoutDocument<'a> {
    pub top_box: LayoutBox<'a>,
}

#[derive(Debug, PartialEq)]
pub struct LayoutBox<'a> {
    pub box_type: BoxType,
    pub box_props: Option<BoxProps<'a>>,
    pub children: Vec<LayoutBox<'a>>,
}
impl<'a> LayoutBox<'a> {
    pub fn inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineBox | BoxType::NoneBox | BoxType::AnonymousBox => self,
            BoxType::BlockBox => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymousBox,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox {
                        box_type: BoxType::AnonymousBox,
                        box_props: None,
                        children: vec![],
                    }),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BoxType {
    BlockBox,
    InlineBox,
    NoneBox,
    AnonymousBox,
}

#[derive(Debug, PartialEq)]
pub struct BoxProps<'a> {
    pub node_type: &'a NodeType,
    pub properties: PropertyMap,
}
