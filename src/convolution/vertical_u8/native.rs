use crate::convolution::optimisations;
#[cfg(not(target_arch = "wasm32"))]
use crate::convolution::Coefficients;
#[cfg(not(target_arch = "wasm32"))]
use crate::image_view::ImageViewMut;
use crate::pixels::PixelExt;
use crate::ImageView;

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub(crate) fn vert_convolution<T>(
    src_image: &ImageView<T>,
    dst_image: &mut ImageViewMut<T>,
    offset: u32,
    coeffs: Coefficients,
) where
    T: PixelExt<Component = u8>,
{
    let normalizer = optimisations::Normalizer16::new(coeffs);
    let coefficients_chunks = normalizer.normalized_chunks();
    let precision = normalizer.precision();
    let initial = 1 << (precision - 1);
    let src_x_initial = offset as usize * T::count_of_components();

    let dst_rows = dst_image.iter_rows_mut();
    let coeffs_chunks_iter = coefficients_chunks.into_iter();
    for (coeffs_chunk, dst_row) in coeffs_chunks_iter.zip(dst_rows) {
        let first_y_src = coeffs_chunk.start;
        let ks = coeffs_chunk.values;
        let mut x_src = src_x_initial;
        let dst_components = T::components_mut(dst_row);

        let (_, dst_chunks, tail) = unsafe { dst_components.align_to_mut::<[u8; 32]>() };
        x_src = convolution_by_chunks(
            src_image,
            &normalizer,
            initial,
            dst_chunks,
            x_src,
            first_y_src,
            ks,
        );
        if tail.is_empty() {
            continue;
        }

        let (_, dst_chunks, tail) = unsafe { tail.align_to_mut::<[u8; 16]>() };
        x_src = convolution_by_chunks(
            src_image,
            &normalizer,
            initial,
            dst_chunks,
            x_src,
            first_y_src,
            ks,
        );
        if tail.is_empty() {
            continue;
        }

        let (_, dst_chunks, tail) = unsafe { tail.align_to_mut::<[u8; 8]>() };
        x_src = convolution_by_chunks(
            src_image,
            &normalizer,
            initial,
            dst_chunks,
            x_src,
            first_y_src,
            ks,
        );
        if tail.is_empty() {
            continue;
        }

        let (_, dst_chunks, tail) = unsafe { tail.align_to_mut::<[u8; 4]>() };
        x_src = convolution_by_chunks(
            src_image,
            &normalizer,
            initial,
            dst_chunks,
            x_src,
            first_y_src,
            ks,
        );

        if !tail.is_empty() {
            convolution_by_u8(
                src_image,
                &normalizer,
                initial,
                tail,
                x_src,
                first_y_src,
                ks,
            );
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
fn convolution_by_chunks<T, const CHUNK_SIZE: usize>(
    src_image: &ImageView<T>,
    normalizer: &optimisations::Normalizer16,
    initial: i32,
    dst_chunks: &mut [[u8; CHUNK_SIZE]],
    mut x_src: usize,
    first_y_src: u32,
    ks: &[i16],
) -> usize
    where
        T: PixelExt<Component = u8>,
{
    for dst_chunk in dst_chunks {
        let mut ss = [initial; CHUNK_SIZE];
        let src_rows = src_image.iter_rows(first_y_src);

        foreach_with_pre_reading(
            ks.iter().zip(src_rows),
            |(&k, src_row)| {
                let src_ptr = src_row.as_ptr() as *const u8;
                let src_chunk = unsafe {
                    let ptr = src_ptr.add(x_src) as *const [u8; CHUNK_SIZE];
                    ptr.read_unaligned()
                };
                (src_chunk, k)
            },
            |(src_chunk, k)| {
                for (s, c) in ss.iter_mut().zip(src_chunk) {
                    *s += c as i32 * (k as i32);
                }
            },
        );

        for (i, s) in ss.iter().copied().enumerate() {
            dst_chunk[i] = unsafe { normalizer.clip(s) };
        }
        x_src += CHUNK_SIZE;
    }
    x_src
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub(crate) fn foreach_with_pre_reading<D, I>(
    mut iter: impl Iterator<Item = I>,
    mut read_data: impl FnMut(I) -> D,
    mut process_data: impl FnMut(D),
) {
    let mut next_data: D;
    if let Some(src) = iter.next() {
        next_data = read_data(src);
        for src in iter {
            let data = next_data;
            next_data = read_data(src);
            process_data(data);
        }
        process_data(next_data);
    }
}

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
            let src_component = unsafe { *src_ptr.add(x_src) };
            ss += src_component as i32 * (k as i32);
        }
        *dst_component = unsafe { normalizer.clip(ss) };
        x_src += 1
    }
    x_src
}
