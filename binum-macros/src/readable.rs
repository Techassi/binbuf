use quote::{format_ident, quote};

use proc_macro2::{Ident, Span, TokenStream};
use syn::{Data, DataStruct, DeriveInput, Error, FieldsNamed, Result, Type};

/// Expand the `Readable` derive macro by returning the generated
/// [`TokenStream`].
pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;

    // First make sure we have a struct. Return the struct data
    let struct_data = match is_struct(input.data) {
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
    let named_fields = match extract_named_fields(struct_data) {
        Some(f) => f,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "The target struct only supports named fields",
            ))
        }
    };

    // These are the allowed / supported types which we can read from the byte slice
    let allowed_types = vec!["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"];

    // Prepare the indivual parts of the code gen
    let mut try_funcs: Vec<TokenStream> = Vec::new();
    let mut funcs: Vec<TokenStream> = Vec::new();
    let mut inner: Vec<TokenStream> = Vec::new();

    // Iterate over the fields and generate the required code
    for field in named_fields.named {
        let field_name = &field.ident.unwrap();

        let field_type = match extract_last_path_segment_ident(field.ty) {
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

        try_funcs.push(gen_try_read_func(&var_name, &field_type));
        funcs.push(gen_read_func(&var_name, &field_type));

        inner.push(quote! {
            #field_name: #var_name,
        })
    }

    Ok(quote! {
        impl #struct_name {
            pub fn try_read_from(data: &Vec<u8>, endianness: binum::Endianness) -> Result<Self, binum::BinaryError> {
                let mut offset = 0;

                #(#try_funcs)*

                return Ok(Self {
                    #(#inner)*
                })
            }

            pub fn read_from(data: &Vec<u8>, endianness: binum::Endianness) -> Self {
                let mut offset = 0;

                #(#funcs)*

                return Self {
                    #(#inner)*
                }
            }
        }
    })
}

/// Checks if the provided [`Data`] is a struct and if yes, returns the struct
/// data as [`DataStruct`].
fn is_struct(data: Data) -> Option<DataStruct> {
    match data {
        Data::Struct(s) => Some(s),
        _ => None,
    }
}

fn extract_named_fields(strukt: DataStruct) -> Option<FieldsNamed> {
    match strukt.fields {
        syn::Fields::Named(f) => Some(f),
        _ => None,
    }
}

fn extract_last_path_segment_ident(ty: Type) -> Option<Ident> {
    match ty {
        Type::Path(p) => {
            if let Some(last) = p.path.segments.last() {
                return Some(last.ident.to_owned());
            }
            None
        }
        _ => None,
    }
}

fn gen_try_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    let seek_fn_name = format_ident!("try_seek_{}", field_type);

    quote! {
        let #var_name = match binum::#seek_fn_name(data, &mut offset, endianness) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
    }
}

fn gen_read_func(var_name: &Ident, field_type: &Ident) -> TokenStream {
    let seek_fn_name = format_ident!("seek_{}", field_type);

    quote! {
        let #var_name = binum::#seek_fn_name(data, &mut offset, endianness);
    }
}
