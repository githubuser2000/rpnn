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

pub fn bridge_raw_selection(selection: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    bridge_legacy_request(selection)
}

pub fn bridge_raw_pairs<I, O, U>(pairs: I) -> Vec<canonical::SpaltenAnfrage>
where
    I: IntoIterator<Item = (O, U)>,
    O: AsRef<str>,
    U: AsRef<str>,
{
    pairs
        .into_iter()
        .filter_map(|(ober, unter)| bridge_cli_selection(ober.as_ref(), unter.as_ref()))
        .collect()
}
