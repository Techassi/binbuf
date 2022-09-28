use proc_macro::TokenStream;
use syn::{DeriveInput, Error};

mod readable;
mod shared;
mod writeable;

#[proc_macro_derive(Readable)]
/// Annotating a struct with the derive macro [`Readable`] adds the `read_from`
/// functions which provides a convenient method to read data from a (network)
/// byte slice and construct the target struct based on the read values.
///
/// ### Example
///
/// ```
/// use binum::macros::Readable;
///
/// #[derive(Readable)]
/// pub struct Target {
///     a: u16,
///     b: u16,
/// }
///
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let t = Target::read_from(&d, binum::Endianness::Big);
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
pub fn writeable_macro_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    writeable::expand(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
