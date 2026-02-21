# RFC-3: Serde Attribute Support

- **Status:** Implemented
- **Topic:** Parse serde attributes for accurate TypeScript codegen
- **Date:** February 2026
- **PR:** [#40](https://github.com/misha-mad/vercel-rpc/pull/40)

## 1. Summary

The rpc-cli parser currently ignores `#[serde(...)]` attributes on structs, enums, fields, and variants. This means the generated TypeScript types may not match the actual JSON that Rust serializes at runtime. This RFC adds support for the most impactful serde attributes: `rename_all`, `rename`, `skip` / `skip_serializing`, and `default`.

## 2. Motivation

Today a struct like this produces incorrect TypeScript:

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserProfile {
    first_name: String,
    last_name: String,
    #[serde(rename = "DOB")]
    date_of_birth: String,
}
```

Current generated output (wrong):

```typescript
export interface UserProfile {
  first_name: string;
  last_name: string;
  date_of_birth: string;
}
```

Expected output:

```typescript
export interface UserProfile {
  firstName: string;
  lastName: string;
  DOB: string;
}
```

Similarly, `#[serde(skip)]` fields appear in the TypeScript interface even though they are never serialized, and `#[serde(rename_all)]` on enums is ignored.

## 3. Scope

### In scope (this RFC)

| Attribute          | Level          | Effect                                    |
|--------------------|----------------|-------------------------------------------|
| `rename_all`       | struct, enum   | Rename all fields/variants by convention  |
| `rename`           | field, variant | Override name for a single field/variant  |
| `skip`             | field          | Omit from generated TypeScript            |
| `skip_serializing` | field          | Omit from generated TypeScript            |
| `default`          | field          | Mark `Option<T>` fields as optional (`?`) |

### Out of scope (deferred to future RFCs)

- Enum representations: `tag`, `content`, `untagged` (Phase 3, Roadmap)
- `#[serde(flatten)]` (Phase 3)
- `skip_serializing_if` — runtime-only, no type-level impact
- `skip_deserializing` — no effect on serialization output
- `rename_all_fields` on enums — rare, can be added later
- `rename(serialize = "...", deserialize = "...")` split form — codegen only cares about serialization

## 4. Data Model Changes

### 4.1 New `FieldDef` struct

Replace the current `Vec<(String, RustType)>` field representation with a dedicated struct:

```rust
// model.rs

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    /// Original Rust field name.
    pub name: String,
    /// Parsed Rust type.
    pub ty: RustType,
    /// Explicit `#[serde(rename = "...")]` override.
    pub rename: Option<String>,
    /// Field is skipped in serialization (`#[serde(skip)]` or `#[serde(skip_serializing)]`).
    pub skip: bool,
    /// Field has `#[serde(default)]` — combined with `Option<T>` makes it optional in TS.
    pub has_default: bool,
}
```

### 4.2 Updates to `StructDef`

```rust
pub struct StructDef {
    pub name: String,
    pub fields: Vec<FieldDef>,              // was: Vec<(String, RustType)>
    pub rename_all: Option<RenameRule>,      // new
    pub source_file: PathBuf,
    pub docs: Option<String>,
}
```

### 4.3 Updates to `EnumVariant` and `EnumDef`

```rust
pub struct EnumVariant {
    pub name: String,
    pub kind: VariantKind,
    pub rename: Option<String>,             // new
}

pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub rename_all: Option<RenameRule>,      // new
    pub source_file: PathBuf,
    pub docs: Option<String>,
}
```

### 4.4 Update `VariantKind::Struct`

```rust
pub enum VariantKind {
    Unit,
    Tuple(Vec<RustType>),
    Struct(Vec<FieldDef>),                  // was: Vec<(String, RustType)>
}
```

### 4.5 `RenameRule` enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenameRule {
    CamelCase,
    SnakeCase,
    PascalCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
    Lowercase,
    Uppercase,
}
```

Each variant implements a `fn apply(&self, input: &str) -> String` method that transforms a Rust identifier (snake_case for fields, PascalCase for variants) into the target convention.

## 5. Parsing Changes

### 5.1 New `serde.rs` module

Create `crates/rpc-cli/src/parser/serde.rs` with helpers for extracting serde attributes from `syn` AST nodes:

