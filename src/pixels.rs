//! Contains types of pixels.
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PixelType {
    U8x4
}

impl PixelType {
    pub(crate) fn size(&self) -> usize { 4 }

    /// Returns `true` if given buffer is aligned by the alignment of pixel.
    pub(crate) fn is_aligned(&self, buffer: &[u8]) -> bool {
        match self {
            Self::U8x4 => unsafe { buffer.align_to::<U8x4>().0.is_empty() },
        }
    }
}

pub trait GetCount {
    fn count() -> usize;
}

/// Generic type to represent the number of components in single pixel.
pub struct Count<const N: usize>;

impl<const N: usize> GetCount for Count<N> {
    #[inline(always)]
    fn count() -> usize {
        N
    }
}

pub trait GetCountOfValues {
    fn count_of_values() -> usize;
}

/// Generic type to represent the number of available values for a single pixel component.
pub struct Values<const N: usize>;

impl<const N: usize> GetCountOfValues for Values<N> {
    fn count_of_values() -> usize {
        N
    }
}

/// Information about one component of pixel.
pub trait PixelComponent
where
    Self: Sized + Copy + Debug + PartialEq + 'static,
{
    /// Type that provides information about a count of
    /// available values of one pixel's component
    type CountOfComponentValues: GetCountOfValues;

    /// Count of available values of one pixel's component
    fn count_of_values() -> usize {
        Self::CountOfComponentValues::count_of_values()
    }
}

impl PixelComponent for u8 {
    type CountOfComponentValues = Values<256>;
}
impl PixelComponent for u16 {
    type CountOfComponentValues = Values<65536>;
}
impl PixelComponent for i32 {
    type CountOfComponentValues = Values<0>;
}
impl PixelComponent for f32 {
    type CountOfComponentValues = Values<0>;
}

pub trait IntoPixelType {
    fn pixel_type() -> PixelType;
}

/// Additional information about pixel type.
pub trait PixelExt
where
    Self: Copy + Clone + Sized + Debug + PartialEq + IntoPixelType,
{
    /// Type of pixel components
    type Component: PixelComponent;
    /// Type that provides information about a count of pixel's components
    type CountOfComponents: GetCount;

    /// Count of pixel's components
    fn count_of_components() -> usize {
        Self::CountOfComponents::count()
    }

    /// Count of available values of one pixel's component
    fn count_of_component_values() -> usize {
        Self::Component::count_of_values()
    }

    /// Size of pixel in bytes
    ///
    /// Example:
    /// ```
    /// # use fast_image_resize::pixels::{U8x2, U8x3, U8, PixelExt};
    /// assert_eq!(U8x3::size(), 3);
    /// assert_eq!(U8x2::size(), 2);
    /// assert_eq!(U8::size(), 1);
    /// ```
    fn size() -> usize {
        size_of::<Self>()
    }

    /// Create slice of pixel's components from slice of pixels
    fn components(buf: &[Self]) -> &[Self::Component] {
        let size = buf.len() * Self::count_of_components();
        let components_ptr = buf.as_ptr() as *const Self::Component;
        unsafe { slice::from_raw_parts(components_ptr, size) }
    }

    /// Create mutable slice of pixel's components from mutable slice of pixels
    fn components_mut(buf: &mut [Self]) -> &mut [Self::Component] {
        let size = buf.len() * Self::count_of_components();
        let components_ptr = buf.as_mut_ptr() as *mut Self::Component;
        unsafe { slice::from_raw_parts_mut(components_ptr, size) }
    }
}

/// Generic type of pixel.
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Pixel<T, C, const COUNT_OF_COMPONENTS: usize>(
    pub T,
    PhantomData<[C; COUNT_OF_COMPONENTS]>,
)
where
    T: Sized + Copy + Clone + PartialEq + 'static,
    C: PixelComponent;

impl<T, C, const COUNT_OF_COMPONENTS: usize> Pixel<T, C, COUNT_OF_COMPONENTS>
where
    T: Sized + Copy + Clone + PartialEq + 'static,
    C: PixelComponent,
{
    #[cfg(target_arch = "wasm32")]
    #[inline(always)]
    pub const fn new(v: T) -> Self {
        Self(v, PhantomData)
    }
}

impl<T, C, const COUNT_OF_COMPONENTS: usize> PixelExt for Pixel<T, C, COUNT_OF_COMPONENTS>
where
    Self: IntoPixelType + Debug,
    T: Sized + Copy + Clone + PartialEq + 'static,
    C: PixelComponent,
{
    type Component = C;
    type CountOfComponents = Count<COUNT_OF_COMPONENTS>;
}

macro_rules! pixel_struct {
    ($name:ident, $type:tt, $comp_type:tt, $comp_count:literal, $pixel_type:expr, $doc:expr) => {
        #[doc = $doc]
        pub type $name = Pixel<$type, $comp_type, $comp_count>;

        impl IntoPixelType for $name {
            fn pixel_type() -> PixelType {
                $pixel_type
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let components_ptr = self as *const _ as *const $comp_type;
                let components: &[$comp_type] =
                    unsafe { slice::from_raw_parts(components_ptr, $comp_count) };
                write!(f, "{}{:?}", stringify!($name), components)
            }
        }
    };
}

pixel_struct!(
    U8x4,
    [u8; 4],
    u8,
    4,
    PixelType::U8x4,
    "Four bytes per pixel (RGBA8, RGBx8, CMYK8 and other)"
);

pub trait IntoPixelComponent<Out: PixelComponent>
where
    Self: PixelComponent,
{
    fn into_component(self) -> Out;
}

impl<C: PixelComponent> IntoPixelComponent<C> for C {
    fn into_component(self) -> C {
        self
    }
}
