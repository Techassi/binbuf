use std::fmt::Display;

use crate::{
    read::{FromBuffer, ReadBufferResult, ToReadBuffer},
    write::{IntoBuffer, ToWriteBuffer, WriteBufferResult},
};

pub mod error;
pub mod macros;
pub mod read;
pub mod write;

mod impls;

#[derive(Debug, PartialEq)]
pub enum SupportedEndianness {
    BigEndian,
    LittleEndian,
    Both,
}

impl Display for SupportedEndianness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedEndianness::BigEndian => write!(f, "big endian"),
            SupportedEndianness::LittleEndian => write!(f, "little endian"),
            SupportedEndianness::Both => write!(f, "big+little endian"),
        }
    }
}

pub trait Endianness {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool;

    fn read<T: FromBuffer>(buf: &mut impl ToReadBuffer) -> ReadBufferResult<T>;
    fn write<T: IntoBuffer>(n: T, buf: &mut impl ToWriteBuffer) -> WriteBufferResult;
}

pub struct BigEndian {}
impl Endianness for BigEndian {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool {
        match supported {
            SupportedEndianness::BigEndian => true,
            SupportedEndianness::LittleEndian => false,
            SupportedEndianness::Both => true,
        }
    }

    fn read<T: FromBuffer>(buf: &mut impl ToReadBuffer) -> ReadBufferResult<T> {
        T::as_be(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
        n.as_be(buf)
    }
}

pub struct LittleEndian {}
impl Endianness for LittleEndian {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool {
        match supported {
            SupportedEndianness::BigEndian => false,
            SupportedEndianness::LittleEndian => true,
            SupportedEndianness::Both => true,
        }
    }

    fn read<T: FromBuffer>(buf: &mut impl ToReadBuffer) -> ReadBufferResult<T> {
        T::as_le(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
        n.as_le(buf)
    }
}

pub mod prelude {
    pub use crate::{
        bytes_written,
        error::BufferError,
        read::{
            ReadBuffer, ReadBufferResult, Readable, ReadableMulti, ReadableMultiVerify,
            ReadableVerify, ToReadBuffer,
        },
        write::{ToWriteBuffer, WriteBuffer, WriteBufferResult, Writeable},
        BigEndian, Endianness, LittleEndian, SupportedEndianness,
    };
}