```rust
/// Extract `#[serde(rename_all = "...")]` from container attributes.
pub fn parse_rename_all(attrs: &[Attribute]) -> Option<RenameRule>;

/// Extract `#[serde(rename = "...")]` from field/variant attributes.
pub fn parse_rename(attrs: &[Attribute]) -> Option<String>;

/// Check for `#[serde(skip)]` or `#[serde(skip_serializing)]`.
pub fn is_skipped(attrs: &[Attribute]) -> bool;

/// Check for `#[serde(default)]`.
pub fn has_default(attrs: &[Attribute]) -> bool;
```

All functions parse `#[serde(...)]` meta-list items. The general pattern:

```rust
fn parse_serde_attr<T>(attrs: &[Attribute], key: &str, extract: fn(&Meta) -> Option<T>) -> Option<T> {
    for attr in attrs {
        if !attr.path().is_ident("serde") { continue; }
        if let Meta::List(list) = &attr.meta {
            // Parse nested meta items and look for `key`
        }
    }
    None
}
```

### 5.2 Changes to `extract.rs`

**`extract_struct_fields`** — currently returns `Vec<(String, RustType)>`, change to `Vec<FieldDef>`:

```rust
pub fn extract_struct_fields(fields: &Fields) -> Vec<FieldDef> {
    match fields {
        Fields::Named(named) => named.named.iter().filter_map(|f| {
            let name = f.ident.as_ref()?.to_string();
            let ty = extract_rust_type(&f.ty)?;
            let rename = serde::parse_rename(&f.attrs);
            let skip = serde::is_skipped(&f.attrs);
            let has_default = serde::has_default(&f.attrs);
            Some(FieldDef { name, ty, rename, skip, has_default })
        }).collect(),
        _ => vec![],
    }
}
```

**Struct extraction** — add `rename_all` parsing:

```rust
let rename_all = serde::parse_rename_all(&item_struct.attrs);
manifest.structs.push(StructDef {
    name: item_struct.ident.to_string(),
    fields,
    rename_all,
    source_file: path.to_path_buf(),
    docs,
});
```

**`extract_enum_variants`** — add per-variant `rename` and struct variant `FieldDef`:

```rust
let rename = serde::parse_rename(&variant.attrs);
// For struct variants, use extract_struct_fields (returns Vec<FieldDef>)
```

**Enum extraction** — add `rename_all` parsing on the enum itself.

### 5.3 Changes to `parser.rs`

Add `pub mod serde;` to expose the new module.

## 6. Codegen Changes

### 6.1 Field name resolution

Add a helper that resolves the final TypeScript field name:

```rust
/// Resolve the serialized name for a struct field.
fn resolve_field_name(field: &FieldDef, container_rename_all: Option<RenameRule>, config_naming: FieldNaming) -> String {
    // Priority: field rename > container rename_all > config naming > original name
    if let Some(ref rename) = field.rename {
        return rename.clone();
    }
    if let Some(rule) = container_rename_all {
        return rule.apply(&field.name);
    }
    match config_naming {
        FieldNaming::CamelCase => to_camel_case(&field.name),
        FieldNaming::Preserve => field.name.clone(),
    }
}
```

Resolution priority (highest first):

```
#[serde(rename = "...")] > #[serde(rename_all = "...")] > codegen.naming.fields > original name
```

### 6.2 Variant name resolution

```rust
/// Resolve the serialized name for an enum variant.
fn resolve_variant_name(variant: &EnumVariant, container_rename_all: Option<RenameRule>) -> String {
    if let Some(ref rename) = variant.rename {
        return rename.clone();
    }
    if let Some(rule) = container_rename_all {
        return rule.apply(&variant.name);
    }
    variant.name.clone()
}
```

### 6.3 Skip handling

In `generate_interface` and enum struct variants, filter out skipped fields:

```rust
let visible_fields: Vec<&FieldDef> = fields.iter().filter(|f| !f.skip).collect();
```

### 6.4 Optional fields (`default` + `Option<T>`)

When a field has `has_default: true` and its type is `Option<T>`, emit `fieldName?: T | null` instead of `fieldName: T | null`:

