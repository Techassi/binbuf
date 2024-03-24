use std::collections::HashMap;

use binbuf_macros::into_buffer_and_writeable_impl;
use snafu::{ensure, Snafu};

use crate::{BigEndian, Endianness, LittleEndian, SupportedEndianness};

pub type WriteResult<T = usize, E = WriteError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum WriteError {
    #[snafu(display(
        "the length of the character string oveflows the max value encodable using an u8"
    ))]
    LengthLabelOverflow,

    #[snafu(display("max buffer length overflow"))]
    MaxLengthOverflow,

    #[snafu(display("unsupported endianness, only supports: {support}"))]
    UnsupportedEndianness { support: SupportedEndianness },

    #[snafu(display("non-ascii string data cannot be written"))]
    NonAsciiData,
}

#[derive(Debug, Default)]
pub struct WriteBuffer {
    spans: Vec<usize>,
    buf: Vec<u8>,
}

impl WriteBuffer {
    /// Creates a new empty [`Buffer`] backed by a `Vec<u8>`.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new();
    /// 17752u16.write::<BigEndian>(&mut b).unwrap();
    ///
    /// assert_eq!(b.len(), 2);
    /// assert_eq!(b.bytes(), &[69, 88]);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Buffer`] backed by a `Vec<u8>` with the provided bytes
    /// already in the buffer. Possible parameters are: `Vec<u8>`, `&[u8]`, and
    /// `[u8]`.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new_with([69, 88]);
    /// assert_eq!(b.bytes(), &[69, 88]);
    /// ```
    pub fn new_with<T: AsRef<[u8]>>(b: T) -> Self {
        let b = b.as_ref();
        let mut buf = Self {
            buf: Vec::with_capacity(b.len()),
            spans: Vec::new(),
        };

        buf.write(b);
        buf
    }

    /// Adds a new byte to the end of the [`Buffer`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new();
    /// b.push(69);
    ///
    /// assert_eq!(b.len(), 1);
    /// assert_eq!(b.bytes(), &[69]);
    /// ```
    pub fn push(&mut self, b: u8) {
        self.buf.push(b);

        if let Some(last) = self.spans.last_mut() {
            *last += 1;
        }
    }

    /// Clears the [`Buffer`], removing all bytes.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new_with([69, 88]);
    /// b.clear();
    ///
    /// assert_eq!(b.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.buf.clear()
    }

    /// Returns the length of the [`Buffer`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new_with([69, 88]);
    /// assert_eq!(b.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns if the [`Buffer`] is empty.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new();
    /// assert_eq!(b.is_empty(), true);
    ///
    /// b.push(69);
    /// assert_eq!(b.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Writes multiple bytes of data to the [`Buffer`]. Possible
    /// parameters are: `Vec<u8>`, `&[u8]`, and `[u8]`.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new();
    /// b.write(vec![69, 88, 65]);
    ///
    /// assert_eq!(b.len(), 3);
    /// assert_eq!(b.bytes(), &[69, 88, 65]);
    /// ```
    pub fn write(&mut self, b: impl AsRef<[u8]>) -> usize {
        let bytes = b.as_ref();
        let len = bytes.len();

        self.buf.extend_from_slice(bytes);

        if let Some(last) = self.spans.last_mut() {
            *last += len;
        }

        len
    }

    /// Writes a character string to the [`Buffer`]. This will first write the
    /// length of the string as a sequence of bytes which is followed by the
    /// actual string contents.
    ///
    /// If the length of the character string is larger than [`L::MAX`], the
    /// function returns the error [`Error::LengthLabelOverflow`]. If the
    /// length of the string exceeds the optional `max_len`, the function
    /// returns [`Error::MaxLengthOverflow`].
    ///
    /// It is possible to write character strings, which exceeds the length of
    /// [`u8::MAX`][u8max]. Most network protocols however only allow character
    /// strings with a max size of [`u8::MAX`][u8max] at most. DNS being one
    /// prominent example. The length label is currently **only** writen using
    /// big endian byte order.
    ///
    /// [u8max]: https://doc.rust-lang.org/std/primitive.u8.html#associatedconstant.MAX
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::write::Buffer;
    ///
    /// let mut b = Buffer::new();
    /// b.write_char_string(&[88, 65, 77, 80], None).unwrap();
    ///
    /// assert_eq!(b.len(), 5);
    /// assert_eq!(b.bytes(), &[4, 88, 65, 77, 80]);
    /// ```
    pub fn write_char_string<L>(&mut self, s: impl AsRef<[u8]>, max_len: Option<L>) -> WriteResult
    where
        L: num::Unsigned + num::Bounded + Into<usize>,
    {
        let s = s.as_ref();
        let len = s.len();

        // Ensure that the length label of the string doesn't exceed the maximum
        // value which can be encoded using a u8.
        ensure!(len <= L::max_value().into(), LengthLabelOverflowSnafu);

        // Ensure length is smaller than max_len
        if let Some(max_len) = max_len {
            ensure!(len <= max_len.into(), MaxLengthOverflowSnafu);
        }

        let n = len.write_be(self)?;
        Ok(self.write(s) + n)
    }

    pub fn enter(&mut self) {
        self.spans.push(0);
    }

    pub fn exit(&mut self) -> usize {
        let n = self.spans.pop().unwrap_or(self.len());
        if let Some(last) = self.spans.last_mut() {
            *last += n;
        }
        n
    }

    /// Returns the content of [`WriteBuffer`] as a slice of bytes.
    pub fn bytes(&self) -> &[u8] {
        self.buf.as_slice()
    }

    /// Returns the content [`WriteBuffer`] as an owned vector of bytes.
    pub fn owned_bytes(&self) -> Vec<u8> {
        self.buf.clone()
    }
}

