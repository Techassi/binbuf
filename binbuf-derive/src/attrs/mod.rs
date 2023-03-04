use proc_macro2::Span;
use structmeta::StructMeta;
use syn::{parse::Parse, spanned::Spanned, Attribute, Error, LitBool, LitStr};

mod enums;
mod structs;

pub use enums::*;
pub use structs::*;

pub trait TryFromAttrs<T> {
    fn try_from(value: Option<T>, span: Span) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait AttrsParse: Sized + Parse {
    fn parse<T>(attrs: Vec<Attribute>) -> Result<T, Error>
    where
        T: TryFromAttrs<Self> + Default,
    {
        let mut parsed_attrs: Option<Self> = None;
        let mut span = Span::call_site();

        for attr in attrs {
            if !attr.path.is_ident("binbuf") {
                continue;
            }

            parsed_attrs = Some(attr.parse_args::<Self>()?);
            span = attr.span()
        }

        T::try_from(parsed_attrs, span)
    }
}

#[derive(Debug, StructMeta)]
pub struct RawContainerAttrs {
    endianness: Option<LitStr>,
    error: Option<LitStr>,
    repr: Option<LitStr>,
}

impl AttrsParse for RawContainerAttrs {}

#[derive(Debug, StructMeta)]
pub struct RawFieldAttrs {
    skip_write: Option<LitBool>,
    skip_read: Option<LitBool>,
    skip: Option<LitBool>,
}

impl AttrsParse for RawFieldAttrs {}
