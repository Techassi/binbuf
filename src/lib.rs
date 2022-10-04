mod endian;
mod error;
mod macros;

pub use error::BinaryError;
pub use macros::{Readable, Writeable};

pub use endian::*;

use error::BinaryErrorVariant;
use macros::from_and_into_bytes_trait_impl;

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

impl Endianness for BigEndian {
    fn read<T: FromBytes>(buf: &[u8]) -> BinaryReadResult<T> {
        if buf.len() < T::SIZE {
            return Err(BinaryError::new("Slice of bytes too short", T::ERR_VARIANT));
        }

        let n = T::from_be_bytes(&buf[..T::SIZE]);
        Ok(n)
    }

    fn read_multi<T: FromBytes>(buf: &[u8], nints: usize) -> BinaryReadResult<Vec<T>> {
        if buf.len() < T::SIZE * nints {
            return Err(BinaryError::new("Slice of bytes too short", T::ERR_VARIANT));
        }

        let mut v = Vec::with_capacity(nints);

        for i in 0..nints {
            match Self::read(&buf[i * T::SIZE..]) {
                Ok(n) => v.push(n),
                Err(err) => return Err(err),
            };
        }

        Ok(v)
    }

    fn write<T: IntoBytes>(n: T, buf: &mut [u8]) -> BinaryWriteResult {
        if buf.len() < T::SIZE {
            return Err(BinaryError::new("Buf too short", T::ERR_VARIANT));
        }

        let bytes = n.to_be_bytes();

        for (i, b) in bytes.into_iter().enumerate() {
            buf[i] = b;
        }

        Ok(T::SIZE)
    }

    fn write_multi<T: IntoBytes>(n: Vec<T>, buf: &mut [u8]) -> BinaryWriteResult {
        let required_len = n.len() * T::SIZE;
        if buf.len() > required_len {
            return Err(BinaryError::new("Buf too short", T::ERR_VARIANT));
        }

        for (i, n) in n.into_iter().enumerate() {
            match Self::write(n, &mut buf[i * T::SIZE..]) {
                Err(err) => return Err(err),
                _ => {}
            }
        }

        Ok(required_len)
    }
}
