//! Error type definitions

pub type PLYResult<T> = Result<T, PLYError>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// PLY Error type
pub enum PLYError {
    MissmatchDataType,
    UnknownPLYTypeIdentifier,
    ParseFromStrErr,
    TryIntoUsizeEr,
    TypeConversionFail,
    PropertyLengthErr,
}
