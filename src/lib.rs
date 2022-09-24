mod endian;
mod error;
mod iter;
mod put;
mod read;
mod seek;

pub mod macros;

pub use error::BinaryError;

pub use endian::*;
pub use iter::*;
pub use put::*;
pub use read::*;
pub use seek::*;

pub const BITS_IN_OCTET: u32 = 8;
pub const U16_OCTETS: usize = (u16::BITS / BITS_IN_OCTET) as usize;
pub const U32_OCTETS: usize = (u32::BITS / BITS_IN_OCTET) as usize;
pub const U64_OCTETS: usize = (u64::BITS / BITS_IN_OCTET) as usize;

pub type BinaryReadResult<T> = Result<T, BinaryError>;
pub type BinaryWriteResult = Result<usize, BinaryError>;
