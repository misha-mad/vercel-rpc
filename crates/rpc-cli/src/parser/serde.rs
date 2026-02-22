use crate::model::{EnumTagging, RenameRule};

/// Walks `#[serde(...)]` attributes and calls `visitor` for each nested meta item.
/// Returns the last value produced by the visitor, or `None` if no match.
fn find_serde_meta<T>(
    attrs: &[syn::Attribute],
    mut visitor: impl FnMut(&syn::meta::ParseNestedMeta) -> Option<T>,
) -> Option<T> {
    let mut result = None;
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if let Some(value) = visitor(&meta) {
                result = Some(value);
            }
            Ok(())
        });
    }
    result
}

/// Parses `#[serde(rename_all = "...")]` from attributes.
pub fn parse_rename_all(attrs: &[syn::Attribute]) -> Option<RenameRule> {
    find_serde_meta(attrs, |meta| {
        if !meta.path.is_ident("rename_all") {
            return None;
        }
        let value = meta.value().ok()?.parse::<syn::LitStr>().ok()?;
        match value.value().parse::<RenameRule>() {
            Ok(rule) => Some(rule),
            Err(e) => {
                eprintln!(
                    "warning: unknown rename_all value \"{}\" — {e}; attribute ignored",
                    value.value(),
                );
                None
            }
        }
    })
}

/// Parses `#[serde(rename = "...")]` from attributes.
pub fn parse_rename(attrs: &[syn::Attribute]) -> Option<String> {
    find_serde_meta(attrs, |meta| {
        if !meta.path.is_ident("rename") {
            return None;
        }
        let value = meta.value().ok()?.parse::<syn::LitStr>().ok()?;
        Some(value.value())
    })
}

/// Checks for `#[serde(skip)]` or `#[serde(skip_serializing)]` on a field.
pub fn is_skipped(attrs: &[syn::Attribute]) -> bool {
    find_serde_meta(attrs, |meta| {
        if meta.path.is_ident("skip") || meta.path.is_ident("skip_serializing") {
            Some(true)
        } else {
            None
        }
    })
    .unwrap_or(false)
}

/// Parses the serde enum tagging strategy from attributes.
///
/// Recognizes `#[serde(tag = "...", content = "...")]` and `#[serde(untagged)]`.
/// Priority: `untagged` → `Untagged`; `tag` + `content` → `Adjacent`;
/// `tag` only → `Internal`; else → `External`.
pub fn parse_enum_tagging(attrs: &[syn::Attribute]) -> EnumTagging {
    let mut tag = None;
    let mut content = None;
    let mut untagged = false;

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("untagged") {
                untagged = true;
            } else if meta.path.is_ident("tag")
                && let Ok(value) = meta.value()
                && let Ok(lit) = value.parse::<syn::LitStr>()
            {
                tag = Some(lit.value());
            } else if meta.path.is_ident("content")
                && let Ok(value) = meta.value()
                && let Ok(lit) = value.parse::<syn::LitStr>()
            {
                content = Some(lit.value());
            }
            Ok(())
        });
    }

    if untagged {
        EnumTagging::Untagged
    } else if let Some(tag) = tag {
        if let Some(content) = content {
            EnumTagging::Adjacent { tag, content }
        } else {
            EnumTagging::Internal { tag }
        }
    } else {
        EnumTagging::External
    }
}

/// Checks for `#[serde(default)]` on a field.
pub fn has_default(attrs: &[syn::Attribute]) -> bool {
    find_serde_meta(attrs, |meta| {
        if meta.path.is_ident("default") {
            Some(true)
        } else {
            None
        }
    })
    .unwrap_or(false)
}
