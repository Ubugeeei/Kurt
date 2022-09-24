# Kurt Browser

A toy web browser in Rust. (minimal childish implementation)  
HTML parser, CSS parser, layout engine, renderer.

## Deps

- paint: [GTK](https://github.com/gtk-rs/gtk4-rs)
- JavaScript Engine: [Glasper](https://github.com/Ubugeeei/Glasper) (my scratch implementation)

## Usage

1. install deps

   - rust, cargo
   - gtk4

2. edit your html, css (./example)

3. buld and run
   launch browser and sample server (localhost:3000)
   ```sh
   $ make sample
   ```

4. access server  
   able to:
      - localhost:3000
      - localhost:3000/scroll
      - localhost:3000/js  
   
https://user-images.githubusercontent.com/71201308/192105049-58ac6b20-a343-44fb-9f70-1535bf554fb3.mov



5. exit
   ```sh
   $ make exit
   ```

---

