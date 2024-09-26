use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::ExprPath;

/// This generates a single read function call.
pub fn gen_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        let #var_name = #field_type::read::<E>(buf)?;
    }
}

pub fn gen_default_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    quote! {
        let #var_name = <#field_type as ::std::default::Default>::default();
    }
}

/// This generates the Readable trait impl.
pub fn gen_readable_impl(
    struct_name: &Ident,
    read_inner: TokenStream,
    _error: ExprPath,
) -> TokenStream {
    let doc_header = format!(" Read [`{struct_name}`] from a [`ReadBuffer`].");
    let doc_func = format!(
        " let {} = {}::read::<BigEndian>(&mut buf).unwrap();",
        struct_name.to_string().to_lowercase(),
        struct_name
    );

    quote! {
        #[automatically_derived]
        impl ::binbuf::read::Read for #struct_name {
            #[doc = #doc_header]
            ///
            /// ### Example
            ///
            /// ```
            /// use binbuf::prelude::*;
            ///
            /// let mut buf = ReadBuffer::new(&data[..]);
            #[doc = #doc_func]
            /// ```
            fn read<E: ::binbuf::Endianness>(buf: &mut ::binbuf::read::Reader) -> ::binbuf::read::Result<Self> {
                #read_inner
            }
        }
    }
}