```rust
if field.has_default && is_option_type(&field.ty) {
    writeln!(out, "  {}?: {};", resolved_name, inner_ts_type);
} else {
    writeln!(out, "  {}: {};", resolved_name, ts_type);
}
```

### 6.5 Interaction with `codegen.naming.fields`

The config-level `fields = "camelCase"` setting acts as a fallback. Serde attributes always take priority:

| serde attr                 | config naming | result                |
|----------------------------|---------------|-----------------------|
| `rename = "foo"`           | any           | `foo`                 |
| `rename_all = "camelCase"` | any           | camelCase from serde  |
| none                       | `camelCase`   | camelCase from config |
| none                       | `preserve`    | original name         |

## 7. `RenameRule` Transformation Logic

Each rule splits the input into words (by `_` for snake_case fields, by uppercase boundaries for PascalCase variants) and reassembles:

| Rule                   | Input `first_name` | Input `MyVariant` |
|------------------------|--------------------|-------------------|
| `camelCase`            | `firstName`        | `myVariant`       |
| `snake_case`           | `first_name`       | `my_variant`      |
| `PascalCase`           | `FirstName`        | `MyVariant`       |
| `SCREAMING_SNAKE_CASE` | `FIRST_NAME`       | `MY_VARIANT`      |
| `kebab-case`           | `first-name`       | `my-variant`      |
| `SCREAMING-KEBAB-CASE` | `FIRST-NAME`       | `MY-VARIANT`      |
| `lowercase`            | `firstname`        | `myvariant`       |
| `UPPERCASE`            | `FIRSTNAME`        | `MYVARIANT`       |

Implementation: a shared `fn split_words(input: &str) -> Vec<String>` that handles both snake_case and PascalCase inputs, then each rule joins words with its convention.

## 8. Examples

### `rename_all` on struct

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServiceStatus {
    uptime_secs: u64,
    api_version: String,
}
```

```typescript
export interface ServiceStatus {
  uptimeSecs: number;
  apiVersion: string;
}
```

### `rename` on field

```rust
#[derive(Serialize)]
struct Config {
    #[serde(rename = "apiKey")]
    api_key: String,
    debug: bool,
}
```

```typescript
export interface Config {
  apiKey: string;
  debug: boolean;
}
```

### `rename_all` on enum

```rust
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum EventKind {
    PageView,
    ButtonClick,
    FormSubmit,
}
```

```typescript
export type EventKind = "page_view" | "button_click" | "form_submit";
```

### `skip`

```rust
#[derive(Serialize)]
struct User {
    pub name: String,
    #[serde(skip)]
    secret: String,
    #[serde(skip_serializing)]
    password_hash: String,
}
```

```typescript
export interface User {
  name: string;
  // secret and password_hash omitted
}
```

### `default` + `Option<T>`

```rust
#[derive(Serialize, Deserialize)]
struct SearchParams {
    query: String,
    #[serde(default)]
    page: Option<u32>,
    #[serde(default)]
    limit: Option<u32>,
}
```

```typescript
export interface SearchParams {
  query: string;
  page?: number | null;
  limit?: number | null;
}
```

### Combined: `rename_all` + `rename` + `skip`

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserProfile {
    first_name: String,
    last_name: String,
    #[serde(rename = "DOB")]
    date_of_birth: String,
    #[serde(skip)]
    internal_id: u64,
}
```

```typescript
export interface UserProfile {
  firstName: string;
  lastName: string;
  DOB: string;
  // internal_id omitted
}
```

## 9. Files Modified

| File                                       | Action                                                                                    |
|--------------------------------------------|-------------------------------------------------------------------------------------------|
| `crates/rpc-cli/src/model.rs`              | Add `FieldDef`, `RenameRule`; update `StructDef`, `EnumDef`, `EnumVariant`, `VariantKind` |
| `crates/rpc-cli/src/parser/serde.rs`       | **New** — serde attribute parsing helpers                                                 |
| `crates/rpc-cli/src/parser.rs`             | Add `pub mod serde;`                                                                      |
| `crates/rpc-cli/src/parser/extract.rs`     | Use `FieldDef`, parse serde attrs on structs/enums/fields/variants                        |
| `crates/rpc-cli/src/codegen/typescript.rs` | Use `resolve_field_name`, skip logic, optional fields                                     |
| `crates/rpc-cli/src/codegen/client.rs`     | No changes expected (operates on procedure level)                                         |
| `crates/rpc-cli/tests/*.rs`                | Update existing tests for `FieldDef`, add serde-specific tests                            |

