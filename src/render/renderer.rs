use gtk::traits::BoxExt;

use crate::{
    http::fetch::fetch_html,
    javascript::glasper::js::JavaScriptRuntime,
    render::{
        dom::{
            document::Document,
            node::{Node, NodeType},
        },
        layout::create_layout_document,
        parser::css::stylesheet::parse_css,
        parser_next::html::HTMLParser,
        styled_node::create_styled_document,
    },
};
pub fn render_by_url(url: &str, scrolled_window: &gtk::ScrolledWindow, _refresh_btn: &gtk::Button) {
    // reset document_container
    let document_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    scrolled_window.set_child(Some(&document_container));

    if url.is_empty() {
        let default_title = gtk::Label::builder()
            .label("Hello, Kurt Browser!")
            .css_classes(vec!["default-title".to_string()])
            .build();
        let default_text = gtk::Label::builder()
            .label("Type \"localhost:3000\" and enter to get HTML!")
            .css_classes(vec!["default-text".to_string()])
            .build();
        document_container.append(&default_title);
        document_container.append(&default_text);
    } else {
        // refresh_btn.set_icon_name("window-close");
        let html = fetch_html(&url);
        println!("---------------------------------------------------------");
        println!("[\x1b[32mFetch HTML: (url: {})\x1b[0m]", url);
        println!("---------------------------------------------------------");
        println!("content");
        if html.len() > 100 {
            println!("\n\x1b[30m{}\n...\x1b[0m\n", &html[..100]);
        } else {
            println!("{}", html);
        }
        render_document(&html, &url, &document_container);
        // refresh_btn.set_icon_name("view-refresh"); // FIXME: not working!
    }
}

pub fn render_document(html: &str, url: &str, main_container: &gtk::Box) {
    let document = HTMLParser::new(html.to_string()).parse(url.to_string());
    println!("---------------------------------------------------------");
    println!("[\x1b[32mParse Document\x1b[0m]");
    println!("---------------------------------------------------------");
    println!("content");
    println!(
        "\n\x1b[30m{}\n...\x1b[0m\n",
        &format!("{:?}", document)[..100]
    );

    println!("---------------------------------------------------------");
    println!("[\x1b[32mJavaScript Execution\x1b[0m]");
    println!("---------------------------------------------------------");
    println!("log");
    let mut javascript_runtime = JavaScriptRuntime::new();
    let style_string = load(&mut javascript_runtime, &document);

    let default_stylesheets = std::fs::read_to_string("./assets/css/default.css").unwrap();
    let style_string = format!("{}{}", default_stylesheets, style_string);
    let cssom = parse_css(style_string).unwrap();
    println!("---------------------------------------------------------");
    println!("[\x1b[32mParse CSSOM: (default css)\x1b[0m]");
    println!("---------------------------------------------------------");
    println!("content");
    println!("\n\x1b[30m{}\n...\x1b[0m\n", &format!("{:?}", cssom)[..100]);

    let styled_document = create_styled_document(&document, &cssom);
    let layout_document = create_layout_document(styled_document);
    println!("---------------------------------------------------------");
    println!("[\x1b[32mGenerate Layout Document\x1b[0m]");
    println!("---------------------------------------------------------");
    println!("content");
    println!(
        "\n\x1b[30m{}\n...\x1b[0m\n",
        &format!("{:?}", layout_document.top_box)[..100]
    );

    println!("---------------------------------------------------------");
    println!("[\x1b[32mPaint Content\x1b[0m]");
    println!("---------------------------------------------------------");
    crate::render::paint::paint_document(&layout_document.top_box, &main_container);
    println!("done.");
}

type CSSString = String;
/// load script and styles
pub fn load(js_runtime: &mut JavaScriptRuntime, dom: &Document) -> CSSString {
    let mut css = String::new();

    let root_element = &dom.document_element;
    fn _load(js_runtime: &mut JavaScriptRuntime, node: &Node, css: &mut CSSString) -> CSSString {
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
                } else if element.tag_name == "style" {
                    for child in node.children.iter() {
                        match child.node_type {
                            NodeType::Text(ref text) => {
                                css.push_str(&text.data);
                            }
                            _ => (),
                        }
                    }
                } else {
                    for child in node.children.iter() {
                        _load(js_runtime, child, css);
                    }
                }
            }
            _ => (),
        };
        css.to_string()
    }

    _load(js_runtime, root_element, &mut css);

    return css;
}
