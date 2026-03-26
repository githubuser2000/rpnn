use crate::domain::categories::{KategorieMap, KategorieProvider, OberkategorieEntry, UnterkategorieEntry};
use super::normalize::normalize_key;

pub fn finde_spaltennummern_exakt_in_provider<P>(
    provider: &P,
    ober: &str,
    unter: &str,
) -> Vec<u32>
where
    P: KategorieProvider,
{
    let mut gefundene = Vec::new();
    let ober_gesucht = normalize_key(ober);
    let unter_gesucht = normalize_key(unter);

    for haupt in provider.hauptkategorien() {
        if normalize_key(haupt.ober_name().as_str()) == ober_gesucht {
            for unterkategorie in haupt.unterkategorien() {
                if normalize_key(unterkategorie.unter_name().as_str()) == unter_gesucht {
                    gefundene.extend_from_slice(unterkategorie.spaltennummern());
                }
            }
        }
    }

    gefundene.sort();
    gefundene.dedup();
    gefundene
}

pub fn finde_spaltennummern_exakt_in_maps(
    hauptkategorien: &[crate::domain::categories::Oberkategorie],
    ober: &str,
    unter: &str,
) -> Vec<u32> {
    struct SliceProvider<'a> {
        hauptkategorien: &'a [crate::domain::categories::Oberkategorie],
    }
    impl<'a> KategorieProvider for SliceProvider<'a> {
        type Ober = crate::domain::categories::Oberkategorie;
        fn hauptkategorien(&self) -> &[Self::Ober] {
            self.hauptkategorien
        }
    }

    finde_spaltennummern_exakt_in_provider(&SliceProvider { hauptkategorien }, ober, unter)
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
