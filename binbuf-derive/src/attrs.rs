use proc_macro2::Span;
use structmeta::StructMeta;
use syn::{Attribute, Error, LitStr};

#[derive(StructMeta)]
pub struct RawStructReadAttrs {
    pub error: Option<LitStr>,
    pub endianness: Option<LitStr>,
}

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

impl From<Option<RawStructReadAttrs>> for StructReadAttrs {
    fn from(value: Option<RawStructReadAttrs>) -> Self {
        match value {
            Some(attrs) => {
                let mut struct_attrs = StructReadAttrs::default();

                if attrs.endianness.is_some() {
                    struct_attrs.endianness = attrs.endianness.unwrap();
                }

                if attrs.error.is_some() {
                    struct_attrs.error = attrs.error.unwrap();
                }

                struct_attrs
            }
            None => StructReadAttrs::default(),
        }
    }
}

impl StructReadAttrs {
    pub fn parse(attrs: Vec<Attribute>) -> Result<StructReadAttrs, Error> {
        let mut struct_attrs: Option<RawStructReadAttrs> = None;

        for attr in attrs {
            if !attr.path.is_ident("binbuf") {
                continue;
            }

            struct_attrs = Some(attr.parse_args::<RawStructReadAttrs>()?);
        }

        Ok(StructReadAttrs::from(struct_attrs))
    }
}

#[derive(StructMeta)]
pub struct RawEnumReadAttrs {
    error: Option<LitStr>,
    endianness: Option<LitStr>,
    repr: LitStr,
}

pub struct EnumReadAttrs {
    error: LitStr,
    endianness: LitStr,
    repr: LitStr,
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

impl From<Option<RawEnumReadAttrs>> for EnumReadAttrs {
    fn from(value: Option<RawEnumReadAttrs>) -> Self {
        match value {
            Some(attrs) => {
                let mut enum_attrs = EnumReadAttrs::default();

                if attrs.endianness.is_some() {
                    enum_attrs.endianness = attrs.endianness.unwrap();
                }

                if attrs.error.is_some() {
                    enum_attrs.error = attrs.error.unwrap();
                }

                enum_attrs.repr = attrs.repr;

                enum_attrs
            }
            None => EnumReadAttrs::default(),
        }
    }
}

impl EnumReadAttrs {
    pub fn parse(attrs: Vec<Attribute>) -> Result<EnumReadAttrs, Error> {
        let mut struct_attrs: Option<RawEnumReadAttrs> = None;

        for attr in attrs {
            if !attr.path.is_ident("binbuf") {
                continue;
            }

            struct_attrs = Some(attr.parse_args::<RawEnumReadAttrs>()?);
        }

        Ok(EnumReadAttrs::from(struct_attrs))
    }
}
