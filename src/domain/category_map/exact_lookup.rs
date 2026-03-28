use crate::domain::categories::{
    KategorieMap, KategorieProvider, Oberkategorie, OberkategorieEntry, UnterkategorieEntry,
};
use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use super::normalize::names_equal;

pub fn finde_spaltennummern_exakt_in_maps<O, U>(
    hauptkategorien: &[O],
    ober: &str,
    unter: &str,
) -> Vec<u32>
where
    O: OberkategorieEntry<Unter = U>,
    U: UnterkategorieEntry,
{
    let mut gefundene = Vec::new();
    let requested = parse_spalten_anfrage(ober, unter).ok();

    for haupt in hauptkategorien {
        for unterkategorie in haupt.unterkategorien() {
            let is_match = if let Some(request) = &requested {
                parse_spalten_anfrage(haupt.ober_name(), unterkategorie.unter_name())
                    .ok()
                    .as_ref()
                    == Some(request)
            } else {
                names_equal(haupt.ober_name(), ober) && names_equal(unterkategorie.unter_name(), unter)
            };

            if is_match {
                gefundene.extend_from_slice(unterkategorie.column_numbers());
            }
        }
    }

    gefundene.sort();
    gefundene.dedup();
    gefundene
}

pub fn finde_spaltennummern_exakt_in_provider<T>(
    provider: &T,
    ober: &str,
    unter: &str,
) -> Vec<u32>
where
    T: KategorieProvider<Ober = Oberkategorie>,
{
    finde_spaltennummern_exakt_in_maps(provider.hauptkategorien(), ober, unter)
}

pub fn finde_spaltennummern_fuer_kategorien(
    map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Vec<u32> {
    let exakt = finde_spaltennummern_exakt_in_provider(map, ober, unter);
    if !exakt.is_empty() {
        return exakt;
    }
    finde_spaltennummern_exakt_in_provider(map, ober, unter)
}
