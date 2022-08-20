# Kurt Browser
A toy web browser in Rust. (minimal childish implementation)  
HTML parser, CSS parser, layout enginge, renderer.  

### deps
- paint: [sdl2](https://github.com/Rust-SDL2/rust-sdl2)
- javascript engine: [rusty v8](https://github.com/denoland/rusty_v8)
  

## Usage
1. install deps  
    - rust, cargo
    - sdl2 (ttf, image, gfx)  
    https://github.com/Rust-SDL2/rust-sdl2#requirements  
    macOS ex) `brew install sdl2_ttf sdl2_image sdl2_gfx`

2. edit your html, css (./example)

3. buld and run  
    ```sh
    $ cargo run
    ```
---
https://user-images.githubusercontent.com/71201308/184876622-2c15ba58-3e76-4104-8007-1ae178ec5a3e.mov
