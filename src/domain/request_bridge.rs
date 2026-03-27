use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use crate::domain::spalten_anfrage as legacy;

pub fn bridge_cli_selection(ober: &str, unter: &str) -> Option<canonical::SpaltenAnfrage> {
    parse_spalten_anfrage(ober, unter).ok()
}

pub fn bridge_legacy_request(request: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    let (ober, unter) = request.ober_unter_cli_pair();
    bridge_cli_selection(&ober, &unter)
}
