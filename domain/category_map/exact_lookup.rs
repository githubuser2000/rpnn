use crate::domain::categories::{KategorieMap, Oberkategorie};

pub fn finde_spaltennummern_exakt_in_maps(
    hauptkategorien: &[Oberkategorie],
    ober: &str,
    unter: &str,
) -> Vec<u32> {
    let mut gefundene = Vec::new();

    for haupt in hauptkategorien {
        if haupt.key.matches_str(ober) {
            for unterkategorie in &haupt.unterkategorien {
                if unterkategorie.name.matches_str(unter) {
                    gefundene.extend_from_slice(&unterkategorie.spaltennummern);
                }
            }
        }
    }

    gefundene.sort();
    gefundene.dedup();
    gefundene
}

pub fn finde_spaltennummern_fuer_kategorien(
    map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Vec<u32> {
    let exakt = finde_spaltennummern_exakt_in_maps(&map.hauptkategorien, ober, unter);
    if !exakt.is_empty() {
        return exakt;
    }
    finde_spaltennummern_exakt_in_maps(&map.hauptkategorien, ober, unter)
}
