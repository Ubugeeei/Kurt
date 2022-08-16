mod core;

use crate::core::{
    css::stylesheet::parse_css,
    html::parser::parse_html,
    render::render::render,
    runtime::JavaScriptRuntime,
    {create_layout_document, create_styled_document},
};

extern crate sdl2;

const HTML: &str = r#"
    <body>
      <div id="main" class="content">
        <div class="message">Hello, world!! My browser is working!</div>
        <p class="hide">this is hide element</p>
      </div>

      <script>
        let count = 6;
        const double = n => n * 2;
        double(count)
      </script>

      <script>
        const factorical = num => {
          if (num === 0) return 1;
          return num * factorical(num-1);
        };
        factorical(5)
      </script>
    </body>
"#;

const CSS: &str = r#"
  body {
    width: 1600px;
    height: 1000px;
  }

  .content {
    width: 1200px;
    height: 768px;
  }

  .message {
    width: 1200px;
    height: 100px;
  }

  p {
    width: 256px;
    height: 24px;
  }

  .fwb {
    font-weight: bold;
  }

  .red {
    color: red;
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

    // create JavaScript runtime
    let mut javascript_runtime = JavaScriptRuntime::new();

    // TODO: rendering
    render(&layout_document.top_box, &dom, &mut javascript_runtime)
}
