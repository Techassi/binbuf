use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Error, Field, Result};

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

    let doc_header = format!(" Read [`{}`] from a [`ReadBuffer`].", struct_name);
    let doc_func = format!(
        " let {} = {}::read::<BigEndian>(&mut buf).unwrap();",
        struct_name.to_string().to_lowercase(),
        struct_name
    );

    Ok(quote! {
        impl Readable for #struct_name {
            type Error = BufferError;
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
            fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
                #c
            }
        }
    })
}

/// This generates code when there is only one named field in the struct.
fn gen_one_field(field: &Field, struct_ident: &Ident) -> Result<TokenStream> {
    // Extract the field name
    let field_name = field.ident.as_ref().unwrap();

    // Extract the field type and also check if it is an allowed type
    let field_type = match shared::extract_allowed_field_type(&field.ty, struct_ident) {
        Ok(t) => t,
        Err(err) => return Err(err),
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
    let mut inner: Vec<TokenStream> = Vec::new();

    for entry in entries {
        if entry.count == 1 {
            let field_type = &entry.ty;
            let field_name = &entry.idents[0];

            let var_name = format_ident!("_gen_{}", field_name);

            funcs.push(shared::gen_read_func(&var_name, field_type));
            inner.push(quote! {
                #field_name: #var_name,
            });
            continue;
        }

        let mut fields = String::new();
        let field_type = &entry.ty;

        for ident in &entry.idents {
            fields.push_str(&ident.to_string())
        }

        let var_name = format_ident!("_gen_multi_{}", fields);
        funcs.push(shared::gen_multi_read_func(
            &var_name,
            field_type,
            entry.count,
        ));

        for i in 0..entry.count {
            let field_name = &entry.idents[i];

            inner.push(quote! {
                #field_name: #var_name[#i],
            })
        }
    }

    Ok(quote! {
        #(#funcs)*

        return Ok(Self {
            #(#inner)*
        })
    })
}
