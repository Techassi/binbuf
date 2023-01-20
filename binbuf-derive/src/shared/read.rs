use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// This generates a single read function call.
pub fn gen_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        let #var_name = #field_type::read::<E>(buf)?;
    }
}

/// This generates a read function call which reads multiple values from a
/// buffer.
pub fn gen_multi_read_func(var_name: &Ident, field_type: &Ident, nints: usize) -> TokenStream {
    quote! {
        let #var_name = #field_type::read_multi::<E, #nints>(buf)?;
    }
}
