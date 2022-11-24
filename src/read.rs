use crate::{error::BufferError, macros::from_buffer_and_readable_impl, Endianness};

pub type ReadBufferResult<T> = Result<T, BufferError>;

pub trait ToReadBuffer<'a> {
    fn new(buf: &'a [u8]) -> Self;
    fn pop(&mut self) -> ReadBufferResult<u8>;
    fn skip(&mut self) -> ReadBufferResult<()>;
    fn peek(&self) -> Option<u8>;
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
    /// use binbuf::ReadBuffer;
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
    /// use binbuf::{ReadBuffer, BufferError};
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

    /// Peek at the first byte of the buffer. If the buffer is empty
    /// [`None`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
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

    /// Returns the current offset.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
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
    /// use binbuf::ReadBuffer;
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
    /// use binbuf::ReadBuffer;
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
    /// use binbuf::ReadBuffer;
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
    /// use binbuf::ReadBuffer;
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
/// use binbuf::{ReadBuffer, Readable};
///
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut b = ReadBuffer::new(d.as_slice());
/// assert_eq!(u16::read(&mut b), Ok(17752));
/// ```
pub trait Readable: Sized {
    /// Read [`Self`] from a [`ReadBuffer`].
    fn read<E: Endianness>(buf: &mut ReadBuffer) -> ReadBufferResult<Self>;
}

from_buffer_and_readable_impl!(u8);
from_buffer_and_readable_impl!(u16);
from_buffer_and_readable_impl!(u32);
from_buffer_and_readable_impl!(u64);
from_buffer_and_readable_impl!(u128);
