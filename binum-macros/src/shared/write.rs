use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// This generates a single write function call.
pub fn gen_write_func(field_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        return match E::write::<#field_type>(self.#field_name, buf) {
            Ok(n) => Ok(n),
            Err(err) => Err(err),
        };
    }
}

pub fn gen_io_write_func(field_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        match buf.write_into::<#field_type, E>(self.#field_name) {
            Ok(b) => n += b,
            Err(err) => return Err(err),
        }
    }
}

pub fn gen_io_multi_write_func(field_names: Vec<Ident>, field_type: &Ident) -> TokenStream {
    let mut fields: Vec<TokenStream> = Vec::new();

    for name in field_names {
        fields.push(quote! {self.#name})
    }

    quote! {
        match buf.write_multi::<#field_type, E>(vec![#(#fields),*]) {
            Ok(b) => n += b,
            Err(err) => return Err(err),
        }
    }
}
