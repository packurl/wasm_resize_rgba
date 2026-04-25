use convolution::FilterType;
use dynamic_image_view::{DynamicImageView, DynamicImageViewMut};
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

#[link(wasm_import_module = "js")]
unsafe extern "C" {
    fn println(ptr: usize, len: usize);
}

#[inline(always)]
fn log(s: &str) {
    unsafe { println(s.as_ptr() as usize, s.len()) }
}

/// # Safety
/// We assume the pointer points to an array of the correct `len`.
#[unsafe(no_mangle)]
pub unsafe fn resize(
    ptr: *const u8,
    len: usize,
    input_width: usize,
    input_height: usize,
    output_width: usize,
    output_height: usize,
    hq: bool,
) -> Box<[u8; 8]> {
    let rgba = unsafe { std::slice::from_raw_parts(ptr, len) };
    let src = Image::from_vec_u8(input_width, input_height, rgba.to_vec(), PixelType::U8x4)
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    let mut dest = Image::new(output_width, output_height, PixelType::U8x4);
    let mut resizer = Resizer::new(ResizeAlg::Convolution(if hq {
        FilterType::CatmullRom
    } else {
        FilterType::Hamming
    }));
    resizer
        .resize(&src.view(), &mut dest.view_mut())
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    let data = dest.into_buffer().into_boxed_slice();
    let len = data.len() as u32;
    let ptr = Box::into_raw(data) as *mut u8 as u32;
    let mut ptr_and_len = Vec::with_capacity(8);
    ptr_and_len.extend_from_slice(&ptr.to_le_bytes());
    ptr_and_len.extend_from_slice(&len.to_le_bytes());
    unsafe { Box::from_raw(Box::into_raw(ptr_and_len.into_boxed_slice()) as *mut [u8; 8]) }
}

#[unsafe(no_mangle)]
pub fn malloc(len: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len);
    let ptr = vec.as_mut_ptr();
    core::mem::forget(vec);
    ptr
}

/// # Safety
/// We assume the pointer points to an array of the correct `len`.
#[unsafe(no_mangle)]
pub unsafe fn free(ptr: *mut u8, len: usize) {
    unsafe {
        Vec::from_raw_parts(ptr, 0, len);
    }
}
