pub use binum_macros::*;

macro_rules! from_bytes_trait_impl {
    ($SelfT:ty, $Size:expr, $ErrVar:expr) => {
        impl $crate::FromBytes for $SelfT {
            const ERR_VARIANT: $crate::BinaryErrorVariant = $ErrVar;
            const SIZE: usize = $Size;

            fn from_le_bytes(bytes: &[u8]) -> Self {
                Self::from_le_bytes(bytes.try_into().unwrap())
            }

            fn from_be_bytes(bytes: &[u8]) -> Self {
                Self::from_be_bytes(bytes.try_into().unwrap())
            }
        }
    };
}

macro_rules! into_bytes_trait_impl {
    ($SelfT:ty, $Size:expr, $ErrVar:expr) => {
        impl $crate::IntoBytes for $SelfT {
            const ERR_VARIANT: $crate::BinaryErrorVariant = $ErrVar;
            const SIZE: usize = $Size;

            fn to_le_bytes(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }

            fn to_be_bytes(self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }
    };
}

macro_rules! from_and_into_bytes_trait_impl {
    ($SelfT:ty, $Size:expr, $ErrVar:expr) => {
        $crate::macros::from_bytes_trait_impl!($SelfT, $Size, $ErrVar);
        $crate::macros::into_bytes_trait_impl!($SelfT, $Size, $ErrVar);
    };
}

macro_rules! endianness_impl {
    ($End:ty, $FromFn:ident, $ToFn:ident) => {
        impl Endianness for $End {
            fn read<T: $crate::FromBytes>(buf: &[u8]) -> $crate::BinaryReadResult<T> {
                if buf.len() < T::SIZE {
                    return Err($crate::BinaryError::new(
                        "Slice of bytes too short",
                        T::ERR_VARIANT,
                    ));
                }

                let n = T::$FromFn(&buf[..T::SIZE]);
                Ok(n)
            }

            fn read_multi<T: $crate::FromBytes>(
                buf: &[u8],
                nints: usize,
            ) -> $crate::BinaryReadResult<Vec<T>> {
                if buf.len() < T::SIZE * nints {
                    return Err($crate::BinaryError::new(
                        "Slice of bytes too short",
                        T::ERR_VARIANT,
                    ));
                }

                let mut v = Vec::with_capacity(nints);

                for i in 0..nints {
                    match Self::read(&buf[i * T::SIZE..]) {
                        Ok(n) => v.push(n),
                        Err(err) => return Err(err),
                    };
                }

                Ok(v)
            }

            fn write<T: $crate::IntoBytes>(n: T, buf: &mut [u8]) -> $crate::BinaryWriteResult {
                if buf.len() < T::SIZE {
                    return Err($crate::BinaryError::new("Buf too short", T::ERR_VARIANT));
                }

                let bytes = n.$ToFn();

                for (i, b) in bytes.into_iter().enumerate() {
                    buf[i] = b;
                }

                Ok(T::SIZE)
            }

            fn write_multi<T: $crate::IntoBytes>(
                n: Vec<T>,
                buf: &mut [u8],
            ) -> $crate::BinaryWriteResult {
                let required_len = n.len() * T::SIZE;
                if buf.len() > required_len {
                    return Err($crate::BinaryError::new("Buf too short", T::ERR_VARIANT));
                }

                for (i, n) in n.into_iter().enumerate() {
                    match Self::write(n, &mut buf[i * T::SIZE..]) {
                        Err(err) => return Err(err),
                        _ => {}
                    }
                }

                Ok(required_len)
            }
        }
    };
}

pub(crate) use from_and_into_bytes_trait_impl;
pub(crate) use from_bytes_trait_impl;
pub(crate) use into_bytes_trait_impl;

pub(crate) use endianness_impl;
