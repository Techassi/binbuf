use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataStruct, Error, Field, FieldsNamed, Type,
};

mod read;
mod write;

pub use read::*;
pub use write::*;

#[derive(Debug)]
pub struct TyEntry {
    pub idents: Vec<Ident>,
    pub count: usize,
    pub ty: Ident,
}

/// Checks if the provided [`Data`] is a struct and if yes, returns the struct
/// data as [`DataStruct`].
pub fn is_struct(data: Data) -> Option<DataStruct> {
    match data {
        Data::Struct(s) => Some(s),
        _ => None,
    }
}

/// Extracts all named fields of a struct
pub fn extract_named_fields(strukt: DataStruct) -> Option<FieldsNamed> {
    match strukt.fields {
        syn::Fields::Named(f) => Some(f),
        _ => None,
    }
}

/// Extract the last path segment ident. This is useful to retrieve the type
/// of a struct field.
pub fn extract_last_path_segment_ident(ty: &Type) -> Option<Ident> {
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

pub fn extract_continuous_field_types(
    fields: Punctuated<Field, Comma>,
    struct_ident: &Ident,
) -> Result<Vec<TyEntry>, Error> {
    let mut entries: Vec<TyEntry> = Vec::new();

    for field in fields {
        let field_type = match extract_last_path_segment_ident(&field.ty) {
            Some(t) => t,
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "Failed to extract ident from field type",
                ))
            }
        };

        if entries.is_empty() {
            entries.push(TyEntry {
                idents: vec![field.ident.unwrap()],
                count: 1,
                ty: field_type,
            });
            continue;
        }

        let i = entries.len() - 1;

        if entries[i].ty == field_type {
            entries[i].idents.push(field.ident.unwrap());
            entries[i].count += 1;
            continue;
        }

        entries.push(TyEntry {
            idents: vec![field.ident.unwrap()],
            count: 1,
            ty: field_type,
        })
    }

    Ok(entries)
}
