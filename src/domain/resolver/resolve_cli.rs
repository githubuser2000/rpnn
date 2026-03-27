use crate::domain::model::spalten_anfrage::CanonicalColumnSpec;
use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use crate::domain::request_bridge::bridge_cli_selection;
use crate::domain::resolver::request_resolver::resolve_request;

pub fn resolve_cli_ober_unter(ober: &str, unter: &str) -> Option<CanonicalColumnSpec> {
    if let Some(req) = bridge_cli_selection(ober, unter) {
        if let Some(spec) = resolve_request(req) {
            return Some(spec);
        }
    }

    let req = parse_spalten_anfrage(ober, unter).ok()?;
    resolve_request(req)
}
