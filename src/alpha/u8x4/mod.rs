use crate::pixels::U8x4;
use crate::CpuExtensions;
use crate::{ImageView, ImageViewMut};

use super::AlphaMulDiv;


mod native;
#[cfg(target_arch = "wasm32")]
mod wasm32;

impl AlphaMulDiv for U8x4 {
    fn multiply_alpha(
        src_image: &ImageView<Self>,
        dst_image: &mut ImageViewMut<Self>,
        cpu_extensions: CpuExtensions,
    ) {
        match cpu_extensions {
            #[cfg(target_arch = "wasm32")]
            CpuExtensions::Simd128 => unsafe { wasm32::multiply_alpha(src_image, dst_image) },
        }
    }

    fn multiply_alpha_inplace(image: &mut ImageViewMut<Self>, cpu_extensions: CpuExtensions) {
        match cpu_extensions {
            #[cfg(target_arch = "wasm32")]
            CpuExtensions::Simd128 => unsafe { wasm32::multiply_alpha_inplace(image) },
        }
    }

    fn divide_alpha(
        src_image: &ImageView<Self>,
        dst_image: &mut ImageViewMut<Self>,
        cpu_extensions: CpuExtensions,
    ) {
        match cpu_extensions {
            #[cfg(target_arch = "wasm32")]
            CpuExtensions::Simd128 => unsafe { wasm32::divide_alpha(src_image, dst_image) },
        }
    }

    fn divide_alpha_inplace(image: &mut ImageViewMut<Self>, cpu_extensions: CpuExtensions) {
        match cpu_extensions {
            #[cfg(target_arch = "wasm32")]
            CpuExtensions::Simd128 => unsafe { wasm32::divide_alpha_inplace(image) },
        }
    }
}
