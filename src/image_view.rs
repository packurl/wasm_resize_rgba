use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::num::NonZeroU32;
use std::slice;

use crate::pixels::PixelExt;
use crate::{DifferentDimensionsError, ImageBufferError};

/// Parameters of crop box that may be used with [`ImageView`]
/// and [`DynamicImageView`](crate::DynamicImageView)
#[derive(Debug, Clone, Copy)]
pub struct CropBox {
    pub left: u32,
    pub top: u32,
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

/// Generic immutable image view.
#[derive(Debug, Clone)]
pub struct ImageView<'a, P>
where
    P: PixelExt,
{
    width: NonZeroU32,
    height: NonZeroU32,
    crop_box: CropBox,
    rows: Vec<&'a [P]>,
}

impl<'a, P> ImageView<'a, P>
where
    P: PixelExt,
{

    pub fn from_buffer(
        width: NonZeroU32,
        height: NonZeroU32,
        buffer: &'a [u8],
    ) -> Result<Self, ImageBufferError> {
        let size = (width.get() * height.get()) as usize * P::size();
        if buffer.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        let rows_count = height.get() as usize;
        let pixels = align_buffer_to(buffer)?;
        let rows = pixels
            .chunks_exact(width.get() as usize)
            .take(rows_count)
            .collect();
        Ok(Self {
            width,
            height,
            crop_box: CropBox {
                left: 0,
                top: 0,
                width,
                height,
            },
            rows,
        })
    }

    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    pub fn height(&self) -> NonZeroU32 {
        self.height
    }

    pub fn crop_box(&self) -> CropBox {
        self.crop_box
    }

    #[inline(always)]
    pub(crate) fn iter_4_rows<'s>(
        &'s self,
        start_y: u32,
        max_y: u32,
    ) -> impl Iterator<Item = [&'a [P]; 4]> + 's {
        let start_y = start_y as usize;
        let max_y = max_y.min(self.height.get()) as usize;
        let rows = self.rows.get(start_y..max_y).unwrap_or(&[]);
        rows.chunks_exact(4).map(|rows| match *rows {
            [r0, r1, r2, r3] => [r0, r1, r2, r3],
            _ => unreachable!(),
        })
    }

    #[inline(always)]
    pub(crate) fn iter_2_rows<'s>(
        &'s self,
        start_y: u32,
        max_y: u32,
    ) -> impl Iterator<Item = [&'a [P]; 2]> + 's {
        let start_y = start_y as usize;
        let max_y = max_y.min(self.height.get()) as usize;
        let rows = self.rows.get(start_y..max_y).unwrap_or(&[]);
        rows.chunks_exact(2).map(|rows| match *rows {
            [r0, r1] => [r0, r1],
            _ => unreachable!(),
        })
    }

    #[inline(always)]
    pub(crate) fn iter_rows<'s>(&'s self, start_y: u32) -> impl Iterator<Item = &'a [P]> + 's {
        let start_y = start_y as usize;
        let rows = self.rows.get(start_y..).unwrap_or(&[]);
        rows.iter().copied()
    }

    #[inline(always)]
    pub(crate) fn get_row(&self, y: u32) -> Option<&'a [P]> {
        self.rows.get(y as usize).copied()
    }


    #[inline(always)]
    pub(crate) fn iter_cropped_rows<'s>(&'s self) -> impl Iterator<Item = &'a [P]> + 's {
        let first_row = self.crop_box.top as usize;
        let last_row = first_row + self.crop_box.height.get() as usize;
        let rows = unsafe { self.rows.get_unchecked(first_row..last_row) };

        let first_col = self.crop_box.left as usize;
        let last_col = first_col + self.crop_box.width.get() as usize;
        rows.iter()
            // Safety guaranteed by method 'set_crop_box'
            .map(move |row| unsafe { row.get_unchecked(first_col..last_col) })
    }
}

/// Generic mutable image view.
#[derive(Debug)]
pub struct ImageViewMut<'a, P>
where
    P: PixelExt,
{
    width: NonZeroU32,
    height: NonZeroU32,
    rows: Vec<&'a mut [P]>,
}

