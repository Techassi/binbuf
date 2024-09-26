use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, DataEnum, DataStruct,
    DeriveInput, Error, ExprPath, Field, Result as SynResult,
};

use crate::{
    attrs::{
        AttrsParse, EnumReadAttrs, FieldAttrs, RawContainerAttrs, RawFieldAttrs, StructReadAttrs,
    },
    shared,
};

/// Expand the `Readable` derive macro by returning the generated
/// [`TokenStream`].
pub fn expand(input: DeriveInput) -> SynResult<TokenStream> {
    match input.data {
        syn::Data::Struct(s) => expand_struct(s, &input.ident, input.attrs),
        syn::Data::Enum(e) => expand_enum(e, &input.ident, input.attrs),
        syn::Data::Union(_) => Err(Error::new(
            Span::call_site(),
            "The Readable derive macro can only be used with structs or enums",
        )),
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
    let struct_attrs = RawContainerAttrs::parse::<StructReadAttrs>(struct_attrs)?;

    // TODO (Techassi): Make this always a loop to simplify field attr parsing
    let read_inner = match gen_struct_fields(named_fields) {
        Ok(ts) => ts,
        Err(err) => return Err(err),
    };

    // Validate the struct args
    let readable_error: ExprPath = struct_attrs.error.parse()?;

    // Generate trait impls
    let readable_impl = shared::gen_readable_impl(struct_name, read_inner, readable_error);
    // let readable_verify_impl =
    //     shared::gen_readable_verify_impl(struct_name, struct_attrs.endianness)?;

    Ok(quote! {
        #readable_impl
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
    let enum_attrs = RawContainerAttrs::parse::<EnumReadAttrs>(enum_attrs)?;
    let error: ExprPath = enum_attrs.error.parse()?;
    let repr: ExprPath = enum_attrs.repr.parse()?;
    // println!("{enum_attrs:?}");

    let read_inner = quote! {
        Self::try_from(#repr::read::<E>(buf)?)
    };

    // Implement From<REPR> for ENUM
    let from_repr_impl = gen_from_repr_impl_enum(enum_name, &enum_data, &enum_attrs)?;
    let readable_impl = shared::gen_readable_impl(enum_name, read_inner, error);
    // let readable_verify_impl = shared::gen_readable_verify_impl(enum_name, enum_attrs.endianness)?;
    // let from_enum_impl = gen_from_enum_impl_repr(enum_name, &enum_data, &enum_attrs)?;

    Ok(quote! {
        #from_repr_impl
        #readable_impl
    })
}

fn gen_from_repr_impl_enum(
    enum_name: &Ident,
    enum_data: &DataEnum,
    enum_attrs: &EnumReadAttrs,
) -> SynResult<TokenStream> {
    let error: ExprPath = enum_attrs.error.parse()?;
    let repr: ExprPath = enum_attrs.repr.parse()?;

    let repr_type = repr.path.get_ident().unwrap().to_string();

    let mut variants: Vec<TokenStream> = Vec::new();

    for (index, variant) in (0_u128..).zip((&enum_data.variants).into_iter()) {
        let variant_ident = &variant.ident;
        let variant_value = match repr_type.as_str() {
            "u8" => Literal::u8_suffixed(index as u8),
            "u16" => Literal::u16_suffixed(index as u16),
            "u32" => Literal::u32_suffixed(index as u32),
            "u64" => Literal::u64_suffixed(index as u64),
            "u128" => Literal::u128_suffixed(index),
            _ => {
                return Err(Error::new(
                    variant.span(),
                    "Invalid variant representation type",
                ))
            }
        };

        variants.push(quote! {
            #variant_value => Ok(Self::#variant_ident),
        });
    }

    // TODO (Techassi): This should not be hardcoded. Introduce the possibility to specify an error variant
    variants.push(quote! {
        _ => Err(Self::Error::InvalidData),
    });

    Ok(quote! {
        #[automatically_derived]
        impl TryFrom<#repr> for #enum_name {
            type Error = #error;

            fn try_from(value: #repr) -> Result<Self, Self::Error> {
                match value {
                    #(#variants)*
                }
            }
        }
    })
}

// fn gen_from_enum_impl_repr(
//     enum_name: &Ident,
//     enum_data: &DataEnum,
//     enum_attrs: &EnumReadAttrs,
// ) -> SynResult<TokenStream> {
//     let repr: ExprPath = enum_attrs.repr.parse()?;
//     let repr_type = repr.path.get_ident().unwrap().to_string();

//     let mut variants: Vec<TokenStream> = Vec::new();

//     for (index, variant) in (0_u128..).zip((&enum_data.variants).into_iter()) {
//         let variant_ident = &variant.ident;
//         let variant_value = match repr_type.as_str() {
//             "u8" => Literal::u8_suffixed(index as u8),
//             "u16" => Literal::u16_suffixed(index as u16),
//             "u32" => Literal::u32_suffixed(index as u32),
//             "u64" => Literal::u64_suffixed(index as u64),
//             "u128" => Literal::u128_suffixed(index),
//             _ => {
//                 return Err(Error::new(
//                     variant.span(),
//                     "Invalid variant representation type",
//                 ))
//             }
//         };

//         variants.push(quote! {
//             #enum_name::#variant_ident => #variant_value,
//         });
//     }

//     Ok(quote! {
//         impl From<#enum_name> for #repr {
//             fn from(value: #enum_name) -> Self {
//                 match value {
//                     #(#variants)*
//                 }
//             }
//         }
//     })
// }

/// This generates code when there are multiple named fields in the struct.
fn gen_struct_fields(fields: Punctuated<Field, Comma>) -> SynResult<TokenStream> {
    // Here we need ensure the ReadableMulti trait is implemented, how can we achieve that?
    // For now, we just generate a read call for each of the fields
    let mut funcs: Vec<TokenStream> = Vec::new();
    let mut inner: Vec<TokenStream> = Vec::new();

    for field in fields {
        // Extract field attrs
        let attrs = RawFieldAttrs::parse::<FieldAttrs>(field.attrs.clone())?;

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

        // Either generate a read function, or use default when skip_read=true
        let func = if attrs.skip_read.value {
            shared::gen_default_func(&var_name, &field_type)
        } else {
            shared::gen_read_func(&var_name, &field_type)
        };

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
