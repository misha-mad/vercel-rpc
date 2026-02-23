use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The kind of RPC procedure, determined by the macro attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcedureKind {
    Query,
    Mutation,
}

/// Serde `rename_all` naming convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenameRule {
    #[serde(rename = "camelCase")]
    CamelCase,
    #[serde(rename = "snake_case")]
    SnakeCase,
    #[serde(rename = "PascalCase")]
    PascalCase,
    #[serde(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnakeCase,
    #[serde(rename = "kebab-case")]
    KebabCase,
    #[serde(rename = "SCREAMING-KEBAB-CASE")]
    ScreamingKebabCase,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "UPPERCASE")]
    Uppercase,
}

impl RenameRule {
    /// Transforms a name according to this rename rule.
    pub fn apply(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }
        let words = split_words(input);
        match self {
            RenameRule::CamelCase => {
                let mut result = String::new();
                for (i, word) in words.iter().enumerate() {
                    if i == 0 {
                        result.push_str(&word.to_lowercase());
                    } else {
                        capitalize_into(word, &mut result);
                    }
                }
                result
            }
            RenameRule::PascalCase => {
                let mut result = String::new();
                for word in &words {
                    capitalize_into(word, &mut result);
                }
                result
            }
            RenameRule::SnakeCase => join_mapped(&words, "_", str::to_lowercase),
            RenameRule::ScreamingSnakeCase => join_mapped(&words, "_", str::to_uppercase),
            RenameRule::KebabCase => join_mapped(&words, "-", str::to_lowercase),
            RenameRule::ScreamingKebabCase => join_mapped(&words, "-", str::to_uppercase),
            RenameRule::Lowercase => join_mapped(&words, "", str::to_lowercase),
            RenameRule::Uppercase => join_mapped(&words, "", str::to_uppercase),
        }
    }
}

/// Joins words with a separator, applying a transform to each word without intermediate allocation.
fn join_mapped(words: &[String], sep: &str, f: fn(&str) -> String) -> String {
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(&f(w));
    }
    out
}

/// Pushes a word capitalized (first char uppercase, rest lowercase) into `out`.
fn capitalize_into(word: &str, out: &mut String) {
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        out.push_str(&chars.as_str().to_lowercase());
    }
}

/// Splits a name into words, handling snake_case, PascalCase, and acronyms.
///
/// Examples:
/// - `"first_name"` → `["first", "name"]`
/// - `"MyVariant"` → `["My", "Variant"]`
/// - `"HTTPSPort"` → `["HTTPS", "Port"]`
/// - `"IOError"` → `["IO", "Error"]`
fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    for segment in input.split('_') {
        if segment.is_empty() {
            continue;
        }
        let chars: Vec<char> = segment.chars().collect();
        let mut current = String::new();
        for i in 0..chars.len() {
            let ch = chars[i];
            if ch.is_uppercase() && !current.is_empty() {
                let prev_lower = current.chars().last().is_some_and(|c| c.is_lowercase());
                let next_lower = chars.get(i + 1).is_some_and(|c| c.is_lowercase());
                // Split when: previous char was lowercase (camelCase boundary),
                // or next char is lowercase (end of acronym, e.g. "S" in "HTTPSPort")
                if prev_lower || next_lower {
                    words.push(current);
                    current = String::new();
                }
            }
            current.push(ch);
        }
        if !current.is_empty() {
            words.push(current);
        }
    }
    words
}

/// Error returned when parsing an unknown `rename_all` rule string.
#[derive(Debug, Error)]
#[error("unknown rename_all rule: `{0}`")]
pub struct UnknownRenameRule(String);

impl FromStr for RenameRule {
    type Err = UnknownRenameRule;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "camelCase" => Ok(RenameRule::CamelCase),
            "snake_case" => Ok(RenameRule::SnakeCase),
            "PascalCase" => Ok(RenameRule::PascalCase),
            "SCREAMING_SNAKE_CASE" => Ok(RenameRule::ScreamingSnakeCase),
            "kebab-case" => Ok(RenameRule::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(RenameRule::ScreamingKebabCase),
            "lowercase" => Ok(RenameRule::Lowercase),
            "UPPERCASE" => Ok(RenameRule::Uppercase),
            _ => Err(UnknownRenameRule(s.to_owned())),
        }
    }
}

