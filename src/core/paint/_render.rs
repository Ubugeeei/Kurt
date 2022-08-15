use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas};
use sdl2::video::Window;
use sdl2::{event::Event, rect::Rect};

use crate::core::{LayoutBox, NodeType};

// TODO: render layout
pub fn render(layout: &LayoutBox) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("panel-pop", 1600, 1000)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    // draw base
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    // draw text
    let surface = ttf_context
        .load_font("./assets/Arial.ttf", 512)?
        .render("render txt!")
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    canvas.clear();
    let target = Rect::new(10, 10, 120, 24);
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();

    // paint_layout(&mut canvas, layout);

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

fn paint_layout(canvas: &mut Canvas<Window>, layout: &LayoutBox) {
    match layout.box_props {
        Some(ref props) => {
            match props.node_type {
                NodeType::Text(txt_node) => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    // canvas.fill_rect(Rect::new(0, 0, 800, 600));
                    // canvas.set_draw_color(Color::RGB(255, 255, 255));

                    // let mut font = ttf_ctx.load_font(font_path, 128)?;
                    // font.set_style(sdl2::ttf::FontStyle::BOLD);
                    // let surface = font
                    //   .render("Hello Rust!")
                    //   .blended(Color::RGBA(255, 0, 0, 255))
                    //   .map_err(|e| e.to_string())?;
                    // let texture = texture_creator
                    //   .create_texture_from_surface(&surface)
                    //   .map_err(|e| e.to_string())?;

                    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
                    canvas.clear();
                }
                NodeType::Element(elem_node) => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.fill_rect(Rect::new(0, 0, 800, 600));
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    // canvas.draw_text(elem_node.tag_name.as_str(), (0, 0));
                    todo!();
                }
            }
        }
        None => {
            todo!();
        }
    }
    // canvas.set_draw_color(Color::RGB(255, 210, 0));
    // let _ = canvas.fill_rect(Rect::new(10, 10, 780, 580));
    // canvas.present();
}
