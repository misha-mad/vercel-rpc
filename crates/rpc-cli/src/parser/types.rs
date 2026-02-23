use syn::{Fields, FieldsNamed, FieldsUnnamed, Type};

use super::serde as serde_attr;
use crate::model::{FieldDef, RustType};

/// Converts a `syn::Type` into our `RustType` representation.
///
/// Handles paths (simple and generic), references, tuples, arrays, and unit.
/// Unknown or unsupported types fall back to their token representation.
pub fn extract_rust_type(ty: &Type) -> RustType {
    match ty {
        Type::Path(type_path) => {
            let Some(segment) = type_path.path.segments.last() else {
                let token_str = quote::quote!(#ty).to_string();
                return RustType::simple(token_str);
            };
            let name = segment.ident.to_string();
            let generics = extract_generic_args(&segment.arguments);

            if generics.is_empty() {
                RustType::simple(name)
            } else {
                RustType::with_generics(name, generics)
            }
        }

        Type::Reference(type_ref) => extract_rust_type(&type_ref.elem),

        Type::Tuple(tuple) => {
            if tuple.elems.is_empty() {
                RustType::simple("()")
            } else {
                let inner: Vec<RustType> = tuple.elems.iter().map(extract_rust_type).collect();
                RustType::with_generics("tuple", inner)
            }
        }

        Type::Array(array) => {
            let elem = extract_rust_type(&array.elem);
            RustType::with_generics("Array", vec![elem])
        }

        Type::Slice(slice) => {
            let elem = extract_rust_type(&slice.elem);
            RustType::with_generics("Array", vec![elem])
        }

        _ => {
            let token_str = quote::quote!(#ty).to_string();
            RustType::simple(token_str)
        }
    }
}

/// Extracts generic type arguments from a path segment (e.g. `<String, i32>`).
fn extract_generic_args(arguments: &syn::PathArguments) -> Vec<RustType> {
    match arguments {
        syn::PathArguments::AngleBracketed(args) => args
            .args
            .iter()
            .filter_map(|arg| match arg {
                syn::GenericArgument::Type(ty) => Some(extract_rust_type(ty)),
                _ => None,
            })
            .collect(),
        _ => vec![],
    }
}

/// Extracts unnamed (tuple) fields from a struct into `RustType` values.
///
/// Returns an empty vec for named or unit structs.
pub fn extract_tuple_fields(fields: &Fields) -> Vec<RustType> {
    match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.iter().map(|f| extract_rust_type(&f.ty)).collect()
        }
        _ => vec![],
    }
}

/// Extracts named fields from a struct/variant into `FieldDef` values,
/// including serde attributes (`rename`, `skip`, `default`).
pub fn extract_struct_fields(fields: &Fields) -> Vec<FieldDef> {
    match fields {
        Fields::Named(FieldsNamed { named, .. }) => named
            .iter()
            .filter_map(|f| {
                let name = f.ident.as_ref()?.to_string();
                let ty = extract_rust_type(&f.ty);
                let rename = serde_attr::parse_rename(&f.attrs);
                let skip = serde_attr::is_skipped(&f.attrs);
                let has_default = serde_attr::has_default(&f.attrs);
                let flatten = serde_attr::is_flattened(&f.attrs);
                Some(FieldDef {
                    name,
                    ty,
                    rename,
                    skip,
                    has_default,
                    flatten,
                })
            })
            .collect(),
        _ => vec![],
    }
}
