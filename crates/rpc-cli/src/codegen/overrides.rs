use std::collections::HashMap;

use crate::model::{Manifest, RustType, VariantKind};

/// Builds a base-name lookup index from type override keys.
///
/// Each key is normalized to its last path segment (e.g. `"chrono::DateTime"` → `"DateTime"`).
/// This index is used as a fallback when an exact full-path match fails, which happens
/// when the Rust source uses an imported name rather than a fully-qualified path.
pub fn build_base_index(overrides: &HashMap<String, String>) -> HashMap<String, String> {
    overrides
        .iter()
        .map(|(k, v)| {
            let base = k.rsplit("::").next().unwrap_or(k);
            (base.to_string(), v.clone())
        })
        .collect()
}

/// Applies type overrides to every [`RustType`] node in the manifest.
///
/// Matching order for each type node:
/// 1. **Exact match** — the full `RustType.name` is looked up in `overrides`
///    (handles fully-qualified paths like `chrono::DateTime`).
/// 2. **Base-name fallback** — the last segment of `RustType.name` is looked up
///    in `base_index` (handles imported names like `DateTime`).
///
/// When a match is found the node's name is replaced with the TypeScript type
/// string and its generic parameters are cleared (e.g. `DateTime<Utc>` → `string`).
pub fn apply_type_overrides(
    manifest: &mut Manifest,
    overrides: &HashMap<String, String>,
    base_index: &HashMap<String, String>,
) {
    if overrides.is_empty() {
        return;
    }

    for proc in &mut manifest.procedures {
        if let Some(ty) = &mut proc.input {
            override_type(ty, overrides, base_index);
        }
        if let Some(ty) = &mut proc.output {
            override_type(ty, overrides, base_index);
        }
    }

    for s in &mut manifest.structs {
        for field in &mut s.fields {
            override_type(&mut field.ty, overrides, base_index);
        }
        for ty in &mut s.tuple_fields {
            override_type(ty, overrides, base_index);
        }
    }

    for e in &mut manifest.enums {
        for variant in &mut e.variants {
            match &mut variant.kind {
                VariantKind::Unit => {}
                VariantKind::Tuple(types) => {
                    for ty in types {
                        override_type(ty, overrides, base_index);
                    }
                }
                VariantKind::Struct(fields) => {
                    for field in fields {
                        override_type(&mut field.ty, overrides, base_index);
                    }
                }
            }
        }
    }
}

/// Recursively overrides a single [`RustType`] node and its generic parameters.
fn override_type(
    ty: &mut RustType,
    overrides: &HashMap<String, String>,
    base_index: &HashMap<String, String>,
) {
    // Exact match on full name first, then base-name fallback
    if let Some(ts_type) = overrides
        .get(&ty.name)
        .or_else(|| base_index.get(ty.base_name()))
    {
        ty.name = ts_type.clone();
        ty.generics.clear();
        return;
    }

    for g in &mut ty.generics {
        override_type(g, overrides, base_index);
    }
}
