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

   image:  
      https://user-images.githubusercontent.com/71201308/192097835-020f7111-02d2-4898-b2cc-1aeb346d6bb3.mov

5. exit
   ```sh
   $ make exit
   ```

---

