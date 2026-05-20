#![allow(dead_code)]

//! Compile-time embedded CSV assets for the Reta runtime.
//!
//! The generated table is produced by `build.rs` from the checked-in `csv/*.csv`
//! files.  At runtime the loader can use these assets directly from the shared
//! library, so `rreta` and `retaPrompt` no longer depend on the current working
//! directory containing a `csv/` subdirectory.

mod generated {
    include!(concat!(env!("OUT_DIR"), "/reta_embedded_csv_assets.rs"));
}

pub use self::generated::{EmbeddedCsvAlias, EmbeddedCsvAsset};

const LANGUAGE_ALIAS_PREFIXES: &[&str] = &["en-", "cn-", "kr-", "vn-"];

pub fn embedded_csv_assets() -> &'static [EmbeddedCsvAsset] {
    generated::EMBEDDED_CSV_ASSETS
}

pub fn embedded_csv_aliases() -> &'static [EmbeddedCsvAlias] {
    generated::EMBEDDED_CSV_ALIASES
}

pub fn stripped_language_alias_name(name: &str) -> Option<&str> {
    for prefix in LANGUAGE_ALIAS_PREFIXES {
        if let Some(stripped) = name.strip_prefix(prefix) {
            return Some(stripped);
        }
    }
    None
}

fn exact_embedded_csv_asset_name(name: &str) -> Option<&'static str> {
    embedded_csv_assets()
        .iter()
        .find(|asset| asset.name == name)
        .map(|asset| asset.name)
}

fn embedded_csv_alias_target(name: &str) -> Option<&'static str> {
    embedded_csv_aliases()
        .iter()
        .find(|alias| alias.name == name)
        .map(|alias| alias.target)
}

fn canonical_embedded_csv_name_with_depth(name: &str, depth: usize) -> Option<&'static str> {
    if depth > 4 {
        return None;
    }
    if let Some(asset_name) = exact_embedded_csv_asset_name(name) {
        return Some(asset_name);
    }
    if let Some(target) = embedded_csv_alias_target(name) {
        return canonical_embedded_csv_name_with_depth(target, depth + 1);
    }
    if let Some(stripped) = stripped_language_alias_name(name) {
        return canonical_embedded_csv_name_with_depth(stripped, depth + 1);
    }
    None
}

pub fn canonical_embedded_csv_name(name: &str) -> Option<&'static str> {
    canonical_embedded_csv_name_with_depth(name, 0)
}

pub fn embedded_csv_bytes(name: &str) -> Option<&'static [u8]> {
    let canonical_name = canonical_embedded_csv_name(name)?;
    embedded_csv_assets()
        .iter()
        .find(|asset| asset.name == canonical_name)
        .map(|asset| asset.bytes)
}

pub fn embedded_csv_text(name: &str) -> Option<&'static str> {
    std::str::from_utf8(embedded_csv_bytes(name)?).ok()
}

pub fn embedded_csv_asset_count() -> usize {
    embedded_csv_assets().len()
}

pub fn embedded_csv_alias_count() -> usize {
    embedded_csv_aliases().len()
}

pub fn has_embedded_csv(name: &str) -> bool {
    canonical_embedded_csv_name(name).is_some()
}
