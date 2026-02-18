use syn::Type;

use vercel_rpc_cli::model::RustType;
use vercel_rpc_cli::parser::types::{extract_rust_type, extract_struct_fields};

fn parse_type(s: &str) -> Type {
    syn::parse_str(s).expect("failed to parse type")
}

#[test]
fn simple_types() {
    assert_eq!(
        extract_rust_type(&parse_type("String")),
        RustType::simple("String")
    );
    assert_eq!(
        extract_rust_type(&parse_type("i32")),
        RustType::simple("i32")
    );
    assert_eq!(
        extract_rust_type(&parse_type("bool")),
        RustType::simple("bool")
    );
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
    assert_eq!(ty.to_string(), "Vec<String>");

    let nested = RustType::with_generics(
        "HashMap",
        vec![
            RustType::simple("String"),
            RustType::with_generics("Vec", vec![RustType::simple("i32")]),
        ],
    );
    assert_eq!(nested.to_string(), "HashMap<String, Vec<i32>>");
}
