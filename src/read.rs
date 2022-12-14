use crate::{
    error::BufferError, macros::from_buffer_and_readable_impl, BigEndian, Endianness, LittleEndian,
    SupportedEndianness,
};

pub type ReadBufferResult<T> = Result<T, BufferError>;

pub trait ToReadBuffer<'a> {
    fn new(buf: &'a [u8]) -> Self;
    fn pop(&mut self) -> ReadBufferResult<u8>;
    fn skip(&mut self) -> ReadBufferResult<()>;
    fn skipn(&mut self, n: usize) -> ReadBufferResult<()>;

    fn peek(&self) -> Option<u8>;
    fn peekn<const N: usize>(&self) -> Option<[u8; N]>;

    fn offset(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn read_slice(&mut self, nbytes: usize) -> ReadBufferResult<&'a [u8]>;
    fn read_vec(&mut self, nbytes: usize) -> ReadBufferResult<Vec<u8>>;
}

pub struct ReadBuffer<'a> {
    buf: &'a [u8],
    rest: &'a [u8],
}

impl<'a> ToReadBuffer<'a> for ReadBuffer<'a> {
    /// Create a new [`ReadBuffer`] based on a slice of `u8`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let b = ReadBuffer::new(d.as_slice());
    /// assert_eq!(b.len(), 8);
    /// ```
    fn new(buf: &'a [u8]) -> Self {
        ReadBuffer { buf, rest: buf }
    }

    /// Read a single byte from the front of the buffer. If the buffer is
    /// empty, an error is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.pop(), Ok(88));
    /// assert_eq!(b.pop(), Err(BufferError::BufTooShort));
    /// ```
    fn pop(&mut self) -> ReadBufferResult<u8> {
        if let Some((first, rest)) = self.rest.split_first() {
            self.rest = rest;
            return Ok(*first);
        }

        Err(BufferError::BufTooShort)
    }

    /// Pop off a byte from the front of the buffer but do not return the
    /// popped off byte. This is rarely useful other than in combination with
    /// `peek()`.
    fn skip(&mut self) -> ReadBufferResult<()> {
        if let Err(err) = self.pop() {
            return Err(err);
        }

        Ok(())
    }

    /// Pop off `n` bytes from the front of the buffer but do not return the
    /// popped off bytes. This is rarely useful other than in combination with
    /// `peekn()`.
    fn skipn(&mut self, n: usize) -> ReadBufferResult<()> {
        if n > self.len() {
            return Err(BufferError::BufTooShort);
        }

        if n == 1 {
            return self.skip();
        }

        let (_, rest) = self.rest.split_at(n);
        self.rest = rest;

        Ok(())
    }

    /// Peek at the first byte of the buffer. If the buffer is empty
    /// [`None`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.peek(), Some(69));
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.peek(), None);
    /// ```
    fn peek(&self) -> Option<u8> {
        match self.rest.first() {
            Some(b) => Some(*b),
            None => None,
        }
    }

    /// Peek at the first `n` bytes of the buffer. If the buffer is empty
    /// [`None`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.peekn::<2>(), Some([69, 88]));
    /// assert_eq!(b.skipn(2), Ok(()));
    /// assert_eq!(b.peek(), None);
    /// ```
    fn peekn<const N: usize>(&self) -> Option<[u8; N]> {
        match self.rest.get(0..N) {
            Some(s) => match TryInto::<[u8; N]>::try_into(s) {
                Ok(b) => Some(b),
                Err(_) => None,
            },
            None => None,
        }
    }

    /// Returns the current offset.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.offset(), 1);
    /// ```
    fn offset(&self) -> usize {
        return self.buf.len() - self.rest.len();
    }

    /// Returns the len of the remaining buffer.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.len(), 2);
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.len(), 1);
    /// ```
    fn len(&self) -> usize {
        return self.rest.len();
    }

    /// Returns if the buffer is empty.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.is_empty(), false);
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.is_empty(), true);
    /// ```
    fn is_empty(&self) -> bool {
        return self.rest.len() == 0;
    }

    /// Read a slice of bytes with the length `nbytes` from the buffer. If the
    /// number of requested bytes overflow the buffer length, an error is
    /// returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_slice(4), Ok([69, 88, 65, 77].as_slice()));
    /// assert_eq!(b.len(), 4);
    /// ```
    fn read_slice(&mut self, nbytes: usize) -> ReadBufferResult<&'a [u8]> {
        if nbytes > self.len() {
            return Err(BufferError::BufTooShort);
        }

        let (slice, rest) = self.rest.split_at(nbytes);
        self.rest = rest;
        return Ok(slice);
    }

    /// Read `nbytes` bytes from the buffer and return it as a [`Vec<u8>`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_vec(4), Ok(vec![69, 88, 65, 77]));
    /// assert_eq!(b.len(), 4);
    /// ```
    fn read_vec(&mut self, nbytes: usize) -> ReadBufferResult<Vec<u8>> {
        self.read_slice(nbytes).map(ToOwned::to_owned)
    }
}

