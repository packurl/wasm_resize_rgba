use crate::pixels::U8x4;
use crate::{ImageView, ImageViewMut};

/// An immutable view of image data used by resizer as source image.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DynamicImageView<'a> {
    U8x4(ImageView<'a, U8x4>)
}

/// A mutable view of image data used by resizer as destination image.
#[derive(Debug)]
#[non_exhaustive]
pub enum DynamicImageViewMut<'a> {
    U8x4(ImageViewMut<'a, U8x4>)
}

macro_rules! from_typed {
    ($pixel_type: ty, $enum: expr, $enum_mut: expr) => {
        impl<'a> From<ImageView<'a, $pixel_type>> for DynamicImageView<'a> {
            fn from(view: ImageView<'a, $pixel_type>) -> Self {
                $enum(view)
            }
        }

        impl<'a> From<ImageViewMut<'a, $pixel_type>> for DynamicImageViewMut<'a> {
            fn from(view: ImageViewMut<'a, $pixel_type>) -> Self {
                $enum_mut(view)
            }
        }
    };
}

from_typed!(U8x4, DynamicImageView::U8x4, DynamicImageViewMut::U8x4);

impl<'a> From<DynamicImageViewMut<'a>> for DynamicImageView<'a> {
    fn from(dyn_view: DynamicImageViewMut<'a>) -> Self {
        use DynamicImageViewMut::*;
        match dyn_view {
            U8x4(typed_view) => DynamicImageView::U8x4(typed_view.into())
        }
    }
}
