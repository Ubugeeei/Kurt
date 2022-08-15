mod core;
use crate::core::{
    css::stylesheet::parse_css,
    html::parser::parse_html,
    {create_element_container, create_layout_document, create_styled_document},
};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

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

    // create box view
    let view = create_element_container(&layout_document.top_box);

    // TODO: rendering
    // rendering
    // let mut siv = cursive::default();
    // siv.add_fullscreen_layer(view);
    // siv.add_global_callback('q', |s| s.quit());
    // siv.run();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("panel-pop", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                // Quit if the window is closed
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,

                // change background color
                Event::KeyUp {
                    keycode: Option::Some(Keycode::Space),
                    ..
                } => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    Ok(())
}
