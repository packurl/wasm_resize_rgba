#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageBufferError {
    InvalidBufferSize,
    InvalidBufferAlignment,
}

#[derive(Debug, Clone, Copy)]
pub struct DifferentTypesOfPixelsError;

#[derive(Debug, Clone, Copy)]
pub struct DifferentDimensionsError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingError {
    DifferentDimensions,
}

impl From<DifferentDimensionsError> for MappingError {
    fn from(_: DifferentDimensionsError) -> Self {
        MappingError::DifferentDimensions
    }
}
