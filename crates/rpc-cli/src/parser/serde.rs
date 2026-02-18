use crate::model::RenameRule;

/// Parses `#[serde(rename_all = "...")]` from attributes.
pub fn parse_rename_all(attrs: &[syn::Attribute]) -> Option<RenameRule> {
    let mut result = None;
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                let value = meta.value()?.parse::<syn::LitStr>()?;
                if let Ok(rule) = value.value().parse::<RenameRule>() {
                    result = Some(rule);
                }
            }
            Ok(())
        });
    }
    result
}

/// Parses `#[serde(rename = "...")]` from attributes.
pub fn parse_rename(attrs: &[syn::Attribute]) -> Option<String> {
    let mut result = None;
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value = meta.value()?.parse::<syn::LitStr>()?;
                result = Some(value.value());
            }
            Ok(())
        });
    }
    result
}

/// Checks for `#[serde(skip)]` or `#[serde(skip_serializing)]` on a field.
pub fn is_skipped(attrs: &[syn::Attribute]) -> bool {
    let mut skipped = false;
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") || meta.path.is_ident("skip_serializing") {
                skipped = true;
            }
            Ok(())
        });
    }
    skipped
}

/// Checks for `#[serde(default)]` on a field.
pub fn has_default(attrs: &[syn::Attribute]) -> bool {
    let mut found = false;
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("default") {
                found = true;
            }
            Ok(())
        });
    }
    found
}