impl<'a> ReadBuffer<'a> {
    /// Read a character string with an optional maximum length of `max_len`.
    /// A character string is composed of one byte indicating the number of
    /// bytes the string is made of. The string bytes then follow.
    ///
    /// The parameter `max_len` helps to check if the length of the character
    /// string does not exceed any limitations defined by a protocol for
    /// example. This function peeks the next byte to use as the length. If
    /// the length exceeds the provided `max_len` the error
    /// [`BufferError::MaxLengthOverflow`] is returned.
    ///
    /// If the peek returns [`None`] indicating we reached the end of the
    /// buffer the error [`BufferError::BufTooShort`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![4, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_char_string(None), Ok([88, 65, 77, 80].as_slice()));
    /// assert_eq!(b.len(), 3);
    /// ```
    ///
    /// ### Example with a maximum length
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![4, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_char_string(Some(3)), Err(BufferError::MaxLengthOverflow));
    /// assert_eq!(b.len(), 8);
    /// ```
    pub fn read_char_string(&mut self, max_len: Option<usize>) -> ReadBufferResult<&'a [u8]> {
        let len = match self.peek() {
            Some(len) => len as usize,
            None => return Err(BufferError::BufTooShort),
        };

        if let Some(max_len) = max_len {
            if len > max_len {
                return Err(BufferError::MaxLengthOverflow);
            }
        }

        self.skip()?;
        self.read_slice(len)
    }
}

pub trait FromBuffer: Sized {
    const SIZE: usize;

    fn as_be(buf: &mut ReadBuffer) -> ReadBufferResult<Self>;
    fn as_le(buf: &mut ReadBuffer) -> ReadBufferResult<Self>;
}

/// All types which implement this trait can be constructed by reading from
/// a [`ReadBuffer`]. An implementation for all sized unsigned integers is
/// provided.
///
/// ### Example
///
/// ```
/// use binbuf::prelude::*;
///
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut b = ReadBuffer::new(d.as_slice());
/// assert_eq!(u16::read::<BigEndian>(&mut b), Ok(17752));
/// ```
pub trait Readable: Sized + std::fmt::Debug {
    const SUPPORTED_ENDIANNESS: SupportedEndianness;

    /// Read [`Self`] from a [`ReadBuffer`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// let i = u16::read::<BigEndian>(&mut b).unwrap();
    /// assert_eq!(i, 17752);
    /// ```
    fn read<E: Endianness>(buf: &mut ReadBuffer) -> ReadBufferResult<Self>;

    /// Returns if this type [`Self`] supports the requested endianness
    /// encoding. If not [`BufferError::UnsupportedEndianness`] ire
    /// returned.
    fn supports<E: Endianness>() -> ReadBufferResult<()> {
        if !E::is_in_supported_endianness_set(Self::SUPPORTED_ENDIANNESS) {
            return Err(BufferError::UnsupportedEndianness);
        }

        Ok(())
    }

    /// Read [`Self`] with big endian encoding from a [`ReadBuffer`].
    /// Internally this calls `Self::read::<BigEndian>()`.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// let i = u16::read_be(&mut b).unwrap();
    /// assert_eq!(i, 17752);
    /// ```
    fn read_be(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
        Self::read::<BigEndian>(buf)
    }

    /// Read [`Self`] with little endian encoding from a [`ReadBuffer`].
    /// Internally this calls `Self::read::<LittleEndian>()`.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// let i = u16::read_le(&mut b).unwrap();
    /// assert_eq!(i, 22597);
    /// ```
    fn read_le(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
        Self::read::<LittleEndian>(buf)
    }
}

/// Multiple values of types which implement this trait can be read at once
/// from a [`ReadBuffer`]. An implementation for all sized unsigned integers is
/// provided.
///
/// ### Example
///
/// ```
/// use binbuf::prelude::*;
///
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut b = ReadBuffer::new(d.as_slice());
///
/// let [i1, i2] = u16::read_multi::<BigEndian, 2>(&mut b).unwrap();
///
/// assert_eq!(i1, 17752);
/// assert_eq!(i2, 16717);
/// ```
pub trait ReadableMulti: Readable + Default + Copy {
    /// Read multiple [`Self`] from a [`ReadBuffer`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// let [i1, i2, i3, i4] = u16::read_multi::<BigEndian, 4>(&mut b).unwrap();
    ///
    /// assert_eq!(i1, 17752);
    /// assert_eq!(i2, 16717);
    /// assert_eq!(i3, 20556);
    /// assert_eq!(i4, 17697);
    /// ```
    fn read_multi<E: Endianness, const S: usize>(
        buf: &mut ReadBuffer,
    ) -> ReadBufferResult<[Self; S]> {
        let mut a = [Self::default(); S];
        for i in 0..S {
            a[i] = Self::read::<E>(buf)?;
        }
        return Ok(a);
    }

    fn read_multi_be<const S: usize>(buf: &mut ReadBuffer) -> ReadBufferResult<[Self; S]> {
        Self::read_multi::<BigEndian, S>(buf)
    }

    fn read_multi_le<const S: usize>(buf: &mut ReadBuffer) -> ReadBufferResult<[Self; S]> {
        Self::read_multi::<LittleEndian, S>(buf)
    }
}

from_buffer_and_readable_impl!(u8, 1);
from_buffer_and_readable_impl!(u16, 2);
from_buffer_and_readable_impl!(u32, 4);
from_buffer_and_readable_impl!(u64, 8);
from_buffer_and_readable_impl!(u128, 16);
