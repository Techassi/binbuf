use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Error, Field, Result};

use crate::{
    attrs::{AttrsParse, FieldAttrs, RawFieldAttrs},
    shared,
};

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

    // Extract all named fields. This will return an error if there are unnamed
    // fields present
    let named_fields = match shared::extract_named_fields_from_struct(struct_data) {
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

    let doc_header = format!(" Write [`{struct_name}`] to a [`WriteBuffer`].");
    let doc_func = format!(
        " {}.write::<BigEndian>(&mut buf).unwrap();",
        struct_name.to_string().to_lowercase(),
    );

    Ok(quote! {
        #[automatically_derived]
        impl Writeable for #struct_name {
            type Error = BufferError;

            #[doc = #doc_header]
            ///
            /// ### Example
            ///
            /// ```
            /// use binbuf::prelude::*;
            ///
            /// let mut buf = WriteBuffer::new();
            #[doc = #doc_func]
            /// ```
            fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error>
            {
                #c
            }
        }
    })
}

fn gen_one_field(field: &Field) -> Result<TokenStream> {
    // Extract the field name
    let field_name = field.ident.as_ref().unwrap();
    let func = shared::gen_write_func(field_name);

    Ok(quote! {
        #func
    })
}

fn gen_multiple_fields(fields: Punctuated<Field, Comma>) -> Result<TokenStream> {
    // Prepare the individual parts of the code gen
    let mut funcs: Vec<TokenStream> = Vec::new();

    for field in fields {
        // Extract field attrs
        let attrs = RawFieldAttrs::parse::<FieldAttrs>(field.attrs.clone())?;

        if attrs.skip_write.value {
            continue;
        }

        let field_name = field.ident.as_ref().unwrap();
        funcs.push(shared::gen_multi_write_func(field_name));
    }

    Ok(quote! {
        let mut n = 0;

        #(#funcs)*

        Ok(n)
    })
}
