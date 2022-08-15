use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{self, InitFlag, LoadTexture, Sdl2ImageContext};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::{event::Event, rect::Rect};

use crate::core::{LayoutBox, NodeType};

// TODO: render layout
pub fn render(layout: &LayoutBox) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

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

    let _ = paint_base(&mut canvas, &ttf_context);
    // let _ = paint_layout(&mut canvas, &ttf_context, layout);
    // FIXME: 仮
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let surface = ttf_context
        .load_font("./assets/Arial.ttf", 512)?
        .render("Hello, world! My browser is working!")
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let target = Rect::new(10, 80, 360, 24);
    canvas.copy(&texture, None, Some(target))?;
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

fn paint_base(
    canvas: &mut Canvas<Window>,
    ttf_context: &Sdl2TtfContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // background
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    // header
    canvas.set_draw_color(Color::RGB(60, 60, 60));
    let _ = canvas.fill_rect(Rect::new(0, 0, 1600, 70));
    canvas.present();

    // TODO: cursor
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    let _ = canvas.fill_rect(Rect::new(120, 10, 1000, 30));
    canvas.present();
    let _ = canvas.filled_circle(121, 25, 15, Color::RGB(30, 30, 30));
    canvas.present();
    let _ = canvas.filled_circle(1119, 25, 15, Color::RGB(30, 30, 30));
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./assets/img/arrow-left.png")?;
    canvas.copy(&texture, None, Rect::new(10, 15, 20, 24))?;
    canvas.present();
    let texture = texture_creator.load_texture("./assets/img/arrow-right.png")?;
    canvas.copy(&texture, None, Rect::new(40, 15, 20, 24))?;
    canvas.present();
    let texture = texture_creator.load_texture("./assets/img/reload.png")?;
    canvas.copy(&texture, None, Rect::new(70, 15, 20, 24))?;
    canvas.present();
    Ok(())
}

fn paint_layout(
    canvas: &mut Canvas<Window>,
    ttf_context: &Sdl2TtfContext,
    layout: &LayoutBox,
) -> Result<(), Box<dyn std::error::Error>> {
    match layout.box_props {
        Some(ref props) => {
            match props.node_type {
                /*
                 * render text
                 */
                NodeType::Text(txt_node) => {
                    let texture_creator = canvas.texture_creator();
                    // TODO: get color from styles
                    canvas.set_draw_color(Color::RGB(0, 0, 0));

                    let surface = ttf_context
                        // TODO: get font-family from styles
                        .load_font("./assets/Arial.ttf", 512)?
                        .render(&txt_node.data)
                        // TODO: get color from styles
                        .blended(Color::RGB(0, 0, 0))
                        .map_err(|e| e.to_string())?;

                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;

                    canvas.clear();

                    let target = Rect::new(10, 10, 120, 24);

                    canvas.copy(&texture, None, Some(target))?;
                    canvas.present();
                    Ok(())
                }

                /*
                 * render node
                 */
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
