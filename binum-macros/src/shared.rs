use proc_macro2::Ident;
use syn::{Data, DataStruct, FieldsNamed, Type};

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
pub fn extract_last_path_segment_ident(ty: Type) -> Option<Ident> {
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
