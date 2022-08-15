mod core;

use crate::core::{
    css::stylesheet::parse_css,
    html::parser::parse_html,
    paint::_render::render,
    {create_layout_document, create_styled_document},
};

extern crate sdl2;

const HTML: &str = "\
    <body>\
      <div id=\"main\" class=\"content\">\
        <p>hello rust css parser!!</p>\
        <p class=\"hide\">hello rust css parser!!</p>\
        <p class=\"hide\">hello rust css parser!!</p>\
        <p class=\"hide\">hello rust css parser!!</p>\
        <p class=\"hide\">hello rust css parser!!</p>\
        <p class=\"hide\">hello rust css parser!!</p>\
        <p>hello</p>\
        <p>rust</p>\
        <p>parser!!</p>\
        \
      </div>\
    </body>\
";

const CSS: &str = r#"
  .content {
    width: 1024px;
    font-size: 12px;
  }

  p {
    font-size: 10px;
    font-weight: bold;
    color: grey;
  }

  .hide {
    display: none;
  }
"#;

const DEFAULT_STYLESHEET: &str = r#"
  script, style {
    display: none;
  }
  body {
    display: block;
    margin: 8px;
  }
  div {
    display: block;
  }
  p {
    display: block;
    margin-top: 16px;
    margin-bottom: 16px;
    margin-left: 0px;
    margin-right: 0px;
  }
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse html
    let dom = parse_html(HTML).unwrap();

    // parse css
    let styles = format!("{}\n{}", DEFAULT_STYLESHEET, CSS);
    let cssom = parse_css(styles).unwrap();

    // create styled document from dom and cssom
    let styled_document = create_styled_document(&dom, &cssom);

    // calc layout
    let layout_document = create_layout_document(styled_document);

    // TODO: rendering
    render(&layout_document.top_box)
}
