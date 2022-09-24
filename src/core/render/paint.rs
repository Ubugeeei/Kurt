// use crate::core::{BoxType, CSSValue, LayoutBox, NodeType, PropertyMap, Unit};
// use sdl2::gfx::primitives::DrawRenderer;
// use sdl2::image::LoadTexture;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::ttf::Sdl2TtfContext;
// use sdl2::video::Window;

use gtk::prelude::*;

// pub fn paint_base(canvas: &mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
//     // background
//     canvas.set_draw_color(Color::RGB(255, 255, 255));
//     canvas.clear();
//     canvas.present();

//     // header
//     // FIXME: magic number
//     canvas.set_draw_color(Color::RGB(60, 60, 60));
//     let _ = canvas.fill_rect(Rect::new(0, 0, 1600, 58));
//     canvas.present();

//     // TODO: cursor
//     canvas.set_draw_color(Color::RGB(30, 30, 30));
//     let _ = canvas.fill_rect(Rect::new(120, 10, 1000, 30));
//     canvas.present();
//     let _ = canvas.filled_circle(121, 25, 15, Color::RGB(30, 30, 30));
//     canvas.present();
//     let _ = canvas.filled_circle(1119, 25, 15, Color::RGB(30, 30, 30));
//     canvas.present();

//     let texture_creator = canvas.texture_creator();
//     let texture = texture_creator.load_texture("./assets/img/arrow-left.png")?;
//     canvas.copy(&texture, None, Rect::new(10, 15, 20, 24))?;
//     canvas.present();
//     let texture = texture_creator.load_texture("./assets/img/arrow-right.png")?;
//     canvas.copy(&texture, None, Rect::new(40, 15, 20, 24))?;
//     canvas.present();
//     let texture = texture_creator.load_texture("./assets/img/reload.png")?;
//     canvas.copy(&texture, None, Rect::new(70, 15, 20, 24))?;
//     canvas.present();
//     Ok(())
// }

// pub fn paint_layout(
//     canvas: &mut Canvas<Window>,
//     ttf_context: &Sdl2TtfContext,
//     layout: &LayoutBox,
//     pos: &mut PainterHeadPosition,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     match layout.box_type {
//         // if displfay none,  no painting
//         BoxType::NoneBox => return Ok(()),

//         // paint
//         BoxType::AnonymousBox => {
//             for child in layout.children.iter() {
//                 let _ = paint_layout(canvas, ttf_context, child, pos);
//             }
//         }

//         BoxType::BlockBox => {
//             let props = match layout.box_props {
//                 Some(ref props) => props,
//                 _ => return Ok(()),
//             };
//             match props.node_type {
//                 /*
//                  * render text
//                  */
//                 NodeType::Text(txt_node) => {
//                     // css props
//                     let color = get_color(&props.properties);

//                     let texture_creator = canvas.texture_creator();
//                     // TODO: get color from styles
//                     canvas.set_draw_color(color);
//                     let surface = ttf_context
//                         // TODO: get font-family from styles
//                         .load_font("./assets/Arial.ttf", 512)?
//                         .render(&txt_node.data)
//                         .blended(color)
//                         .map_err(|e| e.to_string())?;

//                     let texture = texture_creator
//                         .create_texture_from_surface(&surface)
//                         .map_err(|e| e.to_string())?;

//                     let target = Rect::new(
//                         pos.x as i32,
//                         pos.y as i32,
//                         (txt_node.data.chars().count() as u32) * 10,
//                         24,
//                     );

//                     canvas.copy(&texture, None, Some(target))?;
//                     canvas.present();
//                 }

//                 /*
//                  * render nodep
//                  */
//                 NodeType::Element(_elem_node) => {
//                     // css props
//                     let width = get_width(&props.properties);
//                     let height = get_height(&props.properties);
//                     let background_color = get_background_color(&props.properties);

//                     canvas.set_draw_color(background_color);
//                     let _ = canvas.fill_rect(Rect::new(pos.x as i32, pos.y as i32, width, height));
//                     canvas.present();

//                     // TODO: control brother x, y
//                     // pos.x += width;
//                     // pos.y += height;

//                     for child in layout.children.iter() {
//                         paint_layout(canvas, ttf_context, child, pos)?;
//                     }
//                 }
//             }
//         }

