use crate::pixels::{PixelExt, PixelType};
use crate::{DynamicImageView, DynamicImageViewMut, ImageBufferError, ImageView, ImageViewMut};

#[derive(Debug)]
enum BufferContainer {
    VecU8(Vec<u8>),
}


/// Simple container of image data.
#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    buffer: BufferContainer,
    pixel_type: PixelType,
}

impl Image {
    /// Create empty image with given dimensions and pixel type.
    pub fn new(width: usize, height: usize, pixel_type: PixelType) -> Self {
        let pixels_count = width * height;
        let buffer = BufferContainer::VecU8(vec![0; pixels_count * pixel_type.size()]);
        Self {
            width,
            height,
            buffer,
            pixel_type,
        }
    }

    pub fn from_vec_u8(
        width: usize,
        height: usize,
        buffer: Vec<u8>,
        pixel_type: PixelType,
    ) -> Result<Self, ImageBufferError> {
        let size = width * height * pixel_type.size();
        if buffer.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        if !pixel_type.is_aligned(&buffer) {
            return Err(ImageBufferError::InvalidBufferAlignment);
        }
        Ok(Self {
            width,
            height,
            buffer: BufferContainer::VecU8(buffer),
            pixel_type,
        })
    }

    /// Buffer with image pixels.
    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
        match &self.buffer {
            BufferContainer::VecU8(v) => v,
        }
    }

    /// Mutable buffer with image pixels.
    #[inline(always)]
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        match &mut self.buffer {
            BufferContainer::VecU8(ref mut v) => v.as_mut_slice(),
        }
    }

    #[inline(always)]
    pub fn view(&self) -> DynamicImageView {
        macro_rules! get_dynamic_image {
            ($img_type: expr) => {
                ($img_type(ImageView::from_buffer(self.width, self.height, self.buffer()).unwrap()))
            };
        }

        match self.pixel_type {
            PixelType::U8x4 => get_dynamic_image!(DynamicImageView::U8x4),
        }
    }

    #[inline(always)]
    pub fn view_mut(&mut self) -> DynamicImageViewMut {
        macro_rules! get_dynamic_image {
            ($img_type: expr) => {
                ($img_type(
                    ImageViewMut::from_buffer(self.width, self.height, self.buffer_mut()).unwrap(),
                ))
            };
        }

        match self.pixel_type {
            PixelType::U8x4 => get_dynamic_image!(DynamicImageViewMut::U8x4),
        }
    }
}

/// Generic image container for internal purposes.
pub(crate) struct InnerImage<'a, P>
where
    P: PixelExt,
{
    width: usize,
    height: usize,
    pixels: &'a mut [P],
}

impl<'a, P> InnerImage<'a, P>
where
    P: PixelExt,
{
    pub fn new(width: usize, height: usize, pixels: &'a mut [P]) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    #[inline(always)]
    pub fn dst_view(&mut self) -> ImageViewMut<P> {
        ImageViewMut::from_pixels(self.width, self.height, self.pixels).unwrap()
    }
}
