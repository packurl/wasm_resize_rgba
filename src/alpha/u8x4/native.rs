use crate::alpha::common::mul_div_255;
use crate::pixels::U8x4;

#[inline(always)]
pub(crate) fn multiply_alpha_row(src_row: &[U8x4], dst_row: &mut [U8x4]) {
    for (src_pixel, dst_pixel) in src_row.iter().zip(dst_row) {
        *dst_pixel = multiply_alpha_pixel(*src_pixel);
    }
}

#[inline(always)]
pub(crate) fn multiply_alpha_row_inplace(row: &mut [U8x4]) {
    for pixel in row.iter_mut() {
        *pixel = multiply_alpha_pixel(*pixel);
    }
}

#[inline(always)]
fn multiply_alpha_pixel(mut pixel: U8x4) -> U8x4 {
    let components: [u8; 4] = pixel.0.to_le_bytes();
    let alpha = components[3];
    pixel.0 = u32::from_le_bytes([
        mul_div_255(components[0], alpha),
        mul_div_255(components[1], alpha),
        mul_div_255(components[2], alpha),
        alpha,
    ]);
    pixel
}
