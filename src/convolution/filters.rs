use std::f64::consts::PI;

pub type FilterFn = fn(f64) -> f64;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum FilterType {
    /// Hamming filter has the same performance as `Bilinear` filter while
    /// providing the image downscaling quality comparable to bicubic
    /// (`CatmulRom` or `Mitchell`). Produces a sharper image than `Bilinear`,
    /// doesn't have dislocations on local level like with `Box`.
    /// The filter don’t show good quality for the image upscaling.
    Hamming,
    /// Catmull-Rom bicubic filter calculate the output pixel value using
    /// cubic interpolation on all pixels that may contribute to the output
    /// value.
    #[default]
    CatmullRom,
}

/// Returns reference to filter function and value of `filter_support`.
#[inline]
pub fn get_filter_func(filter_type: FilterType) -> (FilterFn, f64) {
    match filter_type {
        FilterType::Hamming => (hamming_filter, 1.0),
        FilterType::CatmullRom => (catmul_filter, 2.0)
    }
}

#[inline]
fn hamming_filter(mut x: f64) -> f64 {
    x = x.abs();
    if x == 0.0 {
        1.0
    } else if x >= 1.0 {
        0.0
    } else {
        x *= PI;
        (0.54 + 0.46 * x.cos()) * x.sin() / x
    }
}

/// Catmull-Rom (bicubic) filter
/// https://en.wikipedia.org/wiki/Bicubic_interpolation#Bicubic_convolution_algorithm
#[inline]
fn catmul_filter(mut x: f64) -> f64 {
    const A: f64 = -0.5;
    x = x.abs();
    if x < 1.0 {
        ((A + 2.) * x - (A + 3.)) * x * x + 1.
    } else if x < 2.0 {
        (((x - 5.) * x + 8.) * x - 4.) * A
    } else {
        0.0
    }
}

