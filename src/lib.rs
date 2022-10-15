mod endian;
mod error;
mod macros;

use std::io::{Read, Write};

pub use error::BinaryError;
pub use macros::{Readable, Writeable};

pub use endian::*;

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

pub trait ReadExt: Read {
    /// Read a single value from a reader, e.g. a [`Cursor`].
    ///
    /// ### Example
    ///
    /// ```
    /// use std::io::Cursor;
    /// use binum::ReadExt;
    ///
    /// let mut r = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);
    /// let n = r.read_from::<u16, binum::BigEndian>().unwrap();
    /// assert_eq!(n, 17752);
    /// ```
    fn read_from<T: FromBytes, E: Endianness>(&mut self) -> BinaryReadResult<T> {
        let mut buf = vec![0; T::SIZE];

        match self.read_exact(&mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        };

        E::read::<T>(&buf)
    }

    /// Read multiple values from a reader, e.g. a [`Cursor`].
    ///
    /// ### Example
    ///
    /// ```
    /// use std::io::Cursor;
    /// use binum::ReadExt;
    ///
    /// let mut r = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);
    /// let n = r.read_multi::<u16, binum::BigEndian>(2).unwrap();
    /// assert_eq!(n[0], 17752);
    /// assert_eq!(n[1], 16717);
    /// ```
    fn read_multi<T: FromBytes, E: Endianness>(
        &mut self,
        nints: usize,
    ) -> BinaryReadResult<Vec<T>> {
        let mut buf = vec![0; T::SIZE * nints];

        match self.read_exact(&mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        }

        E::read_multi(&buf, nints)
    }
}

// Auto implement ReadExt for types which implement Read
impl<R: Read + ?Sized> ReadExt for R {}

pub trait WriteExt: Write {
    fn write_into<T: IntoBytes, E: Endianness>(&mut self, n: T) -> BinaryWriteResult {
        let mut buf = vec![0; T::SIZE];

        match E::write(n, &mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        };

        match self.write(&buf) {
            Ok(n) => Ok(n),
            Err(err) => Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
        }
    }

    fn write_multi<T: IntoBytes, E: Endianness>(&mut self, n: Vec<T>) -> BinaryWriteResult {
        let mut buf = vec![0; T::SIZE * n.len()];

        match E::write_multi(n, &mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        };

        match self.write(&buf) {
            Ok(n) => Ok(n),
            Err(err) => Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
        }
    }
}

// Auto implement WriteExt for types which implement Write
impl<W: Write + ?Sized> WriteExt for W {}

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
