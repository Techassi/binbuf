use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use structmeta::StructMeta;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, DeriveInput, Error, ExprPath, Field, LitStr,
    Result,
};

use crate::shared;

#[derive(StructMeta)]
struct ReadStructAttrs {
    error: Option<LitStr>,
    endianness: Option<LitStr>,
}

impl Default for ReadStructAttrs {
    fn default() -> Self {
        Self {
            error: Some(LitStr::new("binbuf::error::BufferError", Span::call_site())),
            endianness: Some(LitStr::new("both", Span::call_site())),
        }
    }
}

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

    // Extract all named fields. This will return an error if there are unnamed
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

    if named_fields.is_empty() {
        return Ok(quote! {});
    }

    // Parse struct attributes
    let struct_attrs = parse_struct_attributes(input.attrs)?;

    let read_inner: TokenStream = if named_fields.len() == 1 {
        match gen_one_field(named_fields.first().unwrap()) {
            Ok(ts) => ts,
            Err(err) => return Err(err),
        }
    } else {
        match gen_multiple_fields(named_fields) {
            Ok(ts) => ts,
            Err(err) => return Err(err),
        }
    };

    // Validate the struct args
    let readable_error: ExprPath = struct_attrs.error.unwrap().parse()?;

    // Generate trait impls
    let readable_impl = shared::gen_readable_impl(struct_name, read_inner, readable_error);
    let readable_verify_impl =
        shared::gen_readable_verify_impl(struct_name, struct_attrs.endianness.unwrap())?;

    Ok(quote! {
        #readable_impl
        #readable_verify_impl
    })
}

/// This generates code when there is only one named field in the struct.
fn gen_one_field(field: &Field) -> Result<TokenStream> {
    // Extract the field name
    let field_name = field.ident.as_ref().unwrap();

    // Extract the field type
    let field_type = match shared::extract_last_path_segment_ident(&field.ty) {
        Some(t) => t,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "Failed to extract ident from field type",
            ))
        }
    };

    // Construct the variable name
    let var_name = format_ident!("_gen_{}", field_name);
    let func = shared::gen_read_func(&var_name, &field_type);

    Ok(quote! {
        #func

        return Ok(Self {
            #field_name: #var_name,
        })
    })
}

/// This generates code when there are multiple named fields in the struct.
fn gen_multiple_fields(fields: Punctuated<Field, Comma>) -> Result<TokenStream> {
    // Here we need esnure the ReadableMulti trait is implemented, how can we achieve that?
    // For now, we just generate a read call for each of the fields
    let mut funcs: Vec<TokenStream> = Vec::new();
    let mut inner: Vec<TokenStream> = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let var_name = format_ident!("_gen_{}", field_name);

        let field_type = match shared::extract_last_path_segment_ident(&field.ty) {
            Some(t) => t,
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "Failed to extract ident from field type",
                ))
            }
        };

        let func = shared::gen_read_func(&var_name, &field_type);

        funcs.push(func);
        inner.push(quote! {
            #field_name: #var_name,
        })
    }

    Ok(quote! {
        #(#funcs)*

        return Ok(Self {
            #(#inner)*
        })
    })

    // let entries = match shared::extract_continuous_field_types(fields, struct_ident) {
    //     Ok(e) => e,
    //     Err(err) => return Err(err),
    // };

    // // Prepare the individual parts of the code gen
    // let mut funcs: Vec<TokenStream> = Vec::new();
    // let mut inner: Vec<TokenStream> = Vec::new();

    // for entry in entries {
    //     if entry.count == 1 {
    //         let field_type = &entry.ty;
    //         let field_name = &entry.idents[0];

    //         let var_name = format_ident!("_gen_{}", field_name);

    //         funcs.push(shared::gen_read_func(&var_name, field_type));
    //         inner.push(quote! {
    //             #field_name: #var_name,
    //         });
    //         continue;
    //     }

    //     let mut fields = String::new();
    //     let field_type = &entry.ty;

    //     for ident in &entry.idents {
    //         fields.push_str(&ident.to_string())
    //     }

    //     let var_name = format_ident!("_gen_multi_{}", fields);
    //     funcs.push(shared::gen_multi_read_func(
    //         &var_name,
    //         field_type,
    //         entry.count,
    //     ));

    //     for i in 0..entry.count {
    //         let field_name = &entry.idents[i];

    //         inner.push(quote! {
    //             #field_name: #var_name[#i],
    //         })
    //     }
    // }

    // Ok(quote! {
    //     #(#funcs)*

    //     return Ok(Self {
    //         #(#inner)*
    //     })
    // })
}

fn parse_struct_attributes(attrs: Vec<Attribute>) -> Result<ReadStructAttrs> {
    let mut struct_attrs = ReadStructAttrs::default();

    for attr in attrs {
        if !attr.path.is_ident("binbuf") {
            continue;
        }

        let attr = attr.parse_args::<ReadStructAttrs>().unwrap();

        if attr.error.is_some() {
            struct_attrs.error = attr.error;
        }

        if attr.endianness.is_some() {
            struct_attrs.endianness = attr.endianness;
        }
    }

    Ok(struct_attrs)
}
