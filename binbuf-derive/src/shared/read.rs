use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Error, ExprPath, LitStr, Result};

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

/// This generates the Readable trait impl.
pub fn gen_readable_impl(
    struct_name: &Ident,
    read_inner: TokenStream,
    error: ExprPath,
) -> TokenStream {
    let doc_header = format!(" Read [`{struct_name}`] from a [`ReadBuffer`].");
    let doc_func = format!(
        " let {} = {}::read::<BigEndian>(&mut buf).unwrap();",
        struct_name.to_string().to_lowercase(),
        struct_name
    );

    quote! {
        impl binbuf::read::Readable for #struct_name {
            type Error = #error;
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
            fn read<E: binbuf::Endianness>(buf: &mut binbuf::read::ReadBuffer) -> Result<Self, Self::Error> {
                #read_inner
            }
        }
    }
}

pub fn gen_readable_verify_impl(struct_name: &Ident, endianness: LitStr) -> Result<TokenStream> {
    if endianness.value().is_empty() {
        return Ok(quote! {});
    }

    let supported_endianness = match endianness.value().as_str() {
        "little" => quote! { binbuf::SupportedEndianness::LittleEndian },
        "big" => quote! { binbuf::SupportedEndianness::BigEndian },
        "both" => quote! { binbuf::SupportedEndianness::Both },
        _ => {
            return Err(Error::new(
                endianness.span(),
                format!(
                    "Invalid supported endianness - expected 'big', 'little' or 'both', got {}",
                    endianness.value()
                ),
            ))
        }
    };

    Ok(quote! {
        impl binbuf::read::ReadableVerify for #struct_name {
            const SUPPORTED_ENDIANNESS: binbuf::SupportedEndianness = #supported_endianness;
        }
    })
}
