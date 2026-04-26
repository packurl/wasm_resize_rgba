use std::arch::wasm32::*;

use crate::pixels::U8x4;

#[inline]
#[target_feature(enable = "simd128")]
pub(crate) fn load_v128<T>(buf: &[T], index: usize) -> v128 {
    let ptr = unsafe { buf.get_unchecked(index..) }.as_ptr() as *const v128;
    unsafe { v128_load(ptr) }
}

#[inline]
#[target_feature(enable = "simd128")]
pub(crate) fn loadl_i64<T>(buf: &[T], index: usize) -> v128 {
    let p = unsafe { buf.get_unchecked(index..) }.as_ptr() as *const i64;
    let v = unsafe { p.read_unaligned() };
    i64x2(v, 0)
}

#[inline]
#[target_feature(enable = "simd128")]
pub(crate) fn ptr_i16_to_set1_i32(buf: &[i16], index: usize) -> v128 {
    let p = unsafe { buf.get_unchecked(index..) }.as_ptr() as *const i32;
    let v = unsafe { p.read_unaligned() };
    i32x4_splat(v)
}

#[inline]
#[target_feature(enable = "simd128")]
pub(crate) fn i32x4_extend_low_ptr_u8(buf: &[u8], index: usize) -> v128 {
    let ptr = unsafe { buf.get_unchecked(index..) }.as_ptr() as *const v128;
    let v = unsafe { v128_load(ptr) };
    u32x4_extend_low_u16x8(i16x8_extend_low_u8x16(v))
}

#[inline]
#[target_feature(enable = "simd128")]
pub(crate) fn i32x4_extend_low_ptr_u8x4(buf: &[U8x4], index: usize) -> v128 {
    let v: u32 = u32::from_le_bytes(unsafe { buf.get_unchecked(index) }.0);
    u32x4_extend_low_u16x8(i16x8_extend_low_u8x16(u32x4(v, 0, 0, 0)))
}

#[inline]
#[target_feature(enable = "simd128")]
pub(crate) fn i32x4_v128_from_u8(buf: &[u8], index: usize) -> v128 {
    let p = unsafe { buf.get_unchecked(index..) }.as_ptr() as *const i32;
    let v = unsafe { p.read_unaligned() };
    i32x4(v, 0, 0, 0)
}