impl<'a, P> ImageViewMut<'a, P>
where
    P: PixelExt,
{

    pub fn from_buffer(
        width: NonZeroU32,
        height: NonZeroU32,
        buffer: &'a mut [u8],
    ) -> Result<Self, ImageBufferError> {
        let size = (width.get() * height.get()) as usize * P::size();
        if buffer.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        let rows_count = height.get() as usize;
        let pixels = align_buffer_to_mut(buffer)?;
        let rows = pixels
            .chunks_exact_mut(width.get() as usize)
            .take(rows_count)
            .collect();
        Ok(Self {
            width,
            height,
            rows,
        })
    }

    pub fn from_pixels(
        width: NonZeroU32,
        height: NonZeroU32,
        pixels: &'a mut [P],
    ) -> Result<Self, ImageBufferError> {
        let size = (width.get() * height.get()) as usize;
        if pixels.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        let rows_count = height.get() as usize;
        let rows = pixels
            .chunks_exact_mut(width.get() as usize)
            .take(rows_count)
            .collect();
        Ok(Self {
            width,
            height,
            rows,
        })
    }

    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    pub fn height(&self) -> NonZeroU32 {
        self.height
    }

    #[inline(always)]
    pub(crate) fn iter_rows_mut(&mut self) -> slice::IterMut<&'a mut [P]> {
        self.rows.iter_mut()
    }

    #[inline(always)]
    pub(crate) fn iter_4_rows_mut<'s>(
        &'s mut self,
    ) -> impl Iterator<Item = [&'s mut &'a mut [P]; 4]> {
        self.rows.chunks_exact_mut(4).map(|rows| match rows {
            [a, b, c, d] => [a, b, c, d],
            _ => unreachable!(),
        })
    }

    #[inline(always)]
    pub(crate) fn get_row_mut<'s>(&'s mut self, y: u32) -> Option<&'s mut &'a mut [P]> {
        self.rows.get_mut(y as usize)
    }

    /// Copy pixels from src_view.
    pub(crate) fn copy_from_view(
        &mut self,
        src_view: &ImageView<P>,
    ) -> Result<(), DifferentDimensionsError> {
        let src_crop_box = src_view.crop_box();
        if self.width != src_crop_box.width || self.height != src_crop_box.height {
            return Err(DifferentDimensionsError);
        }
        self.rows
            .iter_mut()
            .zip(src_view.iter_cropped_rows())
            .for_each(|(d, s)| d.copy_from_slice(s));
        Ok(())
    }
}

impl<'a, P> From<ImageViewMut<'a, P>> for ImageView<'a, P>
where
    P: PixelExt,
{
    fn from(view: ImageViewMut<'a, P>) -> Self {
        let rows = {
            let mut old_rows = ManuallyDrop::new(view.rows);
            let (ptr, length, capacity) =
                (old_rows.as_mut_ptr(), old_rows.len(), old_rows.capacity());
            unsafe { Vec::from_raw_parts(ptr as *mut &[P], length, capacity) }
        };
        ImageView {
            width: view.width,
            height: view.height,
            crop_box: CropBox {
                left: 0,
                top: 0,
                width: view.width,
                height: view.height,
            },
            rows,
        }
    }
}

fn align_buffer_to<T>(buffer: &[u8]) -> Result<&[T], ImageBufferError> {
    let (head, pixels, _) = unsafe { buffer.align_to::<T>() };
    if !head.is_empty() {
        return Err(ImageBufferError::InvalidBufferAlignment);
    }
    Ok(pixels)
}

fn align_buffer_to_mut<T>(buffer: &mut [u8]) -> Result<&mut [T], ImageBufferError> {
    let (head, pixels, _) = unsafe { buffer.align_to_mut::<T>() };
    if !head.is_empty() {
        return Err(ImageBufferError::InvalidBufferAlignment);
    }
    Ok(pixels)
}

