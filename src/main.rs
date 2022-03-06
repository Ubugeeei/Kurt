mod core;

use crate::core::css::stylesheet::parse_css;
use crate::core::html::parser::parse_html;

const HTML: &str = r#"
  <html>
    <head>
      <title>my first html parse</title>
    </head>
    <body>
      <div id=\"main\" class=\"content\">
        <p>hello rust html parser!!</p>
      </div>
    </body>
  </html>
"#;

const CSS: &str = r#"
  .content {
    width: 1024px;
    font-size: 12px;
  }

  p[id~=hello] {
    font-size: 10px;
    font-weight: bold;
    color: grey;
  }
"#;
fn main() {
    // parse html
    let dom = parse_html(HTML);
    dbg!(dom);

    // parse css
    let cssom = parse_css(String::from(CSS));
    dbg!(cssom);
}
