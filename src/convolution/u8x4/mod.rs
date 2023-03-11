use crate::convolution::vertical_u8::vert_convolution_u8;
use crate::pixels::U8x4;
use crate::CpuExtensions;
use crate::{ImageView, ImageViewMut};

use super::{Coefficients, Convolution};

#[cfg(target_arch = "wasm32")]
mod wasm32;

impl Convolution for U8x4 {
    fn horiz_convolution(
        src_image: &ImageView<Self>,
        dst_image: &mut ImageViewMut<Self>,
        offset: u32,
        coeffs: Coefficients,
        cpu_extensions: CpuExtensions,
    ) {
        match cpu_extensions {
            #[cfg(target_arch = "wasm32")]
            CpuExtensions::Simd128 => {
                wasm32::horiz_convolution(src_image, dst_image, offset, coeffs)
            }
        }
    }

    fn vert_convolution(
        src_image: &ImageView<Self>,
        dst_image: &mut ImageViewMut<Self>,
        offset: u32,
        coeffs: Coefficients,
        cpu_extensions: CpuExtensions,
    ) {
        vert_convolution_u8(src_image, dst_image, offset, coeffs, cpu_extensions);
    }
}
