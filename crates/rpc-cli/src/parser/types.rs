use syn::{Fields, FieldsNamed, Type};

use crate::model::RustType;

/// Converts a `syn::Type` into our `RustType` representation.
///
/// Handles paths (simple and generic), references, tuples, arrays, and unit.
/// Unknown or unsupported types fall back to their token representation.
pub fn extract_rust_type(ty: &Type) -> RustType {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path
                .path
                .segments
                .last()
                .expect("Type::Path always has at least one segment");
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

/// Extracts named fields from a struct definition into `(name, RustType)` pairs.
pub fn extract_struct_fields(fields: &Fields) -> Vec<(String, RustType)> {
    match fields {
        Fields::Named(FieldsNamed { named, .. }) => named
            .iter()
            .filter_map(|f| {
                let name = f.ident.as_ref()?.to_string();
                let ty = extract_rust_type(&f.ty);
                Some((name, ty))
            })
            .collect(),
        _ => vec![],
    }
}
