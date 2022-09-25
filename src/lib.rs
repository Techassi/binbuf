mod endian;
mod error;

pub mod macros;
pub mod read;
pub mod write;

pub use error::BinaryError;

pub use endian::*;

pub const BITS_IN_OCTET: u32 = 8;
pub const U16_OCTETS: usize = (u16::BITS / BITS_IN_OCTET) as usize;
pub const U32_OCTETS: usize = (u32::BITS / BITS_IN_OCTET) as usize;
pub const U64_OCTETS: usize = (u64::BITS / BITS_IN_OCTET) as usize;

pub type BinaryReadResult<T> = Result<T, BinaryError>;
pub type BinaryWriteResult = Result<usize, BinaryError>;
