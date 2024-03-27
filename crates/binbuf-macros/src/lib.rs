#[macro_export]
macro_rules! from_buffer_and_readable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl crate::read::Read for $SelfT {
            fn read_be(r: &mut crate::read::Reader) -> crate::read::Result<Self> {
                let b = r.read_slice($Size)?;
                Ok(Self::from_be_bytes(b.try_into().unwrap()))
            }

            fn read_le(r: &mut crate::read::Reader) -> crate::read::Result<Self> {
                let b = r.read_slice($Size)?;
                Ok(Self::from_le_bytes(b.try_into().unwrap()))
            }
        }

        impl ReadableMulti for $SelfT {}
    };
}

#[macro_export]
macro_rules! into_buffer_and_writeable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl Write for $SelfT {
            fn write_be(&self, buf: &mut crate::write::Writer) -> crate::write::Result {
                let b = self.to_be_bytes();
                Ok(buf.write(b))
            }

            fn write_le(&self, buf: &mut crate::write::Writer) -> crate::write::Result {
                let b = self.to_le_bytes();
                Ok(buf.write(b))
            }
        }
    };
}

#[macro_export]
macro_rules! bytes_written {
    ($($Fn:expr);+) => {
        {
            let mut __n = 0;
            $(
                __n += $Fn;
            )+
            __n
        }
    };
}
