use proc_macro2::Span;
use structmeta::StructMeta;
use syn::{spanned::Spanned, Attribute, Error, LitStr};

mod enums;
mod structs;

pub use enums::*;
pub use structs::*;

pub trait TryFromAttrs<T> {
    fn try_from(value: Option<T>, span: Span) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(StructMeta)]
pub struct RawContainerAttrs {
    endianness: Option<LitStr>,
    error: Option<LitStr>,
    repr: Option<LitStr>,
}

impl RawContainerAttrs {
    pub fn parse<C>(attrs: Vec<Attribute>) -> Result<C, Error>
    where
        C: TryFromAttrs<RawContainerAttrs> + Default,
    {
        let mut container_attrs: Option<Self> = None;
        let mut span = Span::call_site();

        for attr in attrs {
            if !attr.path.is_ident("binbuf") {
                continue;
            }

            container_attrs = Some(attr.parse_args::<Self>()?);
            span = attr.span()
        }

        C::try_from(container_attrs, span)
    }
}
