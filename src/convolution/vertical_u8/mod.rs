use crate::convolution::Coefficients;
use crate::pixels::PixelExt;
use crate::CpuExtensions;
use crate::{ImageView, ImageViewMut};

pub(crate) mod native;
#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm32;

pub(crate) fn vert_convolution_u8<T: PixelExt<Component = u8>>(
    src_image: &ImageView<T>,
    dst_image: &mut ImageViewMut<T>,
    offset: u32,
    coeffs: Coefficients,
    cpu_extensions: CpuExtensions,
) {
    match cpu_extensions {
        #[cfg(target_arch = "wasm32")]
        CpuExtensions::Simd128 => wasm32::vert_convolution(src_image, dst_image, offset, coeffs),
        #[cfg(not(target_arch = "wasm32"))]
        CpuExtensions::None => native::vert_convolution(src_image, dst_image, offset, coeffs),
    }
}
