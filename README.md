# Kurt Browser

A toy web browser in Rust. (minimal childish implementation)  
HTML parser, CSS parser, layout engine, renderer.

![スクリーンショット 2022-09-25 13 46 18](https://user-images.githubusercontent.com/71201308/192128809-f01c77ff-7f4e-41c0-8010-6e4058a7627d.png)

## Deps

- paint: [GTK](https://github.com/gtk-rs/gtk4-rs)
- JavaScript Engine: [Glasper](https://github.com/Ubugeeei/Glasper) (my scratch implementation)

## Usage

1. install deps

   - rust, cargo
   - gtk4
   - gnome-icon-theme

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