pub trait IntoBuffer: Sized {
    const SIZE: usize;

    fn as_be(&self, buf: &mut WriteBuffer) -> usize;
    fn as_le(&self, buf: &mut WriteBuffer) -> usize;
}

pub trait Writeable: Sized {
    type Error: std::error::Error + std::fmt::Display;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error>;

    fn write_be(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error> {
        self.write::<BigEndian>(buf)
    }

    fn write_le(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error> {
        self.write::<LittleEndian>(buf)
    }
}

pub trait WriteableVerify: Writeable
where
    <Self as Writeable>::Error: From<WriteError>,
{
    const SUPPORTED_ENDIANNESS: SupportedEndianness;

    fn write_verify<E: Endianness>(
        &self,
        buf: &mut WriteBuffer,
    ) -> WriteResult<usize, Self::Error> {
        Self::supports::<E>()?;
        self.write::<E>(buf)
    }

    fn write_verify_be(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error> {
        self.write_verify::<BigEndian>(buf)
    }

    fn write_verify_le(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error> {
        self.write_verify::<LittleEndian>(buf)
    }

    /// Returns if this type [`Self`] supports the requested endianness
    /// encoding. If not [`BufferError::UnsupportedEndianness`] ire
    /// returned.
    fn supports<E: Endianness>() -> WriteResult {
        ensure!(
            E::is_in_supported_endianness_set(Self::SUPPORTED_ENDIANNESS),
            UnsupportedEndiannessSnafu {
                support: Self::SUPPORTED_ENDIANNESS
            }
        );

        Ok(0)
    }
}

into_buffer_and_writeable_impl!(u8, 1);
into_buffer_and_writeable_impl!(u16, 2);
into_buffer_and_writeable_impl!(u32, 4);
into_buffer_and_writeable_impl!(u64, 8);
into_buffer_and_writeable_impl!(u128, 16);
into_buffer_and_writeable_impl!(usize, (usize::BITS / 8) as usize);

impl<T: Writeable> Writeable for Vec<T> {
    type Error = T::Error;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error> {
        buf.enter();
        for item in self.iter() {
            item.write::<E>(buf)?;
        }
        Ok(buf.exit())
    }
}

impl<T: WriteableVerify> WriteableVerify for Vec<T>
where
    <Self as Writeable>::Error: From<WriteError>,
{
    const SUPPORTED_ENDIANNESS: SupportedEndianness = T::SUPPORTED_ENDIANNESS;
}

impl<K, V: Writeable> Writeable for HashMap<K, V> {
    type Error = V::Error;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> WriteResult<usize, Self::Error> {
        buf.enter();
        for value in self.values() {
            value.write::<E>(buf)?;
        }
        Ok(buf.exit())
    }
}

impl<K, V: WriteableVerify> WriteableVerify for HashMap<K, V>
where
    <Self as Writeable>::Error: From<WriteError>,
{
    const SUPPORTED_ENDIANNESS: SupportedEndianness = V::SUPPORTED_ENDIANNESS;
}
