mod core;

use crate::core::css::stylesheet::parse_css;
use crate::core::html::parser::parse_nodes;

fn main() {
    let html_string = "\
      <html>\
        <head>\
          <title>my first html parse</title>\
        </head>\
\
        <body>\
          <div id=\"main\" class=\"content\">\
            <p>hello rust html parser!!</p>\
          </div>\
        </body>\
      </html>\
    ";

    let css_string = ".content { width: 1024px; font-size: 12px; }";

    // parse html
    let dom = parse_nodes(html_string);
    dbg!(dom);

    // parse css
    let cssom = parse_css(String::from(css_string));
    dbg!(cssom);
}
