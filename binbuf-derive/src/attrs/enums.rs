use proc_macro2::Span;
use structmeta::StructMeta;
use syn::{spanned::Spanned, Attribute, Error, LitStr};

#[derive(StructMeta)]
pub struct RawEnumReadAttrs {
    error: Option<LitStr>,
    endianness: Option<LitStr>,
    repr: Option<LitStr>,
}

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

impl EnumReadAttrs {
    pub fn parse(attrs: Vec<Attribute>) -> Result<EnumReadAttrs, Error> {
        let mut struct_attrs: Option<RawEnumReadAttrs> = None;
        let mut span = Span::call_site();

        for attr in attrs {
            if !attr.path.is_ident("binbuf") {
                continue;
            }

            struct_attrs = Some(attr.parse_args::<RawEnumReadAttrs>()?);
            span = attr.span()
        }

        EnumReadAttrs::try_from(struct_attrs, span)
    }

    fn try_from(value: Option<RawEnumReadAttrs>, span: Span) -> Result<EnumReadAttrs, Error> {
        match value {
            Some(attrs) => {
                let mut enum_attrs = EnumReadAttrs::default();

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
            None => Ok(EnumReadAttrs::default()),
        }
    }
}
