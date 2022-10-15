mod error;
mod io;
mod macros;

pub use error::BinaryError;
pub use io::*;
pub use macros::{Readable, Writeable};

use error::BinaryErrorVariant;
use macros::{endianness_impl, from_and_into_bytes_trait_impl};

pub const BITS_IN_BYTE: u32 = 8;
pub const U16_BYTES: usize = (u16::BITS / BITS_IN_BYTE) as usize;
pub const U32_BYTES: usize = (u32::BITS / BITS_IN_BYTE) as usize;
pub const U64_BYTES: usize = (u64::BITS / BITS_IN_BYTE) as usize;

pub type BinaryReadResult<T> = Result<T, BinaryError>;
pub type BinaryWriteResult = Result<usize, BinaryError>;

pub trait FromBytes {
    const ERR_VARIANT: BinaryErrorVariant;
    const SIZE: usize;

    fn from_le_bytes(bytes: &[u8]) -> Self;
    fn from_be_bytes(bytes: &[u8]) -> Self;
}

pub trait IntoBytes {
    const ERR_VARIANT: BinaryErrorVariant;
    const SIZE: usize;

    fn to_le_bytes(self) -> Vec<u8>;
    fn to_be_bytes(self) -> Vec<u8>;
}

from_and_into_bytes_trait_impl!(u16, U16_BYTES, BinaryErrorVariant::U16);
from_and_into_bytes_trait_impl!(u32, U32_BYTES, BinaryErrorVariant::U32);
from_and_into_bytes_trait_impl!(u64, U64_BYTES, BinaryErrorVariant::U64);

pub trait Endianness {
    fn read<T: FromBytes>(buf: &[u8]) -> BinaryReadResult<T>;
    fn read_multi<T: FromBytes>(buf: &[u8], nints: usize) -> BinaryReadResult<Vec<T>>;

    fn write<T: IntoBytes>(n: T, buf: &mut [u8]) -> BinaryWriteResult;
    fn write_multi<T: IntoBytes>(n: Vec<T>, buf: &mut [u8]) -> BinaryWriteResult;
}

pub struct BigEndian {}
endianness_impl!(BigEndian, from_be_bytes, to_be_bytes);

pub struct LittleEndian {}
endianness_impl!(LittleEndian, from_le_bytes, to_le_bytes);
