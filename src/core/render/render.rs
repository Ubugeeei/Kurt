use crate::core::glasper::js::JavaScriptRuntime;
// use crate::core::runtime::JavaScriptRuntime;
use crate::core::{Document, LayoutBox, Node, NodeType};

use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::Application;

use super::paint::paint;

// use sdl2::event::Event;
// use sdl2::image::{self, InitFlag};
// use sdl2::keyboard::Keycode;
// use sdl2::ttf;

// use super::paint::{paint_base, paint_layout, PainterHeadPosition};

// const HEADER_HEIGHT: u32 = 60;

// TODO: render layout
pub fn render(
    layout: &LayoutBox,
    document: &Document,
    js_runtime: &mut JavaScriptRuntime,
) -> Result<(), Box<dyn std::error::Error>> {
    // execute javascript
    execute_javascript(js_runtime, document);

    // painting
    let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
    paint(&app);

    // // sdl init
    // let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;
    // let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    // let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    // // create window & canvas
    // let window = video_subsystem
    //     .window("panel-pop", 1600, 1000)
    //     .position_centered()
    //     .build()
    //     .map_err(|e| e.to_string())?;
    // let mut canvas = window
    //     .into_canvas()
    //     .software()
    //     .build()
    //     .map_err(|e| e.to_string())?;

    // // painting
    // let mut pos = PainterHeadPosition::new(0, HEADER_HEIGHT);
    // let _ = paint_base(&mut canvas);
    // let _ = paint_layout(&mut canvas, &ttf_context, layout, &mut pos);

    // // event loop
    // 'mainloop: loop {
    //     for event in sdl_context.event_pump()?.poll_iter() {
    //         match event {
    //             // Quit if the window is closed
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Option::Some(Keycode::Escape),
    //                 ..
    //             } => break 'mainloop,

    //             _ => {}
    //         }
    //     }
    // }

    Ok(())
}

fn execute_javascript(js_runtime: &mut JavaScriptRuntime, dom: &Document) {
    println!("");
    let root_element = &dom.document_element;
    fn _execute_javascript(js_runtime: &mut JavaScriptRuntime, node: &Node) {
        match node.node_type {
            NodeType::Element(ref element) => {
                if element.tag_name == "script" {
                    // let script_path = element.attributes.get("src").unwrap();
                    for child in node.children.iter() {
                        match child.node_type {
                            NodeType::Text(ref text) => {
                                let script = text.data.clone();
                                let _ = js_runtime.execute(script);
                            }
                            _ => (),
                        }
                    }
                } else {
                    for child in node.children.iter() {
                        _execute_javascript(js_runtime, child);
                    }
                }
            }
            _ => (),
        }
    }

    _execute_javascript(js_runtime, root_element);
}
