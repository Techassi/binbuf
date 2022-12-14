use crate::{
    read::{FromBuffer, ReadBufferResult, ToReadBuffer},
    write::{IntoBuffer, ToWriteBuffer, WriteBufferResult},
};

pub mod error;
pub mod macros;
pub mod read;
pub mod write;

pub enum SupportedEndianness {
    BigEndian,
    LittleEndian,
    Both,
}

pub trait Endianness {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool;

    fn read<'a, T: FromBuffer<'a>>(buf: &mut impl ToReadBuffer) -> ReadBufferResult<T>;
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

    fn read<'a, T: FromBuffer<'a>>(buf: &mut impl ToReadBuffer) -> ReadBufferResult<T> {
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

    fn read<'a, T: FromBuffer<'a>>(buf: &mut impl ToReadBuffer) -> ReadBufferResult<T> {
        T::as_le(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
        n.as_le(buf)
    }
}

pub mod prelude {
    pub use crate::{
        error::BufferError,
        read::{
            ReadBuffer, ReadBufferResult, Readable, ReadableMulti, ReadableMultiVerify,
            ReadableVerify, ToReadBuffer,
        },
        write::{ToWriteBuffer, WriteBuffer, WriteBufferResult, Writeable},
        BigEndian, Endianness, LittleEndian, SupportedEndianness,
    };
}
