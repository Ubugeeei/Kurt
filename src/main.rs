mod core;

use crate::core::html::parse_nodes;

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

    let result = parse_nodes(html_string);
    dbg!(result);
}
