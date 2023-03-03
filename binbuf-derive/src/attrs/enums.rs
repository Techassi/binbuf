use proc_macro2::Span;
use syn::{Error, LitStr};

use crate::attrs::{RawContainerAttrs, TryFromAttrs};

#[derive(Debug)]
pub struct EnumReadAttrs {
    pub error: LitStr,
    pub endianness: LitStr,
    pub repr: LitStr,
}

impl Default for EnumReadAttrs {
    fn default() -> Self {
        Self {
            error: LitStr::new("binbuf::error::BufferError", Span::call_site()),
            endianness: LitStr::new("both", Span::call_site()),
            repr: LitStr::new("u8", Span::call_site()),
        }
    }
}

impl TryFromAttrs<RawContainerAttrs> for EnumReadAttrs {
    fn try_from(value: Option<RawContainerAttrs>, span: Span) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match value {
            Some(attrs) => {
                let mut enum_attrs = Self::default();

                if attrs.endianness.is_some() {
                    enum_attrs.endianness = attrs.endianness.unwrap();
                }

                if attrs.error.is_some() {
                    enum_attrs.error = attrs.error.unwrap();
                }

                if attrs.repr.is_some() {
                    let repr = attrs.repr.unwrap();
                    if !["u8", "u16", "u32", "u64", "u128"].contains(&repr.value().as_str()) {
                        return Err(
                            Error::new(span, "Only u8, u16, u32, u64 and u128 are supported enum variant representations")
                        );
                    }
                }

                Ok(enum_attrs)
            }
            None => Ok(Self::default()),
        }
    }
}
