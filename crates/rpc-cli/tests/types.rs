use syn::Type;

use vercel_rpc_cli::model::{RenameRule, RustType};
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
fn extract_named_struct_fields() {
    let item: syn::ItemStruct = syn::parse_str("struct Foo { name: String, age: u32 }").unwrap();
    let fields = extract_struct_fields(&item.fields);
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[0].ty, RustType::simple("String"));
    assert_eq!(fields[1].name, "age");
    assert_eq!(fields[1].ty, RustType::simple("u32"));
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

// --- RenameRule tests ---

#[test]
fn rename_rule_camel_case_from_snake() {
    assert_eq!(RenameRule::CamelCase.apply("first_name"), "firstName");
    assert_eq!(RenameRule::CamelCase.apply("created_at_ms"), "createdAtMs");
    assert_eq!(RenameRule::CamelCase.apply("id"), "id");
}

#[test]
fn rename_rule_camel_case_from_pascal() {
    assert_eq!(RenameRule::CamelCase.apply("MyVariant"), "myVariant");
    assert_eq!(RenameRule::CamelCase.apply("UserLogin"), "userLogin");
}

#[test]
fn rename_rule_snake_case() {
    assert_eq!(RenameRule::SnakeCase.apply("first_name"), "first_name");
    assert_eq!(RenameRule::SnakeCase.apply("MyVariant"), "my_variant");
    assert_eq!(RenameRule::SnakeCase.apply("UserLogin"), "user_login");
}

#[test]
fn rename_rule_pascal_case() {
    assert_eq!(RenameRule::PascalCase.apply("first_name"), "FirstName");
    assert_eq!(RenameRule::PascalCase.apply("MyVariant"), "MyVariant");
}

#[test]
fn rename_rule_screaming_snake_case() {
    assert_eq!(
        RenameRule::ScreamingSnakeCase.apply("first_name"),
        "FIRST_NAME"
    );
    assert_eq!(
        RenameRule::ScreamingSnakeCase.apply("MyVariant"),
        "MY_VARIANT"
    );
}

#[test]
fn rename_rule_kebab_case() {
    assert_eq!(RenameRule::KebabCase.apply("first_name"), "first-name");
    assert_eq!(RenameRule::KebabCase.apply("MyVariant"), "my-variant");
}

#[test]
fn rename_rule_screaming_kebab_case() {
    assert_eq!(
        RenameRule::ScreamingKebabCase.apply("first_name"),
        "FIRST-NAME"
    );
    assert_eq!(
        RenameRule::ScreamingKebabCase.apply("MyVariant"),
        "MY-VARIANT"
    );
}

#[test]
fn rename_rule_lowercase() {
    assert_eq!(RenameRule::Lowercase.apply("first_name"), "firstname");
    assert_eq!(RenameRule::Lowercase.apply("MyVariant"), "myvariant");
}

#[test]
fn rename_rule_uppercase() {
    assert_eq!(RenameRule::Uppercase.apply("first_name"), "FIRSTNAME");
    assert_eq!(RenameRule::Uppercase.apply("MyVariant"), "MYVARIANT");
}

#[test]
fn rename_rule_empty_string() {
    assert_eq!(RenameRule::CamelCase.apply(""), "");
    assert_eq!(RenameRule::SnakeCase.apply(""), "");
}

#[test]
fn rename_rule_single_word() {
    assert_eq!(RenameRule::CamelCase.apply("name"), "name");
    assert_eq!(RenameRule::PascalCase.apply("name"), "Name");
    assert_eq!(RenameRule::ScreamingSnakeCase.apply("name"), "NAME");
}

#[test]
fn rename_rule_from_str() {
    assert_eq!(
        "camelCase".parse::<RenameRule>().unwrap(),
        RenameRule::CamelCase
    );
    assert_eq!(
        "snake_case".parse::<RenameRule>().unwrap(),
        RenameRule::SnakeCase
    );
    assert_eq!(
        "PascalCase".parse::<RenameRule>().unwrap(),
        RenameRule::PascalCase
    );
    assert_eq!(
        "SCREAMING_SNAKE_CASE".parse::<RenameRule>().unwrap(),
        RenameRule::ScreamingSnakeCase
    );
    assert_eq!(
        "kebab-case".parse::<RenameRule>().unwrap(),
        RenameRule::KebabCase
    );
    assert_eq!(
        "SCREAMING-KEBAB-CASE".parse::<RenameRule>().unwrap(),
        RenameRule::ScreamingKebabCase
    );
    assert_eq!(
        "lowercase".parse::<RenameRule>().unwrap(),
        RenameRule::Lowercase
    );
    assert_eq!(
        "UPPERCASE".parse::<RenameRule>().unwrap(),
        RenameRule::Uppercase
    );
    assert!("unknown".parse::<RenameRule>().is_err());
}
