use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Error, Field, Result};

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

    if named_fields.is_empty() {
        return Ok(quote! {});
    }

    let c: TokenStream = if named_fields.len() == 1 {
        match gen_one_field(named_fields.first().unwrap(), struct_name) {
            Ok(ts) => ts,
            Err(err) => return Err(err),
        }
    } else {
        match gen_multiple_fields(named_fields, struct_name) {
            Ok(ts) => ts,
            Err(err) => return Err(err),
        }
    };

    Ok(quote! {
        use binum::WriteExt;

        impl #struct_name {
            pub fn write_into<E>(&self, mut buf: &mut [u8]) -> binum::BinaryWriteResult
            where
                E: binum::Endianness
            {
                #c
            }
        }
    })
}

fn gen_one_field(field: &Field, struct_ident: &Ident) -> Result<TokenStream> {
    // Extract the field name
    let field_name = field.ident.as_ref().unwrap();

    // Extract the field type and also check if it is an allowed type
    let field_type = match shared::extract_allowed_field_type(&field.ty, struct_ident) {
        Ok(t) => t,
        Err(err) => return Err(err),
    };

    let func = shared::gen_write_func(field_name, &field_type);

    Ok(quote! {
        #func
    })
}

fn gen_multiple_fields(
    fields: Punctuated<Field, Comma>,
    struct_ident: &Ident,
) -> Result<TokenStream> {
    let entries = match shared::extract_continuous_field_types(fields, struct_ident) {
        Ok(e) => e,
        Err(err) => return Err(err),
    };

    // Prepare the individual parts of the code gen
    let mut funcs: Vec<TokenStream> = Vec::new();

    for entry in entries {
        if entry.count == 1 {
            let field_type = &entry.ty;
            let field_name = &entry.idents[0];

            funcs.push(shared::gen_io_write_func(field_name, field_type));
            continue;
        }

        let field_type = &entry.ty;
        funcs.push(shared::gen_io_multi_write_func(entry.idents, field_type));
    }

    Ok(quote! {
        let mut n = 0;

        #(#funcs)*

        Ok(n)
    })
}
