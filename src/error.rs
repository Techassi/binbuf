use thiserror::Error;

use crate::SupportedEndianness;

#[derive(Debug, PartialEq, Error)]
pub enum BufferError {
    /// This indicates that the type does not support reading / writing with
    /// the requested endianness.
    #[error("Unsupported endianness, only supports: {0}")]
    UnsupportedEndianness(SupportedEndianness),

    #[error("Invalid jump index. Jumping beyond the current offset is not permitted")]
    InvalidJumpIndex,

    /// Some associated functions of `ReadBuffer` provide a way to specify a
    /// maximum length the variable length data can have. If this max length
    /// is exceeded this error is returned.
    #[error("Max buffer length overflow")]
    MaxLengthOverflow,

    /// This indicates that somewhere in the buffer the reader encountered
    /// invalid / unexpected data.
    #[error("Invalid data")]
    InvalidData,

    /// This indicates that the buffer is too short to read the requested
    /// amount of bytes.
    #[error("Buffer too short")]
    BufferTooShort,

    /// This indicates that the caller encountered a custom error, which cannot
    /// be mapped to a different error. This acts as a "fallback" to still
    /// return the error message to callers.
    #[error("Other error: {0}")]
    Other(String),
}
