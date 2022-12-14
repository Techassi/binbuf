pub use binbuf_macros::*;

macro_rules! from_buffer_and_readable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl<'a> FromBuffer<'a> for $SelfT {
            const SIZE: usize = $Size;

            fn as_be(buf: &mut impl ToReadBuffer<'a>) -> ReadBufferResult<Self> {
                let b = buf.read_slice(Self::SIZE)?;
                Ok(Self::from_be_bytes(b.try_into().unwrap()))
            }

            fn as_le(buf: &mut impl ToReadBuffer<'a>) -> ReadBufferResult<Self> {
                let b = buf.read_slice(Self::SIZE)?;
                Ok(Self::from_le_bytes(b.try_into().unwrap()))
            }
        }

        impl<'a> Readable<'a> for $SelfT {
            type Error = BufferError;
            fn read<E: Endianness<'a>>(
                buf: &mut impl ToReadBuffer<'a>,
            ) -> Result<Self, Self::Error> {
                E::read(buf)
            }
        }

        impl<'a> ReadableVerify<'a> for $SelfT {
            const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;
        }

        impl<'a> ReadableMulti<'a> for $SelfT {}
        impl<'a> ReadableMultiVerify<'a> for $SelfT {}
    };
}

macro_rules! into_buffer_and_writeable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl IntoBuffer for $SelfT {
            const SIZE: usize = $Size;

            fn as_be(&self, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
                let b = self.to_be_bytes();
                buf.write_slice(&b[..])
            }

            fn as_le(&self, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
                let b = self.to_le_bytes();
                buf.write_slice(&b[..])
            }
        }

        impl<'a> Writeable<'a> for $SelfT {
            type Error = BufferError;

            fn write<E: Endianness<'a>>(&self, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
                E::write(*self, buf)
            }
        }
    };
}

pub(crate) use from_buffer_and_readable_impl;
pub(crate) use into_buffer_and_writeable_impl;
