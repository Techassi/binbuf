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

pub(crate) use from_and_into_bytes_trait_impl;
pub(crate) use from_bytes_trait_impl;
pub(crate) use into_bytes_trait_impl;
