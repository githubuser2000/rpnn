use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use crate::domain::spalten_anfrage::SpaltenAnfrage as LegacySpaltenAnfrage;

pub fn to_canonical_request(request: &LegacySpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    let (ober, unter) = request.ober_unter_cli_pair();
    parse_spalten_anfrage(&ober, &unter).ok()
}
