use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    read::{self, Read, Reader},
    write::{self, Write, WriteError, Writer},
    Endianness,
};

impl Read for Ipv4Addr {
    fn read_be(buf: &mut Reader) -> read::Result<Self> {
        let b = u32::read_be(buf)?;
        Ok(Self::from(b))
    }

    fn read_le(buf: &mut Reader) -> read::Result<Self> {
        let b = u32::read_le(buf)?;
        Ok(Self::from(b))
    }
}

impl Write for Ipv4Addr {
    fn write_be(&self, buf: &mut Writer) -> write::Result<usize> {
        let b = self.octets();
        Ok(buf.write(b))
    }

    fn write_le(&self, buf: &mut Writer) -> write::Result<usize> {
        let mut b = self.octets();
        b.reverse();
        Ok(buf.write(b))
    }
}

impl Read for Ipv6Addr {
    fn read_be(buf: &mut Reader) -> read::Result<Self> {
        let b = u128::read_be(buf)?;
        Ok(Self::from(b))
    }

    fn read_le(buf: &mut Reader) -> read::Result<Self> {
        let b = u128::read_le(buf)?;
        Ok(Self::from(b))
    }
}

impl Write for Ipv6Addr {
    fn write_be(&self, buf: &mut Writer) -> write::Result<usize> {
        let b = self.octets();
        Ok(buf.write(b))
    }

    fn write_le(&self, buf: &mut Writer) -> write::Result<usize> {
        let mut b = self.octets();
        b.reverse();
        Ok(buf.write(b))
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
