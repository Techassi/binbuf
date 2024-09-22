use binbuf_macros::from_buffer_and_readable_impl;
use snafu::{ensure, OptionExt, Snafu};

use crate::{BigEndian, Endianness, LittleEndian};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, PartialEq, Snafu)]
pub enum Error {
    /// This error indicates that the buffer is too short to read the
    /// requested amount of bytes.
    #[snafu(display("buffer too short"))]
    BufferTooShort,

    #[snafu(display("invalid jump, jumping to {index} beyond offset {offset} is not permitted"))]
    InvalidJump {
        index: usize,
        offset: usize,
    },

    #[snafu(display("max buffer length overflow"))]
    MaxLengthOverflow,

    #[snafu(display("failed to read data because {message}"))]
    Custom {
        message: String,
    },

    #[snafu(display("invalid data"))]
    InvalidData,

    LittleEndianNotSupported,
    BigEndianNotSupported,
}

#[derive(Debug)]
pub struct Reader<'a> {
    jump_indices: Vec<usize>,
    buf: &'a [u8],
    rest: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Create a new [`Reader`] based on a slice of `u8`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::{Reader, Error};
    ///
    /// let d = &[69, 88, 65, 77, 80, 76, 69, 33];
    /// let b = Reader::new(d);
    /// assert_eq!(b.len(), 8);
    /// ```
    pub fn new(buf: &'a [u8]) -> Self {
        Reader {
            buf,
            rest: buf,
            jump_indices: Vec::new(),
        }
    }

    /// Read a single byte from the front of the buffer. If the buffer is
    /// empty, an error is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::{Buffer, Error};
    ///
    /// let mut b = Buffer::new(&[69, 88]);
    ///
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.pop(), Ok(88));
    /// assert_eq!(b.pop(), Err(Error::BufferTooShort));
    /// ```
    pub fn pop(&mut self) -> Result<u8> {
        if let Some((first, rest)) = self.rest.split_first() {
            self.rest = rest;
            return Ok(*first);
        }

        BufferTooShortSnafu.fail()
    }

    /// Pop off a byte from the front of the buffer without returning the byte.
    /// This is rarely useful other than in combination with [`ReadBuffer::peek()`].
    pub fn skip(&mut self) -> Result<()> {
        self.pop()?;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.rest = self.buf;
    }

    /// Pop off `n` bytes from the front of the buffer but do not return the
    /// popped off bytes. This is rarely useful other than in combination with
    /// `peekn()`.
    pub fn skipn(&mut self, n: usize) -> Result<()> {
        // Ensure the buffer is long enough to skip n bytes.
        ensure!(n <= self.len(), BufferTooShortSnafu);

        if n == 1 {
            return self.skip();
        }

        let (_, rest) = self.rest.split_at(n);
        self.rest = rest;

        Ok(())
    }

    /// Peek the next byte of the buffer. If the buffer is empty
    /// [`None`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let mut b = Buffer::new(&[69]);
    ///
    /// assert_eq!(b.peek(), Some(69));
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<u8> {
        self.rest.first().copied()
    }

    /// Peek the next `n` bytes of the buffer. If the buffer is empty
    /// [`None`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let mut b = Buffer::new(&[69, 88]);
    ///
    /// assert_eq!(b.peekn::<2>(), Some([69, 88]));
    /// assert_eq!(b.skipn(2), Ok(()));
    /// assert_eq!(b.peek(), None);
    /// ```
    pub fn peekn<const N: usize>(&self) -> Option<[u8; N]> {
        match self.rest.get(0..N) {
            Some(s) => match TryInto::<[u8; N]>::try_into(s) {
                Ok(b) => Some(b),
                Err(_) => None,
            },
            None => None,
        }
    }

    /// Jumps back to offset `index`. Jumping beyond the current offset is not
    /// permitted and returns [`Error::InvalidJump`].
    pub fn jump_to(&mut self, index: usize) -> Result<()> {
        // Ensure we don't jump to ann index larger than the currennt offset.
        ensure!(
            index <= self.offset(),
            InvalidJumpSnafu {
                index,
                offset: self.offset()
            }
        );

        self.jump_indices.push(self.offset());
        self.rest = &self.buf[index..];

        Ok(())
    }

    /// Resets the jump indices and returns true if there were any indices.
    pub fn jump_reset(&mut self) -> bool {
        if !self.jumped() {
            return false;
        }

        let index = *self.jump_indices.first().unwrap();
        self.jump_indices.clear();

        self.rest = &self.buf[index..];
        true
    }

    /// Returns if there are any jump indices stores.
    pub fn jumped(&self) -> bool {
        !self.jump_indices.is_empty()
    }

    /// Jumps back one index and removes the index from the stored jump indices.
    pub fn jump_back(&mut self) {
        if let Some(index) = self.jump_indices.pop() {
            self.rest = &self.buf[index..];
        }
    }

    /// Returns the current offset.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let mut b = Buffer::new(&[69, 88]);
    ///
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.offset(), 1);
    /// ```
    pub fn offset(&self) -> usize {
        self.buf.len() - self.rest.len()
    }

    /// Returns the length of the remaining buffer.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let mut b = Buffer::new(&[69, 88]);
    ///
    /// assert_eq!(b.len(), 2);
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.rest.len()
    }

    /// Returns if the buffer is empty.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let mut b = Buffer::new(&[69]);
    ///
    /// assert_eq!(b.is_empty(), false);
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }

    /// Read a character string with an optional maximum length of `max_len`.
    /// A character string is composed of one byte indicating the number of
    /// bytes the string is made of. The string bytes then follow.
    ///
    /// The parameter `max_len` helps to check if the length of the character
    /// string does not exceed any limitations defined by a protocol for
    /// example. This function peeks the next byte to use as the length. If
    /// the length exceeds the provided `max_len` the error
    /// [`ReadError::MaxLengthOverflow`] is returned.
    ///
    /// If the peek returns [`None`] indicating we reached the end of the
    /// buffer the error [`ReadError::BufferTooShort`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let d = &[4, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = Buffer::new(d);
    ///
    /// assert_eq!(b.read_char_string(None), Ok(&[88, 65, 77, 80]));
    /// assert_eq!(b.len(), 3);
    /// ```
    ///
    /// ### Example with a maximum length
    ///
    /// ```
    /// use binbuf::read::{Buffer, Error};
    ///
    /// let d = &[4, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = Buffer::new(d);
    ///
    /// assert_eq!(b.read_char_string(Some(3)), Err(Error::MaxLengthOverflow));
    /// assert_eq!(b.len(), 8);
    /// ```
    pub fn read_char_string(&mut self, max_len: Option<u8>) -> Result<&[u8]> {
        let len = self.peek().context(BufferTooShortSnafu)? as usize;

        if let Some(max_len) = max_len {
            ensure!(len <= max_len.into(), MaxLengthOverflowSnafu);
        }

        self.skip()?;
        self.read_slice(len)
    }

    /// Read a slice of bytes with the length `nbytes` from the buffer. If the
    /// number of requested bytes overflow the buffer length, an error is
    /// returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let d = &[69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = Buffer::new(d);
    ///
    /// assert_eq!(b.read_slice(4), Ok(&[69, 88, 65, 77]));
    /// assert_eq!(b.len(), 4);
    /// ```
    pub fn read_slice(&mut self, nbytes: usize) -> Result<&[u8]> {
        ensure!(nbytes <= self.len(), BufferTooShortSnafu);

        let (slice, rest) = self.rest.split_at(nbytes);
        self.rest = rest;

        Ok(slice)
    }

    /// Read `nbytes` bytes from the buffer and return it as a [`Vec<u8>`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::read::Buffer;
    ///
    /// let d = &[69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = Buffer::new(d);
    ///
    /// assert_eq!(b.read_vec(4), Ok(vec![69, 88, 65, 77]));
    /// assert_eq!(b.len(), 4);
    /// ```
    pub fn read_vec(&mut self, nbytes: usize) -> Result<Vec<u8>> {
        self.read_slice(nbytes).map(ToOwned::to_owned)
    }
}

