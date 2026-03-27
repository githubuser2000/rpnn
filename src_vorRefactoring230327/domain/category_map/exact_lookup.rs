use crate::domain::categories::{
    KategorieMap, KategorieProvider, Oberkategorie, OberkategorieEntry, UnterkategorieEntry,
};
use super::normalize::normalize_key;

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
    let ober_gesucht = normalize_key(ober);
    let unter_gesucht = normalize_key(unter);

    for haupt in hauptkategorien {
        if normalize_key(haupt.ober_name()) == ober_gesucht {
            for unterkategorie in haupt.unterkategorien() {
                if normalize_key(unterkategorie.unter_name()) == unter_gesucht {
                    gefundene.extend_from_slice(unterkategorie.column_numbers());
                }
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
