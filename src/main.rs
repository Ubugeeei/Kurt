mod core;

use crate::core::render::render::render;

extern crate sdl2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    render()
}