/// All types which implement this trait can be constructed by reading from
/// a [`ReadBuffer`]. An implementation for all sized unsigned integers is
/// provided.
///
/// ### Example
///
/// ```
/// use binbuf::{BigEndian, read::{Buffer, Readable}};
///
/// let d = &[69, 88, 65, 77, 80, 76, 69, 33];
/// let mut b = Buffer::new(d);
///
/// assert_eq!(u16::read::<BigEndian>(&mut b), Ok(17752));
/// ```
pub trait Read: Sized {
    /// Read [`Self`] from a [`Reader`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::prelude::*;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut r = Reader::new(d.as_slice());
    ///
    /// let i = u16::read::<BigEndian>(&mut r).unwrap();
    /// assert_eq!(i, 17752);
    /// ```
    fn read<E: Endianness>(buf: &mut Reader) -> Result<Self> {
        E::read(buf)
    }

    /// Read [`Self`] with big endian encoding from a [`ReadBuffer`].
    ///
    /// Leaving this function unimplemented (using the default implementation)
    /// will indicate that this type does not support the big endian encoding.
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
    #[allow(unused_variables)]
    fn read_be(buf: &mut Reader) -> Result<Self> {
        BigEndianNotSupportedSnafu.fail()
    }

    /// Read [`Self`] with little endian encoding from a [`ReadBuffer`].
    ///
    /// Leaving this function unimplemented (using the default implementation)
    /// will indicate that this type does not support the little endian
    /// encoding.
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
    #[allow(unused_variables)]
    fn read_le(buf: &mut Reader) -> Result<Self> {
        LittleEndianNotSupportedSnafu.fail()
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
pub trait ReadableMulti: Read + Default + Copy {
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
    fn read_multi<E: Endianness, const S: usize>(buf: &mut Reader) -> Result<[Self; S]> {
        let mut a = [Self::default(); S];

        for b in a.iter_mut().take(S) {
            *b = Self::read::<E>(buf)?;
        }

        Ok(a)
    }

    fn read_multi_be<const S: usize>(buf: &mut Reader) -> Result<[Self; S]> {
        Self::read_multi::<BigEndian, S>(buf)
    }

    fn read_multi_le<const S: usize>(buf: &mut Reader) -> Result<[Self; S]> {
        Self::read_multi::<LittleEndian, S>(buf)
    }
}

from_buffer_and_readable_impl!(u8, 1);
from_buffer_and_readable_impl!(u16, 2);
from_buffer_and_readable_impl!(u32, 4);
from_buffer_and_readable_impl!(u64, 8);
from_buffer_and_readable_impl!(u128, 16);