//         BoxType::InlineBox => {
//             let props = match layout.box_props {
//                 Some(ref props) => props,
//                 _ => return Ok(()),
//             };

//             // css props
//             let color = get_color(&props.properties);
//             let background_color = get_background_color(&props.properties);

//             match props.node_type {
//                 /*
//                  * render text
//                  */
//                 NodeType::Text(txt_node) => {
//                     let texture_creator = canvas.texture_creator();
//                     let surface = ttf_context
//                         // TODO: get font-family from styles
//                         .load_font("./assets/Arial.ttf", 512)?
//                         .render(&txt_node.data)
//                         .blended(color)
//                         .map_err(|e| e.to_string())?;
//                     let texture = texture_creator
//                         .create_texture_from_surface(&surface)
//                         .map_err(|e| e.to_string())?;
//                     // FIXME: 24: get by font-size
//                     let target = Rect::new(
//                         pos.x as i32,
//                         pos.y as i32,
//                         (txt_node.data.chars().count() as u32) * 10,
//                         24,
//                     );
//                     canvas.copy(&texture, None, Some(target))?;
//                     canvas.present();
//                 }

//                 /*
//                  * render nodep
//                  */
//                 NodeType::Element(_elem_node) => {
//                     canvas.set_draw_color(background_color);
//                     // FIXME: 1600. 24
//                     let _ = canvas.fill_rect(Rect::new(pos.x as i32, pos.y as i32, 1600, 24));
//                     canvas.present();

//                     // TODO: control brother x, y
//                     // pos.x += width;
//                     // pos.y += height;

//                     for child in layout.children.iter() {
//                         let _ = paint_layout(canvas, ttf_context, child, pos);
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// fn get_width(props: &PropertyMap) -> u32 {
//     match props.get("width") {
//         Some(v) => match v {
//             CSSValue::Length(l) => match l.1 {
//                 Unit::Px => l.0 as u32,
//                 Unit::Rem => (l.0 * 16) as u32,
//                 Unit::Percent => todo!("todo get parent width"),
//             },
//             // invalid w1idth value
//             // set to default 128
//             CSSValue::Keyword(_) => 128 as u32,
//         },
//         None => todo!("get_width: auto compute. at {:?}", &props),
//     }
// }

// pub fn get_height(props: &PropertyMap) -> u32 {
//     match props.get("height") {
//         Some(v) => match v {
//             CSSValue::Length(l) => match l.1 {
//                 Unit::Px => l.0 as u32,
//                 Unit::Rem => (l.0 * 16) as u32,
//                 Unit::Percent => todo!("todo get parent width"),
//             },
//             // invalid w1idth value
//             // set to default 24
//             CSSValue::Keyword(_) => 24 as u32,
//         },
//         None => todo!("get_height: auto compute. at {:?}", &props),
//     }
// }

// pub fn get_color(props: &PropertyMap) -> Color {
//     match props.get("color") {
//         Some(v) => match v {
//             CSSValue::Keyword(k) => match &**k {
//                 "red" => Color::RED,
//                 "green" => Color::GREEN,
//                 "blue" => Color::BLUE,
//                 "black" => Color::BLACK,
//                 "white" => Color::WHITE,
//                 "yellow" => Color::YELLOW,
//                 "cyan" => Color::CYAN,
//                 "magenta" => Color::MAGENTA,
//                 _ => {
//                     todo!("rgb, rgba, hex")
//                 }
//             },
//             // set to default
//             CSSValue::Length(_) => Color::BLACK,
//         },
//         // set to default
//         None => Color::BLACK,
//     }
// }

// pub fn get_background_color(props: &PropertyMap) -> Color {
//     match props.get("background-color") {
//         Some(v) => match v {
//             CSSValue::Keyword(k) => match &**k {
//                 "red" => Color::RED,
//                 "green" => Color::GREEN,
//                 "blue" => Color::BLUE,
//                 "black" => Color::BLACK,
//                 "white" => Color::WHITE,
//                 "yellow" => Color::YELLOW,
//                 "cyan" => Color::CYAN,
//                 "magenta" => Color::MAGENTA,
//                 _ => {
//                     todo!("rgb, rgba, hex")
//                 }
//             },
//             // set to default
//             CSSValue::Length(_) => Color::WHITE,
//         },
//         // set to default
//         None => Color::WHITE,
//     }
// }
