use crate::core::fetch::fetch_html;
use crate::core::glasper::js::JavaScriptRuntime;
use crate::core::html::parser::parse_html;
// use crate::core::runtime::JavaScriptRuntime;
use crate::core::{Document, Node, NodeType};

use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::Application;

// use sdl2::event::Event;
// use sdl2::image::{self, InitFlag};
// use sdl2::keyboard::Keycode;
// use sdl2::ttf;

// use super::paint::{paint_base, paint_layout, PainterHeadPosition};

// const HEADER_HEIGHT: u32 = 60;

// TODO: render layout
pub fn render() -> Result<(), Box<dyn std::error::Error>> {
    // painting
    let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
    app.connect_startup(|_| load_css());
    app.connect_activate(build_gui);
    app.run();
    Ok(())
}

fn build_gui(app: &gtk::Application) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1600)
        .height_request(1000)
        .build();

    let header_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    // header_container.set_css_classes(&vec!["header-container"]);
    window.set_child(Some(&header_container));

    let header_search_bar = gtk::Entry::builder()
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .css_classes(vec!["input".to_string()])
        .build();

    // handle the input
    header_search_bar.connect_activate(move |header_search_bar| {
        let url = header_search_bar.text().to_string();
        let html = fetch_html(&url);
        println!("---------------------------------------------------------");
        println!("[Fetch HTML: (url: {})]", url);
        println!("---------------------------------------------------------");
        println!("\n\x1b[30m{}\n...\x1b[0m\n", &html[..100]);
        println!("---------------------------------------------------------");

        let document = parse_html(&html).unwrap();
        println!("---------------------------------------------------------");
        println!("[Parse Document]");
        println!("---------------------------------------------------------");
        println!(
            "\n\x1b[30m{}\n...\x1b[0m\n",
            &format!("{:?}", document)[..100]
        );
        println!("---------------------------------------------------------");

        println!("---------------------------------------------------------");
        println!("[JavaScript Execution]");
        println!("---------------------------------------------------------");
        let mut javascript_runtime = JavaScriptRuntime::new();
        execute_javascript(&mut javascript_runtime, &document);
        println!("---------------------------------------------------------");

        // TODO: build layout
    });

    header_container.append(&header_search_bar);

    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        r#"
            .input {
                border-radius: 50px;
                padding-left: 10px;
                padding-right: 10px;
                outline: none;
                font-size: 15px;
                color: #888;
            }
    "#
        .as_bytes(),
    );

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
