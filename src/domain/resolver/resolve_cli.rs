use crate::domain::model::spalten_anfrage::CanonicalColumnSpec;
use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use crate::domain::request_bridge::bridge_request;
use crate::domain::resolver::request_resolver::resolve_request;
use crate::domain::spalten_anfrage::SpaltenAnfrage as LegacyRequest;

pub fn resolve_cli_ober_unter(ober: &str, unter: &str) -> Option<CanonicalColumnSpec> {
    if let Ok(legacy_req) = LegacyRequest::parse(ober, unter) {
        if let Some(canonical_req) = bridge_request(&legacy_req) {
            if let Some(spec) = resolve_request(canonical_req) {
                return Some(spec);
            }
        }
    }

    let req = parse_spalten_anfrage(ober, unter).ok()?;
    resolve_request(req)
}
