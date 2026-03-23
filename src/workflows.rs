
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::env;
use crate::column_categories_complete::lade_kategorie_map;
use crate::csv_importer::import_csvs_to_sqlite;
use crate::table_printer::query_column_by_index;
use crate::tabellen_utils::show_usage;
use crate::argument_verarbeiter::SpaltenVerarbeiter;
use crate::kategorie_verarbeiter::verarbeite_kategorien;
use crate::generated_columns_words_registry::ParametersMain;
use crate::pypy_compat::apply_pypy_compat;

pub fn main_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_usage();
        return Ok(());
    }

    let kategorie_map = lade_kategorie_map();
    let verarbeiter = SpaltenVerarbeiter::new(&args, &kategorie_map);
    let (mut bereich, spalten_namen) = verarbeiter.verarbeite_zu_tupel()?;

    let mut generated_befehle: BTreeSet<String> =
        verarbeite_kategorien(&kategorie_map, &mut bereich, &spalten_namen)?;
    generated_befehle.extend(bereich.exact_generated_befehle.iter().cloned());

    let wants_gebr_prim_generator = generated_befehle.iter().any(|g| g.contains("gebr") && g.contains("prim"));
    if wants_gebr_prim_generator {
        let upper = if bereich.bis_zeile > 1 { bereich.bis_zeile.min(23) } else { 23 };
        for n in 2..=upper {
            bereich.pypy_compat.gebrochengalaxie.insert(n);
            bereich.pypy_compat.gebrochenuniversum.insert(n);
        }
        bereich.pypy_compat.hidden_fraction_inputs = true;
    }

    let parameters_main = ParametersMain {
        bedeutung0: spalten_namen.oberkategorie.clone(),
        procontra0: spalten_namen.oberkategorie.clone(),
        grundstrukturen0: spalten_namen.oberkategorie.clone(),
        unter0: spalten_namen.unterkategorie.clone(),
    };

    let proj_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pfad1 = proj_path.to_string_lossy().into_owned() + "/csv/religion.csv";
    let pfad2 = proj_path.to_string_lossy().into_owned() + "/csv/merged_filtered.csv";
    let dateien = [pfad1, pfad2];

    let conn = import_csvs_to_sqlite(&dateien)?;
    apply_pypy_compat(&conn, &mut bereich, &proj_path)?;
    query_column_by_index(&conn, bereich, &generated_befehle, &parameters_main)?;


    Ok(())
}
