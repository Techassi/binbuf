use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    prelude::*,
    read::FromBuffer,
    write::{IntoBuffer, WriteableVerify},
};

impl FromBuffer for Ipv4Addr {
    const SIZE: usize = 4;

    fn as_be(buf: &mut impl ToReadBuffer) -> ReadBufferResult<Self> {
        let b = u32::read_be(buf)?;
        Ok(Self::from(b))
    }

    fn as_le(buf: &mut impl ToReadBuffer) -> ReadBufferResult<Self> {
        let b = u32::read_le(buf)?;
        Ok(Self::from(b))
    }
}

impl Readable for Ipv4Addr {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        E::read(buf)
    }
}

impl ReadableVerify for Ipv4Addr {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;
}

impl IntoBuffer for Ipv4Addr {
    const SIZE: usize = 4;

    fn as_be(&self, buf: &mut impl ToWriteBuffer) -> usize {
        let b = self.octets();
        buf.write(b)
    }

    fn as_le(&self, buf: &mut impl ToWriteBuffer) -> usize {
        let mut b = self.octets();
        b.reverse();
        buf.write(b)
    }
}

impl Writeable for Ipv4Addr {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        Ok(E::write(*self, buf))
    }
}

impl WriteableVerify for Ipv4Addr {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;
}

impl FromBuffer for Ipv6Addr {
    const SIZE: usize = 16;

    fn as_be(buf: &mut impl ToReadBuffer) -> ReadBufferResult<Self> {
        let b = u128::read_be(buf)?;
        Ok(Self::from(b))
    }

    fn as_le(buf: &mut impl ToReadBuffer) -> ReadBufferResult<Self> {
        let b = u128::read_le(buf)?;
        Ok(Self::from(b))
    }
}

impl Readable for Ipv6Addr {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        E::read(buf)
    }
}

impl ReadableVerify for Ipv6Addr {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;
}

impl IntoBuffer for Ipv6Addr {
    const SIZE: usize = 16;

    fn as_be(&self, buf: &mut impl ToWriteBuffer) -> usize {
        let b = self.octets();
        buf.write(b)
    }

    fn as_le(&self, buf: &mut impl ToWriteBuffer) -> usize {
        let mut b = self.octets();
        b.reverse();
        buf.write(b)
    }
}

impl Writeable for Ipv6Addr {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        Ok(E::write(*self, buf))
    }
}

impl WriteableVerify for Ipv6Addr {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;
}
