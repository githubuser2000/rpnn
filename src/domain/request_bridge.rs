use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use crate::domain::spalten_anfrage as legacy;

pub fn bridge_legacy_request(request: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    let (ober, unter) = request.ober_unter_cli_pair();
    parse_spalten_anfrage(&ober, &unter).ok()
}

pub fn parse_and_bridge(ober: &str, unter: &str) -> Option<canonical::SpaltenAnfrage> {
    let legacy = legacy::SpaltenAnfrage::parse(ober, unter).ok()?;
    bridge_legacy_request(&legacy)
}
