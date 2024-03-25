use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    read::{self, FromReader, Read, Reader},
    write::{self, IntoWriter, Write, WriteError, Writer},
    Endianness, SupportedEndianness,
};

impl FromReader for Ipv4Addr {
    const SIZE: usize = 4;

    fn as_be(buf: &mut Reader) -> read::Result<Self> {
        let b = u32::read_be(buf)?;
        Ok(Self::from(b))
    }

    fn as_le(buf: &mut Reader) -> read::Result<Self> {
        let b = u32::read_le(buf)?;
        Ok(Self::from(b))
    }
}

impl Read for Ipv4Addr {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;

    fn read<E: Endianness>(buf: &mut Reader) -> read::Result<Self> {
        E::read(buf)
    }
}

impl IntoWriter for Ipv4Addr {
    const SIZE: usize = 4;

    fn as_be(&self, buf: &mut Writer) -> usize {
        let b = self.octets();
        buf.write(b)
    }

    fn as_le(&self, buf: &mut Writer) -> usize {
        let mut b = self.octets();
        b.reverse();
        buf.write(b)
    }
}

impl Write for Ipv4Addr {
    fn write<E: Endianness>(&self, buf: &mut Writer) -> write::Result {
        Ok(E::write(*self, buf))
    }
}

impl FromReader for Ipv6Addr {
    const SIZE: usize = 16;

    fn as_be(buf: &mut Reader) -> read::Result<Self> {
        let b = u128::read_be(buf)?;
        Ok(Self::from(b))
    }

    fn as_le(buf: &mut Reader) -> read::Result<Self> {
        let b = u128::read_le(buf)?;
        Ok(Self::from(b))
    }
}

impl Read for Ipv6Addr {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::Both;

    fn read<E: Endianness>(buf: &mut Reader) -> read::Result<Self> {
        E::read(buf)
    }
}

impl IntoWriter for Ipv6Addr {
    const SIZE: usize = 16;

    fn as_be(&self, buf: &mut Writer) -> usize {
        let b = self.octets();
        buf.write(b)
    }

    fn as_le(&self, buf: &mut Writer) -> usize {
        let mut b = self.octets();
        b.reverse();
        buf.write(b)
    }
}

impl Write for Ipv6Addr {
    fn write<E: Endianness>(&self, buf: &mut Writer) -> write::Result {
        Ok(E::write(*self, buf))
    }
}

impl Write for String {
    fn write<E: Endianness>(&self, buf: &mut Writer) -> write::Result {
        if !self.is_ascii() {
            return Err(WriteError::NonAsciiData);
        }

        Ok(buf.write(self.as_bytes()))
    }
}
