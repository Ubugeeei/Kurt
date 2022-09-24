mod core;

// use crate::core::glasper::js::JavaScriptRuntime;

use crate::core::render::render::render;

extern crate sdl2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     *
     * read files
     *
     */

    // TODO: fetch resource from url
    // let sample_html = std::fs::read_to_string("./example/example.html").unwrap();
    // let sample_css = std::fs::read_to_string("./example/example.css").unwrap();
    // let default_stylesheet = std::fs::read_to_string("./assets/css/default.css").unwrap();

    /*
     *
     * pre proccess
     *
     */

    // let dom = parse_html(&sample_html).unwrap();
    // let styles = format!("{}\n{}", default_stylesheet, sample_css);
    // let cssom = parse_css(styles).unwrap();
    // let styled_document = create_styled_document(&dom, &cssom);
    // let _layout_document = create_layout_document(styled_document);
    // let mut javascript_runtime = JavaScriptRuntime::new();

    /*
     *
     * render
     *
     */

    render()
}
