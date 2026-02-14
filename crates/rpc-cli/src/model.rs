use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The kind of RPC procedure, determined by the macro attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcedureKind {
    Query,
    Mutation,
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
    pub fn simple(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            generics: vec![],
        }
    }

    pub fn with_generics(name: impl Into<String>, generics: Vec<RustType>) -> Self {
        Self {
            name: name.into(),
            generics,
        }
    }

    /// Returns a human-readable representation matching Rust syntax.
    pub fn display(&self) -> String {
        if self.generics.is_empty() {
            self.name.clone()
        } else {
            let inner: Vec<String> = self.generics.iter().map(|g| g.display()).collect();
            format!("{}<{}>", self.name, inner.join(", "))
        }
    }
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
}

/// All user-defined struct types found in the scanned source files.
/// Needed for generating corresponding TypeScript interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    /// Struct name
    pub name: String,
    /// Named fields with their types
    pub fields: Vec<(String, RustType)>,
    /// Source file this struct was defined in
    pub source_file: PathBuf,
}

/// A single variant of a Rust enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    /// Variant name (e.g. `Active`, `Error`)
    pub name: String,
    /// Variant kind determines TypeScript representation
    pub kind: VariantKind,
}

/// The shape of an enum variant's data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantKind {
    /// Unit variant: `Active` → string literal `"Active"`
    Unit,
    /// Tuple variant with a single unnamed field: `Error(String)` → `{ Error: string }`
    Tuple(Vec<RustType>),
    /// Struct variant with named fields: `User { name: String }` → `{ User: { name: string } }`
    Struct(Vec<(String, RustType)>),
}

/// All user-defined enum types found in the scanned source files.
/// Needed for generating corresponding TypeScript union types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    /// Enum name
    pub name: String,
    /// Variants of the enum
    pub variants: Vec<EnumVariant>,
    /// Source file this enum was defined in
    pub source_file: PathBuf,
}

/// Complete manifest of all discovered RPC metadata from a scan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Manifest {
    pub procedures: Vec<Procedure>,
    pub structs: Vec<StructDef>,
    pub enums: Vec<EnumDef>,
}
