#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!("{}", s);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn resize(rgba: &[u8], input_width: usize, input_height: usize, output_width: usize, output_height: usize, hq: bool) -> Vec<u8> {
    let src = Image::from_vec_u8(
        input_width,
        input_height,
        rgba.to_vec(),
        PixelType::U8x4
    )
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    let mut dest = Image::new(
        output_width,
        output_height,
        PixelType::U8x4
    );
    let mut resizer = Resizer::new(ResizeAlg::Convolution(if hq { FilterType::CatmullRom } else { FilterType::Hamming }));
    resizer.resize(&src.view(), &mut dest.view_mut())
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    dest.buffer().to_vec()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn resize(rgba: &[u8], input_width: usize, input_height: usize, output_width: usize, output_height: usize, hq: bool) -> Vec<u8> {
    let src = Image::from_vec_u8(
        input_width,
        input_height,
        rgba.to_vec(),
        PixelType::U8x4
    )
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    let mut dest = Image::new(
        output_width,
        output_height,
        PixelType::U8x4
    );
    let mut resizer = Resizer::new(ResizeAlg::Convolution(if hq { FilterType::CatmullRom } else { FilterType::Hamming }));
    resizer.resize(&src.view(), &mut dest.view_mut())
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    dest.buffer().to_vec()
}

use convolution::FilterType;
use dynamic_image_view::{
    DynamicImageView, DynamicImageViewMut,
};
use errors::*;
use image_view::{ImageView, ImageViewMut};
use pixels::PixelType;
use resizer::{CpuExtensions, ResizeAlg, Resizer};

use crate::image::Image;

mod alpha;
mod convolution;
mod dynamic_image_view;
mod errors;
mod image;
mod image_view;
mod pixels;
mod resizer;
#[cfg(target_arch = "wasm32")]
mod wasm32_utils;

#[cfg(test)]
mod tests {
    use image::io::Reader;
    use crate::resize;

    #[test]
    fn test_resize_bee_jpg() {
        let rgb_image = Reader::open("bee.jpg").unwrap().decode().unwrap();
        let input_width = rgb_image.width() as usize;
        let input_height = rgb_image.height() as usize;
        assert_eq!(input_width, 2960);
        assert_eq!(input_height, 2055);
        let rgba_image = rgb_image.into_rgba8();
        let data = rgba_image.to_vec();
        let output_width = 2074;
        let output_height = 1440;
        resize(data.as_slice(), input_width, input_height, output_width, output_height, false);
        resize(data.as_slice(), input_width, input_height, output_width, output_height, true);
    }
}
