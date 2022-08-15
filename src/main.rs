mod core;
use crate::core::{
    css::stylesheet::parse_css,
    html::parser::parse_html,
    {create_element_container, create_layout_document, create_styled_document},
};

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
  p, div {
    display: block;
  }
"#;

fn main() {
    // parse html
    let dom = parse_html(HTML).unwrap();

    // parse css
    let styles = format!("{}\n{}", DEFAULT_STYLESHEET, CSS);
    let cssom = parse_css(styles).unwrap();

    // create styled document from dom and cssom
    let styled_document = create_styled_document(&dom, &cssom);

    // calc layout
    let layout_document = create_layout_document(styled_document);

    // create box view
    let view = create_element_container(&layout_document.top_box);

    // rendering
    let mut siv = cursive::default();
    siv.add_fullscreen_layer(view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}
