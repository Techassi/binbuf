#[macro_export]
macro_rules! from_buffer_and_readable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl crate::FromReader for $SelfT {
            const SIZE: usize = $Size;

            fn as_be(r: &mut crate::read::Reader) -> crate::read::Result<Self> {
                let b = r.read_slice(Self::SIZE)?;
                Ok(Self::from_be_bytes(b.try_into().unwrap()))
            }

            fn as_le(r: &mut crate::read::Reader) -> crate::read::Result<Self> {
                let b = r.read_slice(Self::SIZE)?;
                Ok(Self::from_le_bytes(b.try_into().unwrap()))
            }
        }

        impl crate::read::Read for $SelfT {
            fn read<E: Endianness>(r: &mut crate::read::Reader) -> crate::read::Result<Self> {
                E::read(r)
            }
        }

        impl ReadableMulti for $SelfT {}
    };
}

#[macro_export]
macro_rules! into_buffer_and_writeable_impl {
    ($SelfT:ty, $Size:expr) => {
        impl IntoWriter for $SelfT {
            const SIZE: usize = $Size;

            fn as_be(&self, buf: &mut crate::write::Writer) -> usize {
                let b = self.to_be_bytes();
                buf.write(b)
            }

            fn as_le(&self, buf: &mut crate::write::Writer) -> usize {
                let b = self.to_le_bytes();
                buf.write(b)
            }
        }

        impl Write for $SelfT {
            fn write<E: Endianness>(&self, buf: &mut crate::write::Writer) -> crate::write::Result {
                Ok(E::write(*self, buf))
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
