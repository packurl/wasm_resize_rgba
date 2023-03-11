use crate::convolution::{self, Convolution, FilterType};
use crate::image::InnerImage;
use crate::pixels::PixelExt;
use crate::{
    DifferentTypesOfPixelsError, DynamicImageView, DynamicImageViewMut, ImageView, ImageViewMut,
};

/// SIMD extension of CPU.
/// Specific variants depends from target architecture.
/// Look at source code to see all available variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuExtensions {
    #[cfg(target_arch = "wasm32")]
    /// SIMD extension of Wasm32 architecture
    Simd128,
}

impl CpuExtensions {

}

impl Default for CpuExtensions {
    fn default() -> Self {
        Self::Simd128
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ResizeAlg {
    Convolution(FilterType),
}

impl Default for ResizeAlg {
    fn default() -> Self {
        Self::Convolution(FilterType::CatmullRom)
    }
}

/// Methods of this structure used to resize images.
#[derive(Default, Debug, Clone)]
pub struct Resizer {
    pub algorithm: ResizeAlg,
    cpu_extensions: CpuExtensions,
    convolution_buffer: Vec<u8>
}

impl Resizer {
    /// Creates instance of `Resizer`
    ///
    /// By default, instance of `Resizer` created with best CPU-extensions provided by your CPU.
    /// You can change this by use method [Resizer::set_cpu_extensions].
    pub fn new(algorithm: ResizeAlg) -> Self {
        Self {
            algorithm,
            ..Default::default()
        }
    }

    /// Resize source image to the size of destination image and save
    /// the result to the latter's pixel buffer.
    ///
    /// This method doesn't multiply source image and doesn't divide
    /// destination image by alpha channel.
    /// You must use [MulDiv](crate::MulDiv) for these actions.
    pub fn resize(
        &mut self,
        src_image: &DynamicImageView,
        dst_image: &mut DynamicImageViewMut,
    ) -> Result<(), DifferentTypesOfPixelsError> {
        match (src_image, dst_image) {
            (DynamicImageView::U8x4(src), DynamicImageViewMut::U8x4(dst)) => {
                self.resize_inner(src, dst);
            }
        }
        Ok(())
    }

    fn resize_inner<P>(&mut self, src_image: &ImageView<P>, dst_image: &mut ImageViewMut<P>)
    where
        P: Convolution,
    {
        match self.algorithm {
            ResizeAlg::Convolution(filter_type) => {
                let convolution_buffer = &mut self.convolution_buffer;
                resample_convolution(
                    src_image,
                    dst_image,
                    filter_type,
                    self.cpu_extensions,
                    convolution_buffer,
                )
            }
        }
    }
}

/// Create inner image container from part of given buffer.
/// Buffer may be expanded if it size is less than required for image.
fn get_temp_image_from_buffer<P: PixelExt>(
    buffer: &mut Vec<u8>,
    width: usize,
    height: usize,
) -> InnerImage<P> {
    let pixels_count = width * height;
    // Add pixel size as gap for alignment of resulted buffer.
    let buf_size = pixels_count * P::size() + P::size();
    if buffer.len() < buf_size {
        buffer.resize(buf_size, 0);
    }
    let pixels = unsafe { buffer.align_to_mut::<P>().1 };
    InnerImage::new(width, height, &mut pixels[0..pixels_count])
}

fn resample_convolution<P>(
    src_image: &ImageView<P>,
    dst_image: &mut ImageViewMut<P>,
    filter_type: FilterType,
    cpu_extensions: CpuExtensions,
    temp_buffer: &mut Vec<u8>,
) where
    P: Convolution,
{
    let src_width = src_image.width();
    let src_height = src_image.height();
    let dst_width = dst_image.width();
    let dst_height = dst_image.height();
    let (filter_fn, filter_support) = convolution::get_filter_func(filter_type);

    let need_horizontal = dst_width != src_width;
    let horiz_coeffs = need_horizontal.then(|| {
        convolution::precompute_coefficients(
            src_width,
            0.0,
            src_width as f64,
            dst_width,
            filter_fn,
            filter_support,
        )
    });

    let need_vertical = dst_height != src_height;
    let vert_coeffs = need_vertical.then(|| {
        convolution::precompute_coefficients(
            src_image.height(),
            0.0,
            src_height as f64,
            dst_height,
            filter_fn,
            filter_support,
        )
    });

    match (horiz_coeffs, vert_coeffs) {
        (Some(horiz_coeffs), Some(mut vert_coeffs)) => {
            let y_first = vert_coeffs.bounds[0].start;
            // Last used row in the source image
            let last_y_bound = vert_coeffs.bounds.last().unwrap();
            let y_last = last_y_bound.start + last_y_bound.size;
            let temp_height = y_last - y_first;
            let mut temp_image = get_temp_image_from_buffer(temp_buffer, dst_width, temp_height as usize);
            let mut tmp_dst_view = temp_image.dst_view();
            P::horiz_convolution(
                src_image,
                &mut tmp_dst_view,
                y_first,
                horiz_coeffs,
                cpu_extensions,
            );

            // Shift bounds for vertical pass
            vert_coeffs
                .bounds
                .iter_mut()
                .for_each(|b| b.start -= y_first);
            P::vert_convolution(
                &tmp_dst_view.into(),
                dst_image,
                0,
                vert_coeffs,
                cpu_extensions,
            );
        }
        (Some(horiz_coeffs), None) => {
            P::horiz_convolution(
                src_image,
                dst_image,
                0,
                horiz_coeffs,
                cpu_extensions,
            );
        }
        (None, Some(vert_coeffs)) => {
            P::vert_convolution(
                src_image,
                dst_image,
                0,
                vert_coeffs,
                cpu_extensions,
            );
        }
        _ => {}
    }
}
