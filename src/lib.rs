use std::num::NonZeroU32;
// use fast_image_resize::{FilterType, Image, PixelType, Resizer};
// use fast_image_resize::ResizeAlg::Convolution;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn resize(rgba: &[u8], input_width: usize, input_height: usize, output_width: usize, output_height: usize, hq: bool) -> Vec<u8> {
    let src = Image::from_vec_u8(
        NonZeroU32::new(input_width as u32).unwrap(),
        NonZeroU32::new(input_height as u32).unwrap(),
        rgba.to_vec(),
        PixelType::U8x4
    ).unwrap();
    let mut dest = Image::new(
        NonZeroU32::new(output_width as u32).unwrap(),
        NonZeroU32::new(output_height as u32).unwrap(),
        PixelType::U8x4
    );
    let mut resizer = Resizer::new(ResizeAlg::Convolution(if hq { FilterType::CatmullRom } else { FilterType::Hamming }));
    resizer.resize(&src.view(), &mut dest.view_mut()).unwrap();
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
mod wasm32_utils;
