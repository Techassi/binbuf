mod error;
mod io;
mod macros;

pub use error::BinaryError;
pub use io::*;
pub use macros::{Readable, Writeable};

use error::BinaryErrorVariant;
use macros::{endianness_impl, from_and_into_bytes_trait_impl};

/////////////
// Prelude //
/////////////

pub mod prelude {
    pub use crate::{BigEndian, Endianness, LittleEndian, ReadExt, WriteExt};
}

//////////////////////
// Public constants //
//////////////////////

pub const BITS_IN_BYTE: u32 = 8;
pub const U16_BYTES: usize = (u16::BITS / BITS_IN_BYTE) as usize;
pub const U32_BYTES: usize = (u32::BITS / BITS_IN_BYTE) as usize;
pub const U64_BYTES: usize = (u64::BITS / BITS_IN_BYTE) as usize;
pub const U128_BYTES: usize = (u128::BITS / BITS_IN_BYTE) as usize;

// Public types

pub type BinaryReadResult<T> = Result<T, BinaryError>;
pub type BinaryWriteResult = Result<usize, BinaryError>;

/////////////////////////////////////////////////
// FromBytes + IntoBytes traits and their impl //
/////////////////////////////////////////////////

/// The [`FromBytes`] trait defines methods to create types (which implement
/// this trait) from little and big endian representations. [`FromBytes::SIZE`]
/// specifies the binary size (in bytes / octets) of the type.
pub trait FromBytes {
    const ERR_VARIANT: BinaryErrorVariant;
    const SIZE: usize;

    /// Create [`Self`] from bytes in little endian representation.
    fn from_le_bytes(bytes: &[u8]) -> Self;

    /// Create [`Self`] from bytes in big endian representation.
    fn from_be_bytes(bytes: &[u8]) -> Self;
}

/// The [`IntoBytes`] trait defines methods to create byte vectors of types
/// (which implement this trait) in little and big endian representation.
/// [`IntoBytes::SIZE`] specifies the binary size (in bytes / octets) of the
/// type.
pub trait IntoBytes {
    const ERR_VARIANT: BinaryErrorVariant;
    const SIZE: usize;

    fn to_le_bytes(self) -> Vec<u8>;
    fn to_be_bytes(self) -> Vec<u8>;
}

from_and_into_bytes_trait_impl!(u16, U16_BYTES, BinaryErrorVariant::U16);
from_and_into_bytes_trait_impl!(u32, U32_BYTES, BinaryErrorVariant::U32);
from_and_into_bytes_trait_impl!(u64, U64_BYTES, BinaryErrorVariant::U64);
from_and_into_bytes_trait_impl!(u128, U128_BYTES, BinaryErrorVariant::U128);

///////////////////////////////////////////////////////////////////
// Endianness trait and the impls for BigEndian and LittleEndian //
///////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////
// Public generic read and write functions //
/////////////////////////////////////////////

/// Read an unsigned integer of type `T` from `buf` with [`Endianness`]. This
/// function returns either the integer of type `T` or an error indicating the
/// buf was too short.
///
/// ### Example
///
/// ```
/// use binum::prelude::*;
///
/// let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let n = binum::read::<u16, BigEndian>(&b).unwrap();
///
/// assert_eq!(n, 17752);
/// ```
pub fn read<T: FromBytes, E: Endianness>(buf: &[u8]) -> BinaryReadResult<T> {
    E::read(buf)
}

pub fn read_multi<T: FromBytes, E: Endianness>(
    buf: &[u8],
    nints: usize,
) -> BinaryReadResult<Vec<T>> {
    E::read_multi(buf, nints)
}

pub fn write<T: IntoBytes, E: Endianness>(n: T, buf: &mut [u8]) -> BinaryWriteResult {
    E::write(n, buf)
}

pub fn write_multi<T: IntoBytes, E: Endianness>(n: Vec<T>, buf: &mut [u8]) -> BinaryWriteResult {
    E::write_multi(n, buf)
}
