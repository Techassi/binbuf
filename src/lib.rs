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
//! let b = &[69, 88, 65, 77, 80, 76, 69, 33];
//! let mut b = ReadBuffer::new(b);
//!
//! match u16::read::<BigEndian>(&mut b) {
//!     Ok(n) => assert_eq!(n, 17752),
//!     Err(err) => panic!("{}", err),
//! }
//! ```

use std::fmt::Display;

mod impls;
pub mod read;
pub mod write;

pub use read::*;
pub use write::*;

#[derive(Debug, PartialEq)]
pub enum SupportedEndianness {
    BigEndian,
    LittleEndian,
    Both,
}

impl Display for SupportedEndianness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedEndianness::BigEndian => write!(f, "big endian"),
            SupportedEndianness::LittleEndian => write!(f, "little endian"),
            SupportedEndianness::Both => write!(f, "big+little endian"),
        }
    }
}

pub trait Endianness {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool;

    fn read<T: FromBuffer>(buf: &mut ReadBuffer) -> ReadResult<T>;
    fn write<T: IntoBuffer>(n: T, buf: &mut WriteBuffer) -> usize;
}

#[derive(Debug)]
pub struct BigEndian {}
impl Endianness for BigEndian {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool {
        match supported {
            SupportedEndianness::BigEndian => true,
            SupportedEndianness::LittleEndian => false,
            SupportedEndianness::Both => true,
        }
    }

    fn read<T: FromBuffer>(buf: &mut ReadBuffer) -> ReadResult<T> {
        T::as_be(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut WriteBuffer) -> usize {
        n.as_be(buf)
    }
}

#[derive(Debug)]
pub struct LittleEndian {}
impl Endianness for LittleEndian {
    fn is_in_supported_endianness_set(supported: SupportedEndianness) -> bool {
        match supported {
            SupportedEndianness::BigEndian => false,
            SupportedEndianness::LittleEndian => true,
            SupportedEndianness::Both => true,
        }
    }

    fn read<T: FromBuffer>(buf: &mut ReadBuffer) -> ReadResult<T> {
        T::as_le(buf)
    }

    fn write<T: IntoBuffer>(n: T, buf: &mut WriteBuffer) -> usize {
        n.as_le(buf)
    }
}

#[cfg(feature = "derive")]
pub use binbuf_derive::{Readable, Writeable};

#[cfg(feature = "macros")]
pub use binbuf_macros::bytes_written;
