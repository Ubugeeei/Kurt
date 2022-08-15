use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, rect::Rect};

use crate::core::LayoutBox;

// TODO: render layout
pub fn render(layout: &LayoutBox) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

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

    paint(&mut canvas, layout);

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

fn paint(canvas: &mut Canvas<Window>, _layout: &LayoutBox) {
    // draw base
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    // canvas.set_draw_color(Color::RGB(255, 210, 0));
    // let _ = canvas.fill_rect(Rect::new(10, 10, 780, 580));
    // canvas.present();
}
