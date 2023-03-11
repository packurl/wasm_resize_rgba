//! Functions and structs for working with colorspace and gamma.
use num_traits::bounds::UpperBounded;
use num_traits::Zero;

use crate::pixels::{GetCount, IntoPixelComponent, PixelComponent, PixelExt, Values};
use crate::{DynamicImageView, DynamicImageViewMut, MappingError};
use crate::{ImageView, ImageViewMut};

trait FromF32 {
    fn from_f32(x: f32) -> Self;
}

impl FromF32 for u8 {
    fn from_f32(x: f32) -> Self {
        x as Self
    }
}

impl FromF32 for u16 {
    fn from_f32(x: f32) -> Self {
        x as Self
    }
}
