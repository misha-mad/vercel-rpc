use syn::{Type, Fields, FieldsNamed};

use crate::model::RustType;

/// Converts a `syn::Type` into our `RustType` representation.
///
/// Handles paths (simple and generic), references, tuples, arrays, and unit.
/// Unknown or unsupported types fall back to their token representation.
pub fn extract_rust_type(ty: &Type) -> RustType {
    match ty {
        Type::Path(type_path) => {
            // A parsed Type::Path always has at least one segment
            let segment = &type_path.path.segments[type_path.path.segments.len() - 1];
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_type(s: &str) -> Type {
        syn::parse_str(s).expect("failed to parse type")
    }

    #[test]
    fn simple_types() {
        assert_eq!(extract_rust_type(&parse_type("String")), RustType::simple("String"));
        assert_eq!(extract_rust_type(&parse_type("i32")), RustType::simple("i32"));
        assert_eq!(extract_rust_type(&parse_type("bool")), RustType::simple("bool"));
    }

    #[test]
    fn generic_types() {
        let ty = extract_rust_type(&parse_type("Vec<String>"));
        assert_eq!(ty.name, "Vec");
        assert_eq!(ty.generics.len(), 1);
        assert_eq!(ty.generics[0].name, "String");
    }

    #[test]
    fn nested_generics() {
        let ty = extract_rust_type(&parse_type("Option<Vec<i32>>"));
        assert_eq!(ty.name, "Option");
        assert_eq!(ty.generics[0].name, "Vec");
        assert_eq!(ty.generics[0].generics[0].name, "i32");
    }

    #[test]
    fn hashmap_type() {
        let ty = extract_rust_type(&parse_type("HashMap<String, i64>"));
        assert_eq!(ty.name, "HashMap");
        assert_eq!(ty.generics.len(), 2);
        assert_eq!(ty.generics[0].name, "String");
        assert_eq!(ty.generics[1].name, "i64");
    }

    #[test]
    fn unit_tuple() {
        let ty = extract_rust_type(&parse_type("()"));
        assert_eq!(ty, RustType::simple("()"));
    }

    #[test]
    fn reference_type() {
        let ty = extract_rust_type(&parse_type("&str"));
        assert_eq!(ty.name, "str");
    }

    #[test]
    fn non_empty_tuple() {
        let ty = extract_rust_type(&parse_type("(String, i32)"));
        assert_eq!(ty.name, "tuple");
        assert_eq!(ty.generics.len(), 2);
        assert_eq!(ty.generics[0].name, "String");
        assert_eq!(ty.generics[1].name, "i32");
    }

    #[test]
    fn array_type() {
        let ty = extract_rust_type(&parse_type("[u8; 32]"));
        assert_eq!(ty.name, "Array");
        assert_eq!(ty.generics.len(), 1);
        assert_eq!(ty.generics[0].name, "u8");
    }

    #[test]
    fn slice_type() {
        // &[u8] is a reference to a slice
        let ty = extract_rust_type(&parse_type("&[u8]"));
        assert_eq!(ty.name, "Array");
        assert_eq!(ty.generics.len(), 1);
        assert_eq!(ty.generics[0].name, "u8");
    }

    #[test]
    fn fallback_type() {
        // Function pointer is not handled by specific arms
        let ty = extract_rust_type(&parse_type("fn() -> bool"));
        assert!(!ty.name.is_empty());
    }

    #[test]
    fn lifetime_generic_ignored() {
        // Cow<'a, str> — the lifetime generic should be filtered out
        let ty = extract_rust_type(&parse_type("Cow<'a, str>"));
        assert_eq!(ty.name, "Cow");
        assert_eq!(ty.generics.len(), 1);
        assert_eq!(ty.generics[0].name, "str");
    }

    #[test]
    fn extract_tuple_struct_fields() {
        let item: syn::ItemStruct = syn::parse_str("struct Wrapper(String);").unwrap();
        let fields = extract_struct_fields(&item.fields);
        assert!(fields.is_empty());
    }

    #[test]
    fn display_format() {
        let ty = RustType::with_generics("Vec", vec![RustType::simple("String")]);
        assert_eq!(ty.display(), "Vec<String>");

        let nested = RustType::with_generics(
            "HashMap",
            vec![RustType::simple("String"), RustType::with_generics("Vec", vec![RustType::simple("i32")])],
        );
        assert_eq!(nested.display(), "HashMap<String, Vec<i32>>");
    }
}
