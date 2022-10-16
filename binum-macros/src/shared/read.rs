use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// This generates a single read function call.
pub fn gen_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        let #var_name = match E::read::<#field_type>(buf) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
    }
}

/// This generates a read function call which reads a single value from a
/// reader, e.g. a Cursor.
pub fn gen_cursor_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        let #var_name = match reader.read_from::<#field_type, E>() {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
    }
}

/// This generates a read function call which reads multiple values from a
/// reader, e.g. a Cursor.
pub fn gen_cursor_multi_read_func(
    var_name: &Ident,
    field_type: &Ident,
    nints: usize,
) -> TokenStream {
    quote! {
        let #var_name = match reader.read_multi::<#field_type, E>(#nints) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
    }
}
