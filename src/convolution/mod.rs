pub use filters::{get_filter_func, FilterType};

use crate::pixels::PixelExt;
use crate::CpuExtensions;
use crate::{ImageView, ImageViewMut};

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod macros;

mod filters;
mod optimisations;
mod u8x4;
mod vertical_u8;

pub(crate) trait Convolution
where
    Self: PixelExt,
{
    fn horiz_convolution(
        src_image: &ImageView<Self>,
        dst_image: &mut ImageViewMut<Self>,
        offset: u32,
        coeffs: Coefficients,
        cpu_extensions: CpuExtensions,
    );

    fn vert_convolution(
        src_image: &ImageView<Self>,
        dst_image: &mut ImageViewMut<Self>,
        offset: u32,
        coeffs: Coefficients,
        cpu_extensions: CpuExtensions,
    );
}

#[derive(Debug, Clone, Copy)]
pub struct Bound {
    pub start: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct Coefficients {
    pub values: Vec<f64>,
    pub window_size: usize,
    pub bounds: Vec<Bound>,
}

#[derive(Debug, Clone, Copy)]
pub struct CoefficientsChunk<'a> {
    pub start: u32,
    pub values: &'a [f64],
}

pub fn precompute_coefficients(
    in_size: usize,
    in0: f64, // Left border for cropping
    in1: f64, // Right border for cropping
    out_size: usize,
    filter: fn(f64) -> f64,
    filter_support: f64,
) -> Coefficients {
    let scale = (in1 - in0) / out_size as f64;
    let filter_scale = scale.max(1.0);

    // Determine filter radius size (length of resampling filter)
    let filter_radius = filter_support * filter_scale;
    // Maximum number of coeffs per out pixel
    let window_size = filter_radius.ceil() as usize * 2 + 1;
    // Optimization: replace division by filter_scale
    // with multiplication by recip_filter_scale
    let recip_filter_scale = 1.0 / filter_scale;

    let count_of_coeffs = window_size * out_size;
    let mut coeffs: Vec<f64> = Vec::with_capacity(count_of_coeffs);
    let mut bounds: Vec<Bound> = Vec::with_capacity(out_size);

    for out_x in 0..out_size {
        // Find the point in the input image corresponding to the centre
        // of the current pixel in the output image.
        let in_center = in0 + (out_x as f64 + 0.5) * scale;

        // x_min and x_max are slice bounds for the input pixels relevant
        // to the output pixel we are calculating. Pixel x is relevant
        // if and only if (x >= x_min) && (x < x_max).
        // Invariant: 0 <= x_min < x_max <= width
        let x_min = (in_center - filter_radius).floor().max(0.) as u32;
        let x_max = (in_center + filter_radius).ceil().min(in_size as f64) as u32;

        let cur_index = coeffs.len();
        let mut ww: f64 = 0.0;

        // Optimisation for follow for-cycle:
        // (x + 0.5) - in_center => x - (in_center - 0.5) => x - center
        let center = in_center - 0.5;

        for x in x_min..x_max {
            let w: f64 = filter((x as f64 - center) * recip_filter_scale);
            coeffs.push(w);
            ww += w;
        }
        if ww != 0.0 {
            coeffs[cur_index..].iter_mut().for_each(|w| *w /= ww);
        }
        // Remaining values should stay empty if they are used despite x_max.
        coeffs.resize(cur_index + window_size, 0.);
        bounds.push(Bound {
            start: x_min,
            size: x_max - x_min,
        });
    }

    Coefficients {
        values: coeffs,
        window_size,
        bounds,
    }
}
