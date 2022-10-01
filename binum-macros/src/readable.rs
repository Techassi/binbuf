use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Result};

use crate::shared;

/// Expand the `Readable` derive macro by returning the generated
/// [`TokenStream`].
pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;

    // First make sure we have a struct. Return the struct data
    let struct_data = match shared::is_struct(input.data) {
        Some(s) => s,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "The Readable derive macro can only be used with structs",
            ))
        }
    };

    // Extract all named fields. THis will return an error if there are unnamed
    // fields present
    let named_fields = match shared::extract_named_fields(struct_data) {
        Some(f) => f.named,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "The target struct only supports named fields",
            ))
        }
    };

    // These are the allowed / supported types which we can read from the byte slice
    let allowed_types = vec!["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"];

    // Prepare the individual parts of the code gen
    let mut funcs: Vec<TokenStream> = Vec::new();
    let mut inner: Vec<TokenStream> = Vec::new();

    // Iterate over the fields and generate the required code
    for field in named_fields {
        let field_name = &field.ident.unwrap();

        let field_type = match shared::extract_last_path_segment_ident(field.ty) {
            Some(li) => li,
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "Failed to extract ident from field type",
                ))
            }
        };

        // There has to be a better way to do this, right?
        if !allowed_types.contains(&field_type.to_string().as_str()) {
            return Err(Error::new(
                Span::call_site(),
                format!(
                    "Invalid type found in struct '{}'. Only {:?} allowed",
                    input.ident, allowed_types
                ),
            ));
        }

        let var_name = format_ident!("_gen_{}", field_name);

        funcs.push(gen_read_func(&var_name, &field_type));

        inner.push(quote! {
            #field_name: #var_name,
        })
    }

    Ok(quote! {
        impl #struct_name {
            /// Read data from the byte slice and populate each of teh struct fields.
            pub fn read_from(data: &Vec<u8>, endianness: binum::Endianness) -> Result<Self, binum::BinaryError> {
                let mut offset = 0;

                #(#funcs)*

                return Ok(Self {
                    #(#inner)*
                })
            }
        }
    })
}

fn gen_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    let seek_fn_name = format_ident!("read_seek_{}", field_type);

    quote! {
        let #var_name = match binum::#seek_fn_name(data, &mut offset, endianness) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
    }
}
