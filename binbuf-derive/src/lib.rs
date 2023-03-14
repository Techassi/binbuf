use proc_macro::TokenStream;
use syn::{DeriveInput, Error};

mod attrs;
mod readable;
mod shared;
mod writeable;

#[proc_macro_derive(Readable, attributes(binbuf))]
/// Annotating a struct or enum with the derive macro [`Readable`] automatically
/// implements the [`Readable`] trait and all its required functions and types.
/// The trait provides a convenient method to read data from a (network) byte
/// slice and construct the target struct based on the read values. This macro
/// is only available when the `derive` feature is used.
///
/// ### Example
///
/// ```
/// use binbuf::prelude::*
///
/// #[derive(Read)]
/// pub struct Target {
///     a: u16,
///     b: u16,
/// }
///
/// let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut b = ReadBuffer::new(b.as_slice());
///
/// let t = Target::read::<BigEndian>(&mut b).unwrap();
///
/// assert_eq!(t.a, 17752);
/// assert_eq!(t.b, 16717);
/// ```
pub fn readable_macro_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    readable::expand(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Writeable)]
/// Annotating a struct or enum with the derive macro [`Writeable`]
/// automatically implements the [`Writeable`] trait and all its required
/// functions and types. This macro is only available when the `derive` feature
/// is used.
///
/// ### Example
///
/// ```
/// use binbuf::prelude::*
///
/// #[derive(Write)]
/// pub struct Source {
///     a: u16,
///     b: u16,
/// }
///
/// let mut b = WriteBuffer::new();
/// let s = Source {
///     a: 17752,
///     b: 16717,
/// }
///
/// let n = s.write::<BigEndian>(&mut b).unwrap();
///
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// assert_eq!(n, 4);
/// ```
pub fn writeable_macro_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    writeable::expand(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
