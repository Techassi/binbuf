use crate::{
    error::BufferError, macros::into_buffer_and_writeable_impl, BigEndian, Endianness,
    LittleEndian, SupportedEndianness,
};

pub type WriteBufferResult = Result<usize, BufferError>;

pub trait ToWriteBuffer {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, b: u8);
    fn write_slice(&mut self, s: &[u8]) -> WriteBufferResult;
    fn write_vec(&mut self, v: &mut Vec<u8>) -> WriteBufferResult;
    fn bytes(&self) -> &[u8];
    fn clear(&mut self);
}

pub struct WriteBuffer {
    buf: Vec<u8>,
}

impl ToWriteBuffer for WriteBuffer {
    fn new() -> Self {
        WriteBuffer { buf: Vec::new() }
    }

    fn len(&self) -> usize {
        return self.buf.len();
    }

    fn is_empty(&self) -> bool {
        return self.buf.len() == 0;
    }

    fn push(&mut self, b: u8) {
        self.buf.push(b);
    }

    fn write_slice(&mut self, s: &[u8]) -> WriteBufferResult {
        self.buf.extend_from_slice(s);
        Ok(s.len())
    }

    fn write_vec(&mut self, v: &mut Vec<u8>) -> WriteBufferResult {
        self.buf.append(v);
        Ok(v.len())
    }

    fn bytes(&self) -> &[u8] {
        return self.buf.as_slice();
    }

    fn clear(&mut self) {
        self.buf.clear()
    }
}

impl WriteBuffer {
    pub fn write_char_string(&mut self, s: &[u8]) -> WriteBufferResult {
        let l = s.len();

        if l > u8::MAX as usize {
            return Err(BufferError::MaxLengthOverflow);
        }

        self.push(l as u8);

        match self.write_slice(s) {
            Ok(n) => Ok(n + 1),
            Err(err) => return Err(err),
        }
    }
}

pub trait IntoBuffer: Sized {
    const SIZE: usize;

    fn as_be(&self, buf: &mut impl ToWriteBuffer) -> WriteBufferResult;
    fn as_le(&self, buf: &mut impl ToWriteBuffer) -> WriteBufferResult;
}

pub trait Writeable<'a>: Sized {
    type Error: std::error::Error + From<BufferError>;

    fn write<E: Endianness<'a>>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error>;

    fn write_be(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        self.write::<BigEndian>(buf)
    }

    fn write_le(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        self.write::<LittleEndian>(buf)
    }
}

pub trait WriteableVerify<'a>: Writeable<'a> {
    const SUPPORTED_ENDIANNESS: SupportedEndianness;

    fn write_verify<E: Endianness<'a>>(
        &self,
        buf: &mut impl ToWriteBuffer,
    ) -> Result<usize, Self::Error> {
        Self::supports::<E>()?;
        self.write::<E>(buf)
    }

    fn write_verify_be(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        self.write_verify::<BigEndian>(buf)
    }

    fn write_verify_le(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        self.write_verify::<LittleEndian>(buf)
    }

    /// Returns if this type [`Self`] supports the requested endianness
    /// encoding. If not [`BufferError::UnsupportedEndianness`] ire
    /// returned.
    fn supports<E: Endianness<'a>>() -> WriteBufferResult {
        if !E::is_in_supported_endianness_set(Self::SUPPORTED_ENDIANNESS) {
            return Err(BufferError::UnsupportedEndianness);
        }

        Ok(0)
    }
}

into_buffer_and_writeable_impl!(u8, 1);
into_buffer_and_writeable_impl!(u16, 2);
into_buffer_and_writeable_impl!(u32, 4);
into_buffer_and_writeable_impl!(u64, 8);
into_buffer_and_writeable_impl!(u128, 16);

impl<'a, T: Writeable<'a>> Writeable<'a> for Vec<T> {
    type Error = T::Error;

    fn write<E: Endianness<'a>>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        let mut written = 0;
        for item in self.iter() {
            written += item.write::<E>(buf)?
        }
        Ok(written)
    }
}

impl<'a, T: WriteableVerify<'a>> WriteableVerify<'a> for Vec<T> {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = T::SUPPORTED_ENDIANNESS;
}
