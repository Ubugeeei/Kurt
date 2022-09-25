use super::{BoxProps, BoxType, LayoutBox, LayoutDocument};
use crate::render::styled_node::{Display, StyledDocument, StyledNode};

pub fn create_layout_document(document: StyledDocument<'_>) -> LayoutDocument<'_> {
    let layout_box = create_layout_box(document.document_element);

    LayoutDocument {
        top_box: layout_box,
    }
}

fn create_layout_box(snode: StyledNode<'_>) -> LayoutBox<'_> {
    let box_type = match snode.display() {
        Display::Block => BoxType::BlockBox,
        Display::Inline => BoxType::InlineBox,
        Display::None => BoxType::NoneBox,
    };

    let box_props = BoxProps {
        node_type: snode.node_type,
        properties: snode.properties,
    };

    let mut layout = LayoutBox {
        box_type,
        box_props: Some(box_props),
        children: vec![],
    };

    for child in snode.children {
        match child.display() {
            Display::Block => {
                layout.children.push(create_layout_box(child));
            }
            Display::Inline => {
                layout
                    .inline_container()
                    .children
                    .push(create_layout_box(child));
            }
            Display::None => {}
        }
    }

    layout
}
