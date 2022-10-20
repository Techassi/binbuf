use std::io::{Read, Write};

use crate::{BinaryError, BinaryReadResult, BinaryWriteResult, Endianness, FromBytes, IntoBytes};

pub trait ReadExt: Read {
    /// Read a single value from a reader, e.g. a [`Cursor`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binum::prelude::*;
    /// use std::io::Cursor;
    ///
    /// let mut r = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);
    /// let n = r.read_from::<u16, BigEndian>().unwrap();
    ///
    /// assert_eq!(n, 17752);
    /// ```
    fn read_from<T: FromBytes, E: Endianness>(&mut self) -> BinaryReadResult<T> {
        let mut buf = vec![0; T::SIZE];

        match self.read_exact(&mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        };

        E::read::<T>(&buf)
    }

    /// Read multiple values from a reader, e.g. a [`Cursor`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binum::prelude::*;
    /// use std::io::Cursor;
    ///
    /// let mut r = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);
    /// let n = r.read_multi::<u16, BigEndian>(2).unwrap();
    ///
    /// assert_eq!(n[0], 17752);
    /// assert_eq!(n[1], 16717);
    /// ```
    fn read_multi<T: FromBytes, E: Endianness>(
        &mut self,
        nints: usize,
    ) -> BinaryReadResult<Vec<T>> {
        let mut buf = vec![0; T::SIZE * nints];

        match self.read_exact(&mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        }

        E::read_multi(&buf, nints)
    }
}

// Auto implement ReadExt for types which implement Read
impl<R: Read + ?Sized> ReadExt for R {}

pub trait WriteExt: Write {
    /// Write a single value into a writer, e.g. a [`Cursor`] or [`Vec`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binum::prelude::*;
    ///
    /// let mut w: Vec<u8> = Vec::new();
    /// let n = w.write_into::<u16, BigEndian>(17752).unwrap();
    ///
    /// assert_eq!(n, 2);
    /// assert_eq!(w, vec![69, 88]);
    /// ```
    fn write_into<T: IntoBytes, E: Endianness>(&mut self, n: T) -> BinaryWriteResult {
        let mut buf = vec![0; T::SIZE];

        match E::write(n, &mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        };

        match self.write(&buf) {
            Ok(n) => Ok(n),
            Err(err) => Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
        }
    }

    /// Write multiple values into a writer, e.g. a [`Cursor`] or [`Vec`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binum::prelude::*;
    ///
    /// let mut w: Vec<u8> = Vec::new();
    /// let n = w.write_multi::<u16, BigEndian>(vec![17752, 16717]).unwrap();
    ///
    /// assert_eq!(n, 4);
    /// assert_eq!(w, vec![69, 88, 65, 77]);
    /// ```
    fn write_multi<T: IntoBytes, E: Endianness>(&mut self, n: Vec<T>) -> BinaryWriteResult {
        let mut buf = vec![0; T::SIZE * n.len()];

        match E::write_multi(n, &mut buf) {
            Err(err) => return Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
            _ => {}
        };

        match self.write(&buf) {
            Ok(n) => Ok(n),
            Err(err) => Err(BinaryError::new(err.to_string(), T::ERR_VARIANT)),
        }
    }
}

// Auto implement WriteExt for types which implement Write
impl<W: Write + ?Sized> WriteExt for W {}
