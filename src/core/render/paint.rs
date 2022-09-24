#![allow(unused_variables)]

use crate::core::{
    cssom::{CSSValue, Unit},
    dom::NodeType,
    layout::{BoxType, LayoutBox},
    styled_node::PropertyMap,
};
use gtk::prelude::*;

pub fn paint_document(layout: &LayoutBox, main_container: &gtk::Box) {
    match layout.box_type {
        // if display none,  no painting
        BoxType::NoneBox => return,

        BoxType::AnonymousBox => {
            for child in layout.children.iter() {
                let _ = paint_document(child, main_container);
            }
        }

        BoxType::BlockBox => {
            let props = match layout.box_props {
                Some(ref props) => props,
                _ => return,
            };
            match props.node_type {
                /*
                 * render text
                 */
                NodeType::Text(txt_node) => {
                    // css props
                    let color = get_color(&props.properties); // TODO: set color
                    let text = gtk::Text::builder()
                        .text(txt_node.data.as_str())
                        .css_classes(vec!["kurt-text-default".to_string()])
                        .build();
                    main_container.append(&text);
                }

                /*
                 * render nodep
                 */
                NodeType::Element(_elem_node) => {
                    // css props
                    let width = get_width(&props.properties);
                    let height = get_height(&props.properties);
                    let background_color = get_background_color(&props.properties);

                    // TODO:render node

                    // recursive children
                    for child in layout.children.iter() {
                        paint_document(child, main_container);
                    }
                }
            }
        }

        BoxType::InlineBox => {
            let props = match layout.box_props {
                Some(ref props) => props,
                _ => return,
            };

            // css props
            let color = get_color(&props.properties);
            let background_color = get_background_color(&props.properties);

            match props.node_type {
                /*
                 * render text
                 */
                NodeType::Text(txt_node) => {
                    // css props
                    let color = get_color(&props.properties); // TODO: set color
                    let text = gtk::Text::builder()
                        .text(txt_node.data.as_str())
                        .css_classes(vec!["kurt-text-default".to_string()])
                        .build();
                    main_container.append(&text);
                }

                /*
                 * render nodep
                 */
                NodeType::Element(_elem_node) => {
                    for child in layout.children.iter() {
                        let _ = paint_document(child, main_container);
                    }
                    // TODO:render node
                    // todo!("render node");
                }
            }
        }
    }
}

pub fn get_color(props: &PropertyMap) -> &str {
    match props.get("color") {
        Some(v) => match v {
            CSSValue::Keyword(k) => &k,
            // set to default
            CSSValue::Length(_) => "black",
        },
        // set to default
        None => "black",
    }
}

fn get_width(props: &PropertyMap) -> u32 {
    match props.get("width") {
        Some(v) => match v {
            CSSValue::Length(l) => match l.1 {
                Unit::Px => l.0 as u32,
                Unit::Rem => (l.0 * 16) as u32,
                Unit::Percent => todo!("todo get parent width"),
            },
            // invalid width value
            // set to default 128
            CSSValue::Keyword(_) => 128 as u32,
        },
        None => 1600, // TODO: set to auto
    }
}

pub fn get_height(props: &PropertyMap) -> u32 {
    match props.get("height") {
        Some(v) => match v {
            CSSValue::Length(l) => match l.1 {
                Unit::Px => l.0 as u32,
                Unit::Rem => (l.0 * 16) as u32,
                Unit::Percent => todo!("todo get parent width"),
            },
            // invalid width value
            // set to default 24
            CSSValue::Keyword(_) => 24 as u32,
        },
        None => 15, // TODO: set to auto
    }
}

pub fn get_background_color(props: &PropertyMap) -> &str {
    match props.get("background-color") {
        Some(v) => match v {
            CSSValue::Keyword(k) => &k,
            // set to default
            CSSValue::Length(_) => "white",
        },
        // set to default
        None => "white",
    }
}
