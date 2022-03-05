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

    let css_string = "\
      body {\
        background-color: #f0f0f0;\
      }\
\
      .content {\
        width: 960px;\
        margin: 0 auto;\
      }\
";

    // parse html
    let dom = parse_nodes(html_string);
    dbg!(dom);

    // parse css
    let cssom = parse_css(String::from(css_string));
    dbg!(cssom);
}
