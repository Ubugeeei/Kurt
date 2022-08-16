use crate::core::LayoutBox;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::ttf::{self};

use super::paint::{paint_base, paint_layout, PainterHeadPosition};

const HEADER_HEIGHT: u32 = 70;

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

    let mut pos = PainterHeadPosition::new(0, HEADER_HEIGHT);
    let _ = paint_base(&mut canvas);
    let _ = paint_layout(&mut canvas, &ttf_context, layout, &mut pos);

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
