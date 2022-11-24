use crate::{
    read::{FromBuffer, ReadBuffer, ReadBufferResult},
    write::{IntoBuffer, WriteBuffer, WriteBufferResult},
};

pub mod error;
pub mod macros;
pub mod read;
pub mod write;

pub trait Endianness {
    fn read<T: FromBuffer>(buf: &mut ReadBuffer) -> ReadBufferResult<T>;
    fn write<T: IntoBuffer>(n: T, buf: &mut WriteBuffer) -> WriteBufferResult;
}

pub struct BigEndian {}
impl Endianness for BigEndian {
    fn read<T: FromBuffer>(buf: &mut ReadBuffer) -> ReadBufferResult<T> {
        T::as_be(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut WriteBuffer) -> WriteBufferResult {
        n.as_be(buf)
    }
}

pub struct LittleEndian {}
impl Endianness for LittleEndian {
    fn read<T: FromBuffer>(buf: &mut ReadBuffer) -> ReadBufferResult<T> {
        T::as_le(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut WriteBuffer) -> WriteBufferResult {
        n.as_le(buf)
    }
}

pub mod prelude {
    pub use crate::{
        error::BufferError,
        read::{ReadBuffer, ReadBufferResult, Readable, ToReadBuffer},
        write::{ToWriteBuffer, WriteBuffer, WriteBufferResult, Writeable},
        BigEndian, LittleEndian,
    };
}