/// A single Rust type reference extracted from source code.
///
/// Preserves the full path as written (e.g. `Vec<String>`, `MyStruct`).
/// Generic parameters are stored recursively for accurate TS mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustType {
    /// The base type name (e.g. "Vec", "String", "MyStruct")
    pub name: String,
    /// Generic type parameters, if any (e.g. `Vec<String>` → [RustType("String")])
    pub generics: Vec<RustType>,
}

impl RustType {
    /// Creates a simple type with no generic parameters (e.g. `String`, `i32`).
    pub fn simple(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            generics: vec![],
        }
    }

    /// Creates a generic type with the given type parameters (e.g. `Vec<String>`).
    pub fn with_generics(name: impl Into<String>, generics: Vec<RustType>) -> Self {
        Self {
            name: name.into(),
            generics,
        }
    }

    /// Returns the base name (last path segment) of this type.
    ///
    /// For simple names like `"String"` this returns `"String"`.
    /// For qualified paths like `"chrono::DateTime"` this returns `"DateTime"`.
    pub fn base_name(&self) -> &str {
        self.name.rsplit("::").next().unwrap_or(&self.name)
    }
}

impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.generics.is_empty() {
            write!(f, "<")?;
            for (i, g) in self.generics.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{g}")?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// A single field in a struct or struct variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub ty: RustType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub skip: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_default: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub flatten: bool,
}

/// Metadata for a single RPC procedure extracted from a source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    /// Procedure name derived from the function name
    pub name: String,
    /// Query or Mutation
    pub kind: ProcedureKind,
    /// Input parameter type; `None` means no input (unit type)
    pub input: Option<RustType>,
    /// Return type; `None` means unit return
    pub output: Option<RustType>,
    /// Source file this procedure was extracted from
    pub source_file: PathBuf,
    /// Doc comment extracted from `///` lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
}

/// All user-defined struct types found in the scanned source files.
/// Needed for generating corresponding TypeScript interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    /// Struct name
    pub name: String,
    /// Generic type parameter names (e.g. `["T"]`, `["A", "B"]`)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generics: Vec<String>,
    /// Named fields with their types
    pub fields: Vec<FieldDef>,
    /// Unnamed fields for tuple structs (e.g. `struct UserId(String)`)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tuple_fields: Vec<RustType>,
    /// Source file this struct was defined in
    pub source_file: PathBuf,
    /// Doc comment extracted from `///` lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    /// Container-level `#[serde(rename_all = "...")]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_all: Option<RenameRule>,
}

/// A single variant of a Rust enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    /// Variant name (e.g. `Active`, `Error`)
    pub name: String,
    /// Variant kind determines TypeScript representation
    pub kind: VariantKind,
    /// Field-level `#[serde(rename = "...")]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<String>,
}

/// The shape of an enum variant's data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantKind {
    /// Unit variant: `Active` → string literal `"Active"`
    Unit,
    /// Tuple variant with a single unnamed field: `Error(String)` → `{ Error: string }`
    Tuple(Vec<RustType>),
    /// Struct variant with named fields: `User { name: String }` → `{ User: { name: string } }`
    Struct(Vec<FieldDef>),
}

/// Serde enum tagging strategy.
///
/// Corresponds to the four representations serde supports:
/// - `External` (default): `{ "Variant": data }`
/// - `Internal { tag }`: `{ "tag": "Variant", ...data }`
/// - `Adjacent { tag, content }`: `{ "tag": "Variant", "content": data }`
/// - `Untagged`: `data` (no wrapping)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnumTagging {
    #[default]
    External,
    Internal {
        tag: String,
    },
    Adjacent {
        tag: String,
        content: String,
    },
    Untagged,
}

/// All user-defined enum types found in the scanned source files.
/// Needed for generating corresponding TypeScript union types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    /// Enum name
    pub name: String,
    /// Generic type parameter names (e.g. `["T"]`, `["A", "B"]`)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generics: Vec<String>,
    /// Variants of the enum
    pub variants: Vec<EnumVariant>,
    /// Source file this enum was defined in
    pub source_file: PathBuf,
    /// Doc comment extracted from `///` lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    /// Container-level `#[serde(rename_all = "...")]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_all: Option<RenameRule>,
    /// Serde enum tagging strategy
    #[serde(default)]
    pub tagging: EnumTagging,
}

/// Complete manifest of all discovered RPC metadata from a scan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Manifest {
    pub procedures: Vec<Procedure>,
    pub structs: Vec<StructDef>,
    pub enums: Vec<EnumDef>,
}
