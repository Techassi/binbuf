use proc_macro2::Span;
use syn::{Error, LitBool, LitStr};

use crate::attrs::{RawContainerAttrs, RawFieldAttrs, TryFromAttrs};

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

pub struct FieldAttrs {
    pub skip_write: LitBool,
    pub skip_read: LitBool,
}

impl Default for FieldAttrs {
    fn default() -> Self {
        Self {
            skip_write: LitBool::new(false, Span::call_site()),
            skip_read: LitBool::new(false, Span::call_site()),
        }
    }
}

impl TryFromAttrs<RawFieldAttrs> for FieldAttrs {
    fn try_from(value: Option<RawFieldAttrs>, span: Span) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match value {
            Some(attrs) => {
                let mut field_attrs = Self::default();

                if attrs.skip.is_some() && (attrs.skip_write.is_some() || attrs.skip_read.is_some())
                {
                    return Err(Error::new(
                        span,
                        "Setting both 'skip' and 'skip_write' / 'skip_read' is not supported",
                    ));
                }

                if attrs.skip.is_some() {
                    field_attrs.skip_write = LitBool::new(true, span);
                    field_attrs.skip_read = LitBool::new(true, span);
                }

                if attrs.skip_write.is_some() {
                    field_attrs.skip_write = LitBool::new(true, span);
                }

                if attrs.skip_read.is_some() {
                    field_attrs.skip_read = LitBool::new(true, span);
                }

                Ok(field_attrs)
            }
            None => Ok(Self::default()),
        }
    }
}
