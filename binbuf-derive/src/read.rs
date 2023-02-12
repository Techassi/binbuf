use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, DataEnum, DataStruct, DeriveInput, Error,
    ExprPath, Field, Result as SynResult,
};

use crate::{
    attrs::{EnumReadAttrs, StructReadAttrs},
    shared,
};

/// Expand the `Readable` derive macro by returning the generated
/// [`TokenStream`].
pub fn expand(input: DeriveInput) -> SynResult<TokenStream> {
    match input.data {
        syn::Data::Struct(s) => expand_struct(s, &input.ident, input.attrs),
        syn::Data::Enum(e) => expand_enum(e, &input.ident, input.attrs),
        syn::Data::Union(_) => {
            return Err(Error::new(
                Span::call_site(),
                "The Readable derive macro can only be used with structs or enums",
            ))
        }
    }
}

fn expand_struct(
    struct_data: DataStruct,
    struct_name: &Ident,
    struct_attrs: Vec<Attribute>,
) -> SynResult<TokenStream> {
    // Extract all named fields. This will return an error if there are unnamed
    // fields present
    let named_fields = match shared::extract_named_fields_from_struct(struct_data) {
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
    let struct_attrs = StructReadAttrs::parse(struct_attrs)?;

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
    let readable_error: ExprPath = struct_attrs.error.parse()?;

    // Generate trait impls
    let readable_impl = shared::gen_readable_impl(struct_name, read_inner, readable_error);
    let readable_verify_impl =
        shared::gen_readable_verify_impl(struct_name, struct_attrs.endianness)?;

    Ok(quote! {
        #readable_impl
        #readable_verify_impl
    })
}

fn expand_enum(
    enum_data: DataEnum,
    enum_name: &Ident,
    enum_attrs: Vec<Attribute>,
) -> SynResult<TokenStream> {
    // If there are no variants, we don't generate code
    if enum_data.variants.is_empty() {
        return Ok(quote! {});
    }

    // Parse enum attributes
    let enum_attrs = EnumReadAttrs::parse(enum_attrs)?;

    Ok(quote! {})
}

/// This generates code when there is only one named field in the struct.
fn gen_one_field(field: &Field) -> SynResult<TokenStream> {
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
fn gen_multiple_fields(fields: Punctuated<Field, Comma>) -> SynResult<TokenStream> {
    // Here we need ensure the ReadableMulti trait is implemented, how can we achieve that?
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
}
