#[inline(always)]
pub(crate) fn mul_div_255(a: u8, b: u8) -> u8 {
    let tmp = a as u32 * b as u32 + 128;
    (((tmp >> 8) + tmp) >> 8) as u8
}

#[cfg(not(target_arch = "wasm32"))]
const PRECISION: u32 = 8;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RECIP_ALPHA: [u32; 256] = recip_alpha_array(PRECISION);

#[cfg(not(target_arch = "wasm32"))]
const fn recip_alpha_array(precision: u32) -> [u32; 256] {
    let mut res = [0; 256];
    let scale = 1 << (precision + 1);
    let scaled_max = 255 * scale;
    let mut i: usize = 1;
    while i < 256 {
        res[i] = ((scaled_max / i as u32) + 1) >> 1;
        i += 1;
    }
    res
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub(crate) fn div_and_clip(v: u8, recip_alpha: u32) -> u8 {
    ((v as u32 * recip_alpha) >> PRECISION).min(255) as u8
}
