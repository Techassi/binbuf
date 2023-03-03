use proc_macro2::Span;
use syn::{Error, LitStr};

use crate::attrs::{RawContainerAttrs, TryFromAttrs};

pub struct StructReadAttrs {
    pub error: LitStr,
    pub endianness: LitStr,
}

impl Default for StructReadAttrs {
    fn default() -> Self {
        Self {
            error: LitStr::new("binbuf::error::BufferError", Span::call_site()),
            endianness: LitStr::new("both", Span::call_site()),
        }
    }
}

impl TryFromAttrs<RawContainerAttrs> for StructReadAttrs {
    fn try_from(value: Option<RawContainerAttrs>, _span: Span) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match value {
            Some(attrs) => {
                let mut struct_attrs = Self::default();

                if attrs.endianness.is_some() {
                    struct_attrs.endianness = attrs.endianness.unwrap();
                }

                if attrs.error.is_some() {
                    struct_attrs.error = attrs.error.unwrap();
                }

                Ok(struct_attrs)
            }
            None => Ok(Self::default()),
        }
    }
}
