use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::slice;

use crate::pixels::PixelExt;
use crate::ImageBufferError;

/// Generic immutable image view.
#[derive(Debug, Clone)]
pub struct ImageView<'a, P>
where
    P: PixelExt,
{
    width: usize,
    height: usize,
    rows: Vec<&'a [P]>,
}

impl<'a, P> ImageView<'a, P>
where
    P: PixelExt,
{

    pub fn from_buffer(
        width: usize,
        height: usize,
        buffer: &'a [u8],
    ) -> Result<Self, ImageBufferError> {
        let size = (width * height) * P::size();
        if buffer.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        let rows_count = height;
        let pixels = align_buffer_to(buffer)?;
        let rows = pixels
            .chunks_exact(width)
            .take(rows_count)
            .collect();
        Ok(Self {
            width,
            height,
            rows,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[cfg(target_arch = "wasm32")]
    #[inline(always)]
    pub(crate) fn iter_4_rows<'s>(
        &'s self,
        start_y: u32,
        max_y: u32,
    ) -> impl Iterator<Item = [&'a [P]; 4]> + 's {
        let start_y = start_y as usize;
        let max_y = (max_y as usize).min(self.height());
        let rows = self.rows.get(start_y..max_y).unwrap_or(&[]);
        rows.chunks_exact(4).map(|rows| match *rows {
            [r0, r1, r2, r3] => [r0, r1, r2, r3],
            _ => unreachable!(),
        })
    }

    #[cfg(target_arch = "wasm32")]
    #[inline(always)]
    pub(crate) fn iter_2_rows<'s>(
        &'s self,
        start_y: u32,
        max_y: u32,
    ) -> impl Iterator<Item = [&'a [P]; 2]> + 's {
        let start_y = start_y as usize;
        let max_y = (max_y as usize).min(self.height);
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

    #[cfg(target_arch = "wasm32")]
    #[inline(always)]
    pub(crate) fn get_row(&self, y: usize) -> Option<&'a [P]> {
        self.rows.get(y).copied()
    }
}

/// Generic mutable image view.
#[derive(Debug)]
pub struct ImageViewMut<'a, P>
where
    P: PixelExt,
{
    width: usize,
    height: usize,
    rows: Vec<&'a mut [P]>,
}

impl<'a, P> ImageViewMut<'a, P>
where
    P: PixelExt,
{

    pub fn from_buffer(
        width: usize,
        height: usize,
        buffer: &'a mut [u8],
    ) -> Result<Self, ImageBufferError> {
        let size = width * height * P::size();
        if buffer.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        let rows_count = height;
        let pixels = align_buffer_to_mut(buffer)?;
        let rows = pixels
            .chunks_exact_mut(width)
            .take(rows_count)
            .collect();
        Ok(Self {
            width,
            height,
            rows,
        })
    }

    pub fn from_pixels(
        width: usize,
        height: usize,
        pixels: &'a mut [P],
    ) -> Result<Self, ImageBufferError> {
        let size = width * height;
        if pixels.len() < size {
            return Err(ImageBufferError::InvalidBufferSize);
        }
        let rows_count = height;
        let rows = pixels
            .chunks_exact_mut(width)
            .take(rows_count)
            .collect();
        Ok(Self {
            width,
            height,
            rows,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub(crate) fn iter_rows_mut(&mut self) -> slice::IterMut<&'a mut [P]> {
        self.rows.iter_mut()
    }

    #[cfg(target_arch = "wasm32")]
    #[inline(always)]
    pub(crate) fn iter_4_rows_mut<'s>(
        &'s mut self,
    ) -> impl Iterator<Item = [&'s mut &'a mut [P]; 4]> {
        self.rows.chunks_exact_mut(4).map(|rows| match rows {
            [a, b, c, d] => [a, b, c, d],
            _ => unreachable!(),
        })
    }

    #[cfg(target_arch = "wasm32")]
    #[inline(always)]
    pub(crate) fn get_row_mut<'s>(&'s mut self, y: usize) -> Option<&'s mut &'a mut [P]> {
        self.rows.get_mut(y)
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

