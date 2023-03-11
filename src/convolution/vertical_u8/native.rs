use crate::convolution::optimisations;
use crate::pixels::PixelExt;
use crate::ImageView;

#[inline(always)]
pub(crate) fn convolution_by_u8<T>(
    src_image: &ImageView<T>,
    normalizer: &optimisations::Normalizer16,
    initial: i32,
    dst_components: &mut [u8],
    mut x_src: usize,
    first_y_src: u32,
    ks: &[i16],
) -> usize
    where
        T: PixelExt<Component = u8>,
{
    for dst_component in dst_components {
        let mut ss = initial;
        let src_rows = src_image.iter_rows(first_y_src);
        for (&k, src_row) in ks.iter().zip(src_rows) {
            let src_ptr = src_row.as_ptr() as *const u8;
            let src_component = unsafe { *src_ptr.add(x_src as usize) };
            ss += src_component as i32 * (k as i32);
        }
        *dst_component = unsafe { normalizer.clip(ss) };
        x_src += 1
    }
    x_src
}
