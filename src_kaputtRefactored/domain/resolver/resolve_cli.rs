use crate::domain::model::spalten_anfrage::CanonicalColumnSpec;
use crate::domain::request_bridge::bridge_cli_selection;
use crate::domain::resolver::request_resolver::resolve_request;

pub fn resolve_cli_ober_unter(ober: &str, unter: &str) -> Option<CanonicalColumnSpec> {
    bridge_cli_selection(ober, unter).and_then(resolve_request)
}
