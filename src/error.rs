
use thiserror::Error;

pub type Result<T> = core::result::Result<T, InterpError>;

#[derive(Error, Debug)]
pub enum InterpError{
    #[error("Data contains a nan or inf")]
    InvalidData,

    #[error("Interpolation requested out of range. \
             Point is to the left of all data. \
             point: {point}; min: {min}")]
    OutOfRangeLeft { point: String, min: String },

    #[error("Interpolation requested out of range. \
             Point is to the left of all data. \
             point: {point}; min: {max}")]
    OutOfRangeRight { point: String, max: String },

    #[error("U -> T try_from conversion failed")]
    ValueFromTFailed
}