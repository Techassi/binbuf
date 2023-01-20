#[macro_export]
macro_rules! from_buffer_and_readable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl FromBuffer for $SelfT {
            const SIZE: usize = $Size;

            fn as_be(buf: &mut impl ToReadBuffer) -> ReadBufferResult<Self> {
                let b = buf.read_slice(Self::SIZE)?;
                Ok(Self::from_be_bytes(b.try_into().unwrap()))
            }

            fn as_le(buf: &mut impl ToReadBuffer) -> ReadBufferResult<Self> {
                let b = buf.read_slice(Self::SIZE)?;
                Ok(Self::from_le_bytes(b.try_into().unwrap()))
            }
        }

        impl Readable for $SelfT {
            type Error = BufferError;
            fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
                E::read(buf)
            }
        }

        impl ReadableVerify for $SelfT {
            const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;
        }

        impl ReadableMulti for $SelfT {}
        impl ReadableMultiVerify for $SelfT {}
    };
}

#[macro_export]
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

        impl Writeable for $SelfT {
            type Error = BufferError;

            fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> WriteBufferResult {
                E::write(*self, buf)
            }
        }
    };
}

#[macro_export]
macro_rules! bytes_written {
    ($($Fn:expr);+) => {
        {
            let mut __n = 0;
            $(
                __n += $Fn;
            )+
            __n
        }
    };
}
