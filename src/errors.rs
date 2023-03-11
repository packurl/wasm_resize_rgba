#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageBufferError {
    InvalidBufferSize,
    InvalidBufferAlignment,
}

#[derive(Debug, Clone, Copy)]
pub struct DifferentTypesOfPixelsError;

#[derive(Debug, Clone, Copy)]
pub struct DifferentDimensionsError;
