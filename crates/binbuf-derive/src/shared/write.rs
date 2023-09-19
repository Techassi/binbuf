use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// This generates a single write function call.
pub fn gen_write_func(field_name: &Ident) -> TokenStream {
    quote! {
        self.#field_name.write::<E>(buf)
    }
}

pub fn gen_multi_write_func(field_name: &Ident) -> TokenStream {
    quote! {
        n += self.#field_name.write::<E>(buf)?;
    }
}
