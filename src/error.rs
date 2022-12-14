use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum BufferError {
    #[error("Unsupported endianness")]
    UnsupportedEndianness,

    #[error("Max buffer length overflow")]
    MaxLengthOverflow,

    #[error("Invalid buffer jump index")]
    InvalidJumpIndex,

    // This indicates that somewhere in the buffer the reader encountered
    // invalid / unexpected data.
    #[error("Invalid data")]
    InvalidData,

    #[error("Buffer too short")]
    BufTooShort,
}
