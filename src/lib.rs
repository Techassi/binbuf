//! A small yet powerfull library to work with binary (network) data in Rust.
//! The library makes the proccess of writing and reading data of network
//! protocols easy. It has out-of-the-box support for various `std` types,
//! like all unsigned integers as well as `Ipv4Addr` and `Ipv6Addr` in big and
//! little endian byte order.
//!
//! # A Tour of binbuf
//!
//! binbuf consists of a number of modules that provide functionality to read
//! and write data, as well as handling of errors which can occur during these
//! actions. The easiest way to get started is to enable all features. Do this
//! by enabling the full feature flag:
//!
//! ```toml
//! binbuf = { version = "0.0.1", features = ["full"] }
//! ```
//!
//! ## Reading Simple Data Types
//!
//! ```rust
//! # use binbuf::*;
//!
//! let b = &[69, 88, 65, 77, 80, 76, 69, 33];
//! let mut b = Reader::new(b);
//!
//! match u16::read::<BigEndian>(&mut b) {
//!     Ok(n) => assert_eq!(n, 17752),
//!     Err(err) => panic!("{}", err),
//! }
//! ```

mod impls;
pub mod read;
pub mod write;

pub use crate::{
    read::{Read, ReadableMulti, Reader},
    write::{Write, Writer},
};

pub trait Endianness {
    fn read<T: Read>(buf: &mut Reader) -> read::Result<T>;
    fn write<T: Write>(n: &T, buf: &mut Writer) -> write::Result;
}

#[derive(Debug)]
pub struct BigEndian;
impl Endianness for BigEndian {
    fn read<T: Read>(buf: &mut Reader) -> read::Result<T> {
        T::read_be(buf)
    }

    fn write<T: Write>(n: &T, buf: &mut Writer) -> write::Result {
        n.write_be(buf)
    }
}

#[derive(Debug)]
pub struct LittleEndian;
impl Endianness for LittleEndian {
    fn read<T: Read>(buf: &mut Reader) -> read::Result<T> {
        T::read_le(buf)
    }

    fn write<T: Write>(n: &T, buf: &mut Writer) -> write::Result {
        n.write_le(buf)
    }
}

#[cfg(feature = "derive")]
pub use binbuf_derive::{Read, Write};

#[cfg(feature = "macros")]
pub mod macros {
    pub use binbuf_macros::bytes_written;
}
