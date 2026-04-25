![crates.io](https://img.shields.io/crates/v/wasm_resize_rgba.svg)

[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) libs for resizing an rgba ImageData.

This is a based on a fork of the [fast_image_resize](https://github.com/Cykooz/fast_image_resize) [rust](https://www.rust-lang.org/) [crate](https://crates.io/crates/fast_image_resize).

<br>

Compilation:

`cargo build --release`

Wasm file optimization:

`wasm-opt --dce --vacuum -Os target/wasm32-unknown-unknown/release/wasm_avif.wasm -o avif.wasm`

<br>

