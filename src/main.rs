mod core;

use crate::core::create_styled_nodes;
use crate::core::css::stylesheet::parse_css;
use crate::core::html::parser::parse_html;

const HTML: &str = "\
  <html>\
    <head>\
      <title>my first html parse</title>\
    </head>\
    <body>\
      <span class=\"hide\">hide</span>\
      <div id=\"main\" class=\"content\">\
        <p>hello rust html parser!!</p>\
      </div>\
    </body>\
  </html>\
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
fn main() {
    // parse html
    let dom = parse_html(HTML).unwrap();
    dbg!(&dom);

    // parse css
    let cssom = parse_css(String::from(CSS)).unwrap();
    dbg!(&cssom);

    let styled_nodes = create_styled_nodes(&dom, &cssom);
    dbg!(&styled_nodes);
}
