use crate::domain::categories::{normalize_key, KategorieMap, Oberkategorie};

pub fn finde_spaltennummern_exakt_in_maps(
    hauptkategorien: &[Oberkategorie],
    ober: &str,
    unter: &str,
) -> Vec<u32> {
    let mut gefundene = Vec::new();
    let ober_gesucht = normalize_key(ober);
    let unter_gesucht = normalize_key(unter);

    for haupt in hauptkategorien {
        if normalize_key(&haupt.key.to_string()) == ober_gesucht {
            for unterkategorie in &haupt.unterkategorien {
                if normalize_key(unterkategorie.name.as_str()) == unter_gesucht {
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
