#[allow(unused_imports)]
use combine::EasyParser;

mod html_parser;
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
    let parser = html_parser::parse_node::parse_elements;
    let result = parser().easy_parse(html_string).ok().unwrap();
    dbg!(result);
}
