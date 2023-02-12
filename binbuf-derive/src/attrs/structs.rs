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