## 10. Test Plan

### Unit tests (parser/serde.rs)

| Test                            | Description                                               |
|---------------------------------|-----------------------------------------------------------|
| `parse_rename_all_camel_case`   | `#[serde(rename_all = "camelCase")]` returns `CamelCase`  |
| `parse_rename_all_snake_case`   | `#[serde(rename_all = "snake_case")]` returns `SnakeCase` |
| `parse_rename_all_all_variants` | All 8 rename rules parse correctly                        |
| `parse_rename_all_missing`      | No serde attr returns `None`                              |
| `parse_rename_value`            | `#[serde(rename = "foo")]` returns `Some("foo")`          |
| `is_skipped_skip`               | `#[serde(skip)]` returns `true`                           |
| `is_skipped_skip_serializing`   | `#[serde(skip_serializing)]` returns `true`               |
| `is_skipped_skip_deserializing` | `#[serde(skip_deserializing)]` returns `false`            |
| `is_skipped_none`               | No skip attr returns `false`                              |
| `has_default_present`           | `#[serde(default)]` returns `true`                        |

### Unit tests (model — RenameRule)

| Test                          | Description                                           |
|-------------------------------|-------------------------------------------------------|
| `rename_rule_camel_case`      | `first_name` → `firstName`, `MyVariant` → `myVariant` |
| `rename_rule_snake_case`      | `MyVariant` → `my_variant`                            |
| `rename_rule_pascal_case`     | `first_name` → `FirstName`                            |
| `rename_rule_screaming_snake` | `first_name` → `FIRST_NAME`                           |
| `rename_rule_kebab`           | `first_name` → `first-name`                           |
| `rename_rule_screaming_kebab` | `first_name` → `FIRST-NAME`                           |
| `rename_rule_lowercase`       | `first_name` → `firstname`                            |
| `rename_rule_uppercase`       | `first_name` → `FIRSTNAME`                            |

### Integration tests (extract)

| Test                   | Description                                  |
|------------------------|----------------------------------------------|
| `struct_rename_all`    | Struct with `rename_all` parsed into model   |
| `struct_field_rename`  | Field-level `rename` parsed                  |
| `struct_field_skip`    | Skipped field has `skip: true`               |
| `struct_field_default` | Field with `default` has `has_default: true` |
| `enum_rename_all`      | Enum `rename_all` parsed                     |
| `enum_variant_rename`  | Variant-level `rename` parsed                |

### Integration tests (codegen)

| Test                                   | Description                                   |
|----------------------------------------|-----------------------------------------------|
| `ts_struct_rename_all_camel`           | Fields renamed in generated TS                |
| `ts_field_rename_overrides_rename_all` | `rename` takes priority                       |
| `ts_skip_field_omitted`                | Skipped fields not in output                  |
| `ts_default_option_optional`           | `default` + `Option<T>` → `field?: T \| null` |
| `ts_enum_rename_all_snake`             | Variant names transformed in union            |
| `ts_enum_variant_rename`               | Single variant override                       |
| `ts_rename_all_priority_over_config`   | Serde attr beats `codegen.naming.fields`      |

## 11. Backward Compatibility

- **No breaking changes.** Structs and enums without serde attributes behave identically.
- The existing `codegen.naming.fields` config continues to work as a fallback.
- Serde attributes simply add more precision — the generated TypeScript becomes more accurate, not different in behavior for code that doesn't use serde attributes.

## 12. Future Extensions

These are explicitly out of scope but the model changes in this RFC are designed to accommodate them:

- **Enum tagging strategies** (`tag`, `content`, `untagged`) — requires new fields on `EnumDef`, no field-level changes.
- **`#[serde(flatten)]`** — requires a new `FieldDef` flag and intersection type (`&`) codegen.
- **`rename_all_fields`** on enums — requires an additional `RenameRule` on `EnumDef`.
- **`skip_serializing_if`** — runtime-only behavior, no type-level impact, but could optionally mark the field as `?` in TypeScript.
