use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Result};

use crate::shared;

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;

    // First make sure we have a struct. Return the struct data
    let struct_data = match shared::is_struct(input.data) {
        Some(s) => s,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "The Writeable derive macro can only be used with structs",
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
                "The source struct only supports named fields",
            ))
        }
    };

    // These are the allowed / supported types which we can read from the byte slice
    let allowed_types = vec!["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"];

    // Prepare the individual parts of the code gen
    let mut funcs: Vec<TokenStream> = Vec::new();

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

        funcs.push(gen_write_func(&field_name, &field_type));
    }

    Ok(quote! {
        impl #struct_name {
            pub fn write_into(&self, buf: &mut [u8], endianness: binum::Endianness) -> binum::BinaryWriteResult {
                let mut offset = 0;

                #(#funcs)*

                Ok(offset)
            }
        }
    })
}

fn gen_write_func(field_name: &Ident, field_type: &Ident) -> TokenStream {
    let seek_fn_name = format_ident!("write_seek_{}", field_type);

    quote! {
        match binum::write::#seek_fn_name(self.#field_name, buf, &mut offset, endianness) {
            Err(err) => return Err(err),
            _=> {},
        };
    }
}
