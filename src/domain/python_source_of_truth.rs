use crate::domain::decl_model::HtmlDeclMeta;
use crate::domain::typed_exact_decl::{
    all_typed_exact_decls, is_typed_exact_decl_column, typed_exact_decl_for_column,
};
// Auto-generated from reta.todel Python sources and runtime metadata

#[derive(Debug, Clone, Copy)]
pub struct PyDecl {
    pub main_aliases: &'static [&'static str],
    pub sub_aliases: &'static [&'static str],
    pub columns: &'static [u32],
}

fn normalize_key(s: &str) -> String {
    s.to_lowercase().replace("_", "").replace("-", "").replace(" ", "")
}

pub static DECL_0: PyDecl = PyDecl {
    main_aliases: &["Wichtigstes_zum_verstehen", "wichtigsteverstehen"],
    sub_aliases: &["Wichtigste", "wichtigste"],
    columns: &[4u32, 5u32, 8u32, 10u32],
};

pub static DECL_1: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Mensch-zu-Tier", "menschtier", "tiermensch"],
    columns: &[314u32],
};

pub static DECL_2: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Superkräfte", "Superkraefte"],
    columns: &[444u32, 494u32, 496u32, 503u32],
};

pub static DECL_3: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Superkräfte", "Superkraefte"],
    columns: &[444u32, 494u32, 496u32],
};

pub static DECL_4: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Evolution_vs_Design_intelligent"],
    columns: &[519u32],
};

pub static DECL_5: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Evolution_vs_Design_intelligent"],
    columns: &[519u32],
};

pub static DECL_6: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Superkräfte", "Superkraefte"],
    columns: &[444u32, 494u32, 496u32],
};

pub static DECL_7: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Formationen"],
    columns: &[461u32],
};

pub static DECL_8: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Ansichten_Standpunkte_(18_17)", "ansichten"],
    columns: &[240u32, 346u32],
};

pub static DECL_9: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["(politische)_Richtungen_(7)", "richtungen", "politische"],
    columns: &[235u32],
};

pub static DECL_10: PyDecl = PyDecl {
    main_aliases: &["Planet_(10_und_oder_12)", "planet"],
    sub_aliases: &["Wirklichkeiten_(10)", "wirklichkeit", "wirklichkeiten"],
    columns: &[233u32, 265u32, 268u32, 322u32, 420u32],
};

pub static DECL_11: PyDecl = PyDecl {
    main_aliases: &["Planet_(10_und_oder_12)", "planet"],
    sub_aliases: &["Meta-Systeme_(12)", "metasysteme", "metasystem", "meta-systeme", "meta-system"],
    columns: &[79u32, 80u32, 232u32, 288u32, 334u32, 410u32, 411u32, 483u32, 497u32, 498u32, 499u32],
};

pub static DECL_12: PyDecl = PyDecl {
    main_aliases: &["Planet_(10_und_oder_12)", "planet"],
    sub_aliases: &["Intelligenz", "intelligenz"],
    columns: &[214u32],
};

pub static DECL_13: PyDecl = PyDecl {
    main_aliases: &["Planet_(10_und_oder_12)", "planet"],
    sub_aliases: &["Gleichheit_Freiheit_Ordnung", "gleichheit", "freiheit", "gleichheit"],
    columns: &[79u32, 80u32, 132u32, 324u32, 328u32, 331u32, 335u32, 497u32, 498u32, 499u32],
};

pub static DECL_14: PyDecl = PyDecl {
    main_aliases: &["Planet_(10_und_oder_12)", "planet"],
    sub_aliases: &["Komplexität", "komplexität", "komplexitaet"],
    columns: &[213u32],
};

pub static DECL_15: PyDecl = PyDecl {
    main_aliases: &["Planet_(10_und_oder_12)", "planet"],
    sub_aliases: &["Mechanismen", "mechanismen", "mechanismus"],
    columns: &[107u32],
};

pub static DECL_16: PyDecl = PyDecl {
    main_aliases: &["Wichtigstes_zum_verstehen", "wichtigsteverstehen"],
    sub_aliases: &["Zweitwichtigste", "zweitwichtigste"],
    columns: &[19u32, 65u32, 183u32],
};

pub static DECL_17: PyDecl = PyDecl {
    main_aliases: &["Wichtigstes_zum_verstehen", "wichtigsteverstehen"],
    sub_aliases: &["Drittwichtigste", "drittwichtigste"],
    columns: &[64u32],
};

pub static DECL_19: PyDecl = PyDecl {
    main_aliases: &["Wichtigstes_zum_gedanklich_einordnen", "wichtigsteeinordnen"],
    sub_aliases: &["Wichtigste", "wichtigstes"],
    columns: &[0u32, 1u32, 2u32, 36u32, 37u32, 207u32],
};

pub static DECL_20: PyDecl = PyDecl {
    main_aliases: &["Wichtigstes_zum_gedanklich_einordnen", "wichtigsteeinordnen"],
    sub_aliases: &["Zweitwichtigste", "zweitwichtigste"],
    columns: &[30u32],
};

pub static DECL_21: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["Halbierung", "halbierung", "halbierungen"],
    columns: &[86u32],
};

pub static DECL_22: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Religions-Gründer-Typ", "religionsgründertyp", "prophet", "archon", "religionsgruendertyp"],
    columns: &[72u32, 503u32],
};

pub static DECL_23: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Satan_Teufel"],
    columns: &[495u32],
};

pub static DECL_24: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Satan_Teufel"],
    columns: &[495u32],
};

pub static DECL_25: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Hinduismus", "hinduismus"],
    columns: &[217u32],
};

pub static DECL_26: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Sternpolygon", "sternpolygon"],
    columns: &[0u32, 6u32, 36u32],
};

pub static DECL_27: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["der_Tierkreiszeichen", "dertierkreiszeichen", "babylon"],
    columns: &[0u32, 36u32, 207u32, 477u32, 478u32],
};

pub static DECL_28: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Sternpolygon_vs_gleichförmiges", "vergleich", "sternpolygonvsgleichfoermiges", "vergleichnvs1divn"],
    columns: &[87u32],
};

pub static DECL_29: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Messias", "messias", "heptagramm", "hund", "messiase", "messiasse"],
    columns: &[7u32, 503u32],
};

pub static DECL_30: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["gleichförmiges_Polygon", "gleichförmigespolygon", "gleichfoermigespolygon", "nichtsternpolygon", "polygon"],
    columns: &[16u32, 37u32],
};

pub static DECL_31: PyDecl = PyDecl {
    main_aliases: &["Religionen", "religionen", "religion"],
    sub_aliases: &["Vertreter_höherer_Konzepte", "vertreterhoehererkonzepte", "galaxien", "galaxie", "schwarzesonne", "schwarzesonnen", "universum", "universen", "kreis", "kreise", "kugel", "kugeln"],
    columns: &[23u32],
};

pub static DECL_32: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Lebewesen_Galaxie_am_Besten"],
    columns: &[470u32, 471u32, 473u32],
};

pub static DECL_33: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Offenbarung_des_Johannes", "offenbarung", "offenbarungdesjohannes", "johannes", "bibel", "offenbarungjohannes"],
    columns: &[90u32],
};

pub static DECL_34: PyDecl = PyDecl {
    main_aliases: &["Inkrementieren", "inkrementieren"],
    sub_aliases: &["Teilchen-Meta-Physik", "addition", "identitaet", "Identität"],
    columns: &[219u32, 223u32, 307u32, 308u32, 333u32, 387u32, 388u32, 406u32],
};

pub static DECL_35: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Hochzüchten", "hochzüchten", "hochzuechten"],
    columns: &[318u32, 319u32],
};

pub static DECL_36: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Teilchen_anderes_Universum"],
    columns: &[512u32],
};

pub static DECL_37: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Teilchen_anderes_Universum"],
    columns: &[512u32],
};

pub static DECL_38: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Teilchen_anderes_Universum"],
    columns: &[512u32],
};

pub static DECL_39: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Zusammenhang_Gehirn_Kosmos_Universum"],
    columns: &[489u32],
};

pub static DECL_40: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Zahlenarten"],
    columns: &[462u32],
};

pub static DECL_41: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Bestrafung"],
    columns: &[463u32],
};

pub static DECL_42: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Bestrafung"],
    columns: &[463u32],
};

pub static DECL_43: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["weniger_am_Menschen"],
    columns: &[464u32],
};

pub static DECL_44: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Erlösung", "Erloesung"],
    columns: &[465u32],
};

pub static DECL_45: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Erlösung", "Erloesung"],
    columns: &[465u32],
};

pub static DECL_46: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Gewalt"],
    columns: &[466u32],
};

pub static DECL_47: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Gewalt"],
    columns: &[466u32, 479u32],
};

pub static DECL_48: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Farben"],
    columns: &[444u32],
};

pub static DECL_49: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["künstliches_Leben_(15)", "künstlichesleben", "grosseki"],
    columns: &[409u32],
};

pub static DECL_50: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Software-Lizenzen_akademische_Grade", "softwarelizenz", "akademischeGrade"],
    columns: &[422u32],
};

pub static DECL_51: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Strategie_Taktik_(15m8)", "strategie", "taktik"],
    columns: &[385u32],
};

pub static DECL_52: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Universelles_Verhältnis_gleicher_Zahlen", "verhaeltnisgleicherzahl"],
    columns: &[383u32],
};

pub static DECL_53: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["universelles_Recht", "recht", "jura"],
    columns: &[34u32, 65u32, 382u32],
};

pub static DECL_54: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["sowas_wie_Kombinieren_Verknüpfen", "kombinierenetc"],
    columns: &[320u32],
};

pub static DECL_55: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Hochzüchten", "hochzüchten", "hochzuechten"],
    columns: &[318u32, 319u32],
};

pub static DECL_56: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Teilchen-Meta-Physik"],
    columns: &[219u32, 308u32],
};

pub static DECL_57: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["das_Universelle_(15)"],
    columns: &[219u32, 308u32],
};

pub static DECL_58: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["Wirklichkeiten_(10)", "wirklichkeit", "wirklichkeiten"],
    columns: &[420u32],
};

pub static DECL_59: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["das_Galaktische_(14)"],
    columns: &[406u32],
};

pub static DECL_60: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["das_Multiverselle_(16)"],
    columns: &[388u32, 418u32],
};

pub static DECL_61: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["die_Tugendsortierung_(13_mit_14)"],
    columns: &[411u32],
};

pub static DECL_62: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["die_Galaxie_Unterbereiche_(13)"],
    columns: &[223u32, 307u32, 412u32],
};

pub static DECL_63: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["das_Gute_die_Richtung_(7)"],
    columns: &[333u32],
};

pub static DECL_64: PyDecl = PyDecl {
    main_aliases: &["Teilchen-Meta-Physik", "teilchen"],
    sub_aliases: &["Raum_und_Dimensionen_(8)"],
    columns: &[387u32],
};

pub static DECL_65: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["keine_Nur-Paradigma-Religionen", "metaparadigmareligion"],
    columns: &[190u32, 191u32, 196u32],
};

pub static DECL_66: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Kugeln_Kreise", "kugelnkreise", "kugeln", "kreise"],
    columns: &[77u32, 145u32],
};

pub static DECL_67: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Raumzeit_Anordnung_mathematisch_universell"],
    columns: &[472u32],
};

pub static DECL_68: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Multiversalien_(16)", "multiversalien"],
    columns: &[389u32],
};

pub static DECL_69: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Meta-Physik-Teilchen_(1)", "teilchen"],
    columns: &[388u32],
};

pub static DECL_70: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Kugeln_Kreise", "kugelnkreise", "kugeln", "kreise"],
    columns: &[77u32, 145u32],
};

pub static DECL_71: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["chinesisches_Horoskop", "chinesischeshoroskop", "china"],
    columns: &[91u32],
};

pub static DECL_72: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["babylonische_Tierkreiszeichen", "tierkreiszeichen", "babylon"],
    columns: &[1u32, 2u32],
};

pub static DECL_73: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Thomasevangelium", "thomasevangelium", "thomas"],
    columns: &[0u32, 3u32, 303u32],
};

pub static DECL_74: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Netzwerk", "netzwerk"],
    columns: &[417u32, 436u32],
};

pub static DECL_75: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Kontroverse_(51)", "kontroverse"],
    columns: &[421u32],
};

pub static DECL_76: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["mathematisches_Design_(32)", "mathematischesdesign"],
    columns: &[419u32],
};

pub static DECL_77: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["analytische_Ontologie", "analytischeontologie", "ontologie"],
    columns: &[84u32],
};

pub static DECL_78: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["analytische_Ontologie", "analytischeontologie", "ontologie"],
    columns: &[84u32],
};

pub static DECL_79: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Transzendentalien_innen_außen", "innenaussenstrukur", "strukturalieninnenaußen", "strukturalieninnenaussen", "innenaußenstrukur", "transzendentalieninnenaußen", "transzendentalieninnenaussen"],
    columns: &[149u32],
};

pub static DECL_80: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Modallogik", "modallogik"],
    columns: &[148u32],
};

pub static DECL_81: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["5", "fünf", "fünfer", "fünferstruktur", "fuenf", "fuenfer", "fuenferstruktur"],
    columns: &[96u32],
};

pub static DECL_82: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["9", "neun", "neuner", "neunerstruktur"],
    columns: &[94u32],
};

pub static DECL_83: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["3", "drei", "dreier", "dreierstruktur"],
    columns: &[92u32, 93u32, 315u32, 316u32],
};

pub static DECL_84: PyDecl = PyDecl {
    main_aliases: &["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"],
    sub_aliases: &["Licht", "licht"],
    columns: &[20u32, 27u32, 313u32],
};

pub static DECL_85: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Strukturgrösse", "Größenordnung", "größe", "groesse", "gross", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße"],
    columns: &[4u32, 21u32, 54u32, 197u32, 425u32],
};

pub static DECL_86: PyDecl = PyDecl {
    main_aliases: &["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"],
    sub_aliases: &["Strukturgrösse", "Größenordnung", "größe", "groesse", "gross", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße"],
    columns: &[4u32, 21u32, 54u32, 197u32, 425u32],
};

pub static DECL_87: PyDecl = PyDecl {
    main_aliases: &["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"],
    sub_aliases: &["Organisationen", "organisationen", "organisation"],
    columns: &[30u32, 82u32, 425u32],
};

pub static DECL_88: PyDecl = PyDecl {
    main_aliases: &["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"],
    sub_aliases: &["politische_Systeme", "politischesysteme", "politik"],
    columns: &[83u32],
};

pub static DECL_108: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["analytische_Ontologie", "analytischeontologie", "ontologie"],
    columns: &[84u32],
};

pub static DECL_109: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Gegentranszendentalien", "gegentranszendentalien", "gegentranszendentalie", "gegenstrukturalien", "gegenalien", "gegenuniversalien"],
    columns: &[138u32, 202u32],
};

pub static DECL_110: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Systemsachen", "systemsachen"],
    columns: &[150u32],
};

pub static DECL_111: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Transzendentalien", "transzendentalien", "transzendentalie", "strukturalien", "alien", "universalien"],
    columns: &[5u32, 54u32, 55u32, 198u32, 390u32],
};

pub static DECL_112: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Reziproke_von_Transzendentalien", "transzendentalienreziproke", "transzendentaliereziproke", "strukturalienreziproke", "alienreziproke", "universalienreziproke"],
    columns: &[131u32, 201u32],
};

pub static DECL_113: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Netzwerk", "netzwerk"],
    columns: &[25u32, 55u32, 386u32, 390u32],
};

pub static DECL_114: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["warum_Transzendentalie_=_Strukturgroesse_=_Charakter", "warumtranszendentaliezustrukturgroesseundcharakter"],
    columns: &[4u32, 5u32, 54u32, 165u32],
};

pub static DECL_115: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Kategorie", "kategorie"],
    columns: &[204u32, 205u32, 281u32],
};

pub static DECL_116: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Raum-Missionen", "weltall"],
    columns: &[218u32],
};

pub static DECL_117: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Programmier-Paradigmen", "programmierparadigmen"],
    columns: &[351u32],
};

pub static DECL_118: PyDecl = PyDecl {
    main_aliases: &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
    sub_aliases: &["Raum-Missionen", "weltall"],
    columns: &[218u32],
};

pub static DECL_119: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Geist__(15)", "geist"],
    columns: &[242u32, 426u32],
};

pub static DECL_120: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["warum_Transzendentalie_=_Komplexität_von_Michael_Commons", "warumtranszendentaliegleichkomplexitaet"],
    columns: &[5u32, 65u32, 166u32],
};

pub static DECL_121: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Model_of_Hierarchical_Complexity", "modelofhierarchicalcomplexity", "komplex", "komplexität", "komplexitaet", "complexity", "model", "abstraktion"],
    columns: &[65u32, 75u32, 203u32, 483u32],
};

pub static DECL_122: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Model_of_Hierarchical_Complexity", "modelofhierarchicalcomplexity", "komplex", "komplexität", "komplexitaet", "complexity", "model", "abstraktion"],
    columns: &[65u32, 75u32, 203u32],
};

pub static DECL_123: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Model_of_Hierarchical_Complexity", "modelofhierarchicalcomplexity", "komplex", "komplexität", "komplexitaet", "complexity", "model", "abstraktion"],
    columns: &[65u32, 75u32, 203u32],
};

pub static DECL_124: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["2", "zwei", "gerade", "ungerade", "alternierung", "alternierend", "zweierstruktur"],
    columns: &[78u32, 79u32, 80u32, 331u32, 497u32, 498u32, 499u32],
};

pub static DECL_125: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["Multiplikation", "multiplikation"],
    columns: &[158u32],
};

pub static DECL_126: PyDecl = PyDecl {
    main_aliases: &["Operationen", "operationen"],
    sub_aliases: &["4", "vier", "viererstruktur", "viererabfolgen"],
    columns: &[76u32, 77u32, 81u32, 104u32, 145u32],
};

pub static DECL_127: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Gesellschaftsschicht", "klasse", "klassen"],
    columns: &[241u32],
};

pub static DECL_128: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Moral", "moral", "warummoral"],
    columns: &[215u32, 216u32],
};

pub static DECL_129: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Fachgebiete", "fachgebiete", "fachbereiche", "themen"],
    columns: &[183u32],
};

pub static DECL_130: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["Fachgebiete", "fachgebiete", "fachbereiche", "themen"],
    columns: &[183u32],
};

pub static DECL_131: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["Pflanzen", "pflanzen"],
    columns: &[113u32],
};

pub static DECL_132: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["Maschinen", "maschinen", "maschine", "gerät", "geräte", "geraete", "geraet"],
    columns: &[89u32],
};

pub static DECL_133: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["Organisationsform", "organisationsform", "organisationsart", "firma", "verein"],
    columns: &[99u32],
};

pub static DECL_134: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["System", "system"],
    columns: &[69u32, 70u32, 440u32, 455u32, 476u32, 513u32],
};

pub static DECL_135: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["System", "system"],
    columns: &[69u32, 70u32, 440u32, 455u32, 476u32, 513u32],
};

pub static DECL_136: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["Erklärung", "erklärung", "erklaerung"],
    columns: &[71u32],
};

pub static DECL_137: PyDecl = PyDecl {
    main_aliases: &["Wirtschaft", "wirtschaft"],
    sub_aliases: &["BWL", "bwl"],
    columns: &[109u32],
};

pub static DECL_138: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Sinn_des_Lebens", "sinndeslebens", "lebenssinn", "sinn", "sinnsuche"],
    columns: &[88u32, 189u32],
};

pub static DECL_139: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Intelligenzprobleme", "intelligenzprobleme", "intelligenzmaengel", "intelligenzmängel"],
    columns: &[147u32],
};

pub static DECL_140: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Denkweise_von_Lebewesen", "lebewesendenkweise", "denkweise"],
    columns: &[146u32],
};

pub static DECL_141: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Gegentranszendentalien", "gegentranszendentalien", "gegenstrukturalien"],
    columns: &[138u32, 139u32, 202u32],
};

pub static DECL_142: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Gleichheit_Freiheit", "gleichheitfreiheit", "ungleichheit", "dominieren", "gleichheit", "freiheit"],
    columns: &[132u32, 328u32, 331u32, 335u32],
};

pub static DECL_143: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Gefühle", "emotionen", "gefuehle", "emotion", "gefühl", "gefuehl"],
    columns: &[105u32, 230u32, 243u32, 283u32, 284u32, 285u32, 286u32, 305u32],
};

pub static DECL_144: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Egoismus", "egoismus", "altruismus", "selbstlosigkeit"],
    columns: &[136u32],
};

pub static DECL_145: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Wirkung", "wirkung"],
    columns: &[135u32],
};

pub static DECL_146: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["INCELs", "incel", "incels"],
    columns: &[68u32],
};

pub static DECL_147: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["irrationale_Zahlen_durch_Wurzelbildung", "irrationalezahlendurchwurzelbildung", "ausgangslage"],
    columns: &[73u32],
};

pub static DECL_148: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["dominierendes_Geschlecht", "dominierendesgeschlecht", "maennlich", "männlich", "weiblich"],
    columns: &[51u32],
};

pub static DECL_149: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Liebe", "liebe", "ethik"],
    columns: &[8u32, 9u32, 28u32, 208u32, 330u32],
};

pub static DECL_150: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Glaube_Erkenntnis", "glauben", "erkenntnis", "glaube"],
    columns: &[59u32],
};

pub static DECL_151: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Angreifbarkeit", "angreifbarkeit", "angreifbar"],
    columns: &[57u32, 58u32],
};

pub static DECL_152: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)", "Transzendentalien", "transzendentalien", "transzendentalie", "strukturalien", "alien", "universalien", "meta-paradigmen"],
    columns: &[5u32, 131u32, 229u32],
};

pub static DECL_153: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)", "Transzendentalien", "transzendentalien", "transzendentalie", "strukturalien", "alien", "universalien", "meta-paradigmen"],
    columns: &[5u32, 131u32, 229u32],
};

pub static DECL_154: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Bedingung_und_Auslöser_(1/3)", "bedingung", "bedingungen", "auslöser", "ausloeser"],
    columns: &[338u32],
};

pub static DECL_155: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Lebensbereiche_Problemklassen_(28)", "lebensbereiche", "lebensfelder", "problemklassen"],
    columns: &[405u32, 415u32, 416u32],
};

pub static DECL_156: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Maßnahmen_(39)", "massnahmen"],
    columns: &[384u32],
};

pub static DECL_157: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)", "relativreziprokuniversell"],
    columns: &[350u32],
};

pub static DECL_158: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["universeller_Komperativ_(18→15)", "universellerkomperativ"],
    columns: &[349u32],
};

pub static DECL_159: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Existenzialien_(3)", "existenzialien"],
    columns: &[348u32],
};

pub static DECL_160: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Extremalien_(19)", "extremalien"],
    columns: &[347u32, 352u32],
};

pub static DECL_161: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Erwartungshaltungen_(26)", "erwartungen", "erwartungshaltungen"],
    columns: &[344u32],
};

pub static DECL_162: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Leidenschaften_(21)", "leidenschaft", "leidenschaften"],
    columns: &[343u32],
};

pub static DECL_163: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["relativer_Zeit-Betrag_(15_10_4_18_6)", "relativerzeitbetrag"],
    columns: &[339u32],
};

pub static DECL_164: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Zahlenvergleich_(15_18_6)", "zahlenvergleich"],
    columns: &[340u32],
};

pub static DECL_165: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Bestrebungen(1/5)", "bestrebung", "bestrebungen"],
    columns: &[332u32, 414u32],
};

pub static DECL_166: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Prinzipien(1/8)", "prinzipien"],
    columns: &[329u32, 378u32],
};

pub static DECL_167: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Attraktionen_(36)", "attraktionen"],
    columns: &[311u32],
};

pub static DECL_168: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Optimierung_(10)", "optimierung"],
    columns: &[310u32],
};

pub static DECL_169: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Themen_(6)", "themen", "thema"],
    columns: &[309u32],
};

pub static DECL_170: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Bedeutung_(10)", "bedeutung"],
    columns: &[306u32],
};

pub static DECL_171: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Reziprokes", "reziproke", "reziprokes"],
    columns: &[42u32, 131u32, 204u32, 205u32, 231u32, 257u32, 273u32, 281u32, 284u32, 285u32, 326u32, 327u32, 328u32, 329u32, 330u32, 331u32, 332u32, 334u32, 335u32, 338u32, 416u32],
};

pub static DECL_172: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Achtung_(4)", "achtung", "achten"],
    columns: &[270u32, 393u32],
};

pub static DECL_173: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Zeit_(4)_als_Wirklichkeit", "zeit"],
    columns: &[266u32, 267u32],
};

pub static DECL_174: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_16_ist_zu_genügen", "absicht16"],
    columns: &[312u32],
};

pub static DECL_175: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_17_ist_zu_meinen", "absicht17"],
    columns: &[263u32],
};

pub static DECL_176: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_6_ist_Vorteilsmaximierung", "absicht6"],
    columns: &[262u32],
};

pub static DECL_177: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_7_ist_Selbstlosigkeit", "absicht7"],
    columns: &[261u32],
};

pub static DECL_178: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Bewusstheit_statt_Bewusstsein_(1)", "bewusstheit"],
    columns: &[282u32],
};

pub static DECL_179: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Verhalten_(11)", "verhalten"],
    columns: &[301u32, 302u32, 413u32],
};

pub static DECL_180: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Energie_und_universelle_Eigenschaften_(30)", "energie", "universelleeigenschaften", "lebensenergie"],
    columns: &[287u32, 293u32],
};

pub static DECL_181: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Garben_und_Verhalten_nachfühlen(31)", "garben", "verhaltenfuehlen", "verhaltenfühlen"],
    columns: &[295u32],
};

pub static DECL_182: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)", "nachvollziehen"],
    columns: &[242u32, 297u32],
};

pub static DECL_183: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Empathie_(37)", "empathie", "mitgefuehl"],
    columns: &[294u32],
};

pub static DECL_184: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_1/6_ist_Reinigung_und_Klarheit", "absicht1/6", "absicht1pro6"],
    columns: &[298u32],
};

pub static DECL_185: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["innere_Werte_1/6_der_Reinigung_und_Klarheit", "innerewerte"],
    columns: &[398u32, 399u32, 400u32, 401u32],
};

pub static DECL_186: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_10_ist_Wirklichkeit_erkennen", "absicht10"],
    columns: &[260u32],
};

pub static DECL_187: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Wohlbefinden_(7mit6)", "wohlbefinden"],
    columns: &[427u32, 428u32],
};

pub static DECL_188: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Geist_(15)", "geist", "bewusstsein"],
    columns: &[229u32, 231u32, 242u32, 273u32, 297u32, 304u32, 426u32],
};

pub static DECL_189: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Geist_(15)", "geist", "bewusstsein"],
    columns: &[229u32, 231u32, 242u32, 273u32, 297u32, 304u32, 426u32],
};

pub static DECL_190: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Reflexe_(3)", "reflex", "reflexe"],
    columns: &[256u32],
};

pub static DECL_191: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Lust_(9)", "lust", "einheiten"],
    columns: &[255u32, 391u32],
};

pub static DECL_192: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Paradigmen_sind_Absichten_(13)", "paradigmen", "absichten"],
    columns: &[10u32, 42u32, 410u32, 411u32, 493u32, 494u32],
};

pub static DECL_193: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Wirklichkeiten_Wahrheit_Wahrnehmung_(10)", "wirklichkeit", "wirklichkeiten", "wahrheit", "wahrnehmung"],
    columns: &[233u32, 265u32, 268u32, 322u32, 342u32, 480u32],
};

pub static DECL_194: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Rechnen", "rechnen"],
    columns: &[404u32],
};

pub static DECL_195: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Stimmungen_Kombinationen_(14)", "stimmung", "stimmungen", "kombination", "kombinationen"],
    columns: &[33u32, 290u32, 296u32, 325u32, 326u32, 327u32, 402u32, 403u32, 406u32, 407u32, 408u32, 430u32, 492u32],
};

pub static DECL_196: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Struktur-Wissenschaften_(10)"],
    columns: &[438u32],
};

pub static DECL_197: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Muster-Wissenschaften_(20)"],
    columns: &[439u32, 484u32],
};

pub static DECL_198: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Anführer_Arten_(7)"],
    columns: &[429u32, 455u32, 481u32, 482u32, 490u32, 497u32, 498u32, 499u32, 502u32, 509u32],
};

pub static DECL_199: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Klassen_(20)", "klasse", "klassen"],
    columns: &[241u32, 289u32, 394u32, 395u32, 485u32, 516u32],
};

pub static DECL_200: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Ordnung_und_Filterung_12_und_1pro12", "ordnen", "ordnenundfiltern", "filtern"],
    columns: &[132u32, 328u32, 331u32, 335u32],
};

pub static DECL_201: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Meta-Systeme_(12)", "metasysteme", "metasystem", "meta-systeme", "meta-system", "menge", "mengen"],
    columns: &[79u32, 80u32, 232u32, 288u32, 334u32, 410u32, 411u32, 483u32, 497u32, 498u32, 499u32],
};

pub static DECL_202: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_1/8", "absicht1pro8", "absicht1/8"],
    columns: &[272u32, 379u32],
};

pub static DECL_203: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Ziele_(19)", "ziele", "maxima", "höhenvorstellungen"],
    columns: &[271u32],
};

pub static DECL_204: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Konkreta_und_Focus_(2)", "konkreta", "focus", "fokus"],
    columns: &[250u32, 253u32, 269u32],
};

pub static DECL_205: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Gefühle_(7)", "gefuehle", "emotionen", "emotion", "gefühle"],
    columns: &[29u32, 243u32, 283u32, 284u32, 285u32, 286u32, 305u32],
};

pub static DECL_206: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["abhängige_Verbundenheit_(90)", "abhaengigkeit", "abhängigkeit"],
    columns: &[357u32],
};

pub static DECL_207: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Karte_Filter_und_Unterscheidung_(1/12)", "karte", "filter", "unterscheidung"],
    columns: &[377u32],
};

pub static DECL_208: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Fundament_(1/19)", "fundament"],
    columns: &[356u32],
};

pub static DECL_209: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Gedanken_sind_Positionen_(17)", "positionen", "gedanken"],
    columns: &[249u32, 317u32, 323u32],
};

pub static DECL_210: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Funktionen_Vorstellungen_(16)", "vorstellungen", "vorstellung", "funktionen"],
    columns: &[264u32, 345u32, 388u32, 418u32],
};

pub static DECL_211: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Sollen_Frage_Vorgehensweise_(1/13)", "sollen", "frage", "vorgehensweise"],
    columns: &[353u32, 354u32],
};

pub static DECL_212: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Ansichten_Standpunkte_(18_17)", "ansichten"],
    columns: &[240u32, 346u32],
};

pub static DECL_213: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Verbundenheiten_(18)", "verbundenheiten"],
    columns: &[252u32, 299u32, 300u32, 336u32],
};

pub static DECL_214: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Absicht_13_ist_Helfen", "absicht13", "helfen"],
    columns: &[370u32],
};

pub static DECL_215: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Liebe_(7)", "liebe"],
    columns: &[8u32, 9u32, 28u32, 208u32, 221u32, 330u32],
};

pub static DECL_216: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Koalitionen_(10)", "koalitionen"],
    columns: &[321u32],
};

pub static DECL_217: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["gegen_5"],
    columns: &[24u32],
};

pub static DECL_218: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Impulse_(5)", "impulse"],
    columns: &[251u32, 253u32, 257u32, 341u32],
};

pub static DECL_219: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Triebe_und_Bedürfnisse_(6)", "trieb", "triebe", "bedürfnis", "bedürfnisse", "werte"],
    columns: &[254u32, 392u32, 396u32, 397u32, 423u32],
};

pub static DECL_220: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Taetigkeiten", "tätigkeiten", "taetigkeiten"],
    columns: &[424u32],
};

pub static DECL_221: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Reflektion_und_Kategorien_(1/15)", "reflektion", "kategorien"],
    columns: &[204u32, 205u32, 281u32],
};

pub static DECL_222: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Modus_und_Sein_(8)", "zustaende", "zustände", "modus", "modi", "sein"],
    columns: &[234u32, 337u32, 385u32, 387u32, 491u32],
};

pub static DECL_223: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Motive", "motive", "motivation", "motiv", "absicht", "absichten"],
    columns: &[10u32, 18u32, 42u32, 149u32, 167u32, 168u32, 229u32, 230u32],
};

pub static DECL_224: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Gedanken_sind_Positionen_(17)", "positionen", "gedanken"],
    columns: &[249u32, 276u32],
};

pub static DECL_225: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Bewusstsein_und_Wahrnehmung", "bewusstsein", "wahrnehmung"],
    columns: &[229u32, 231u32, 265u32, 281u32, 304u32, 342u32],
};

pub static DECL_226: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Errungenschaften", "errungenschaften", "ziele", "erhalten"],
    columns: &[11u32, 251u32, 257u32],
};

pub static DECL_227: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["evolutionär_erwerben_und_Intelligenz_Kreativität", "evolutionärerwerbenundintelligenz", "intelligenz", "erwerben", "erlernen", "lernen", "evolutionaer", "evolutionär", "kreativität", "kreativitaet", "kreativ"],
    columns: &[12u32, 13u32, 27u32, 32u32, 47u32],
};

pub static DECL_228: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["brauchen", "benoetigen", "benötigen", "notwendig"],
    columns: &[13u32, 14u32],
};

pub static DECL_229: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Krankheit", "krankheit", "krankheiten", "pathologisch", "pathologie", "psychiatrisch"],
    columns: &[24u32],
};

pub static DECL_230: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["alpha_beta", "alphabeta", "alpha", "beta", "omega", "sigma"],
    columns: &[46u32],
};

pub static DECL_231: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Anführer", "anfuehrer", "chef"],
    columns: &[29u32, 170u32, 429u32, 455u32, 490u32, 502u32, 509u32],
};

pub static DECL_232: PyDecl = PyDecl {
    main_aliases: &["Grundstrukturen", "grundstrukturen"],
    sub_aliases: &["Biologischer_Baum_(15)"],
    columns: &[500u32],
};

pub static DECL_233: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["Biologischer_Baum_(16_->_5)"],
    columns: &[500u32],
};

pub static DECL_234: PyDecl = PyDecl {
    main_aliases: &["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"],
    sub_aliases: &["Biologischer_Baum_(15)"],
    columns: &[500u32],
};

pub static DECL_235: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Biologischer_Baum_(15)"],
    columns: &[500u32],
};

pub static DECL_236: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Manipulation", "manipulation"],
    columns: &[153u32],
};

pub static DECL_237: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Berufe", "berufe", "beruf"],
    columns: &[30u32],
};

pub static DECL_238: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Lösungen", "lösungen", "loesungen", "loesung", "lösungen"],
    columns: &[31u32],
};

pub static DECL_239: PyDecl = PyDecl {
    main_aliases: &["Menschliches", "menschliches"],
    sub_aliases: &["Musik", "musik"],
    columns: &[33u32],
};

pub static DECL_240: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["ergibt_Sinn", "ergibtsinn", "machtsinn", "sinn"],
    columns: &[140u32],
};

pub static DECL_241: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Veränderung", "veraenderung", "veraendern", "veränderung", "verändern"],
    columns: &[142u32],
};

pub static DECL_242: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["bändigen_kontrollieren", "baendigenkontrollieren", "kontrollieren", "baendigen", "bändigen"],
    columns: &[143u32],
};

pub static DECL_243: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["vereinen", "einheit"],
    columns: &[144u32],
};

pub static DECL_244: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Vorteile", "vorteile", "veraenderungnutzen"],
    columns: &[141u32],
};

pub static DECL_245: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Gegenspieler", "gegenspieler", "antagonist"],
    columns: &[137u32],
};

pub static DECL_246: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["nervig"],
    columns: &[120u32],
};

pub static DECL_247: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["pro_nutzen", "pronutzen"],
    columns: &[117u32],
};

pub static DECL_248: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Gegenposition", "gegenposition"],
    columns: &[116u32],
};

pub static DECL_249: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Hilfe_erhalten", "hilfeerhalten"],
    columns: &[114u32],
};

pub static DECL_250: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Helfen", "helfen", "hilfe"],
    columns: &[115u32],
};

pub static DECL_251: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Pro", "pro", "dafür", "dafuer"],
    columns: &[17u32, 48u32],
};

pub static DECL_252: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["nicht_miteinander_auskommen", "nichtauskommen"],
    columns: &[123u32],
};

pub static DECL_253: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["nicht_dagegen", "nichtdagegen"],
    columns: &[124u32],
};

pub static DECL_254: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["kein_Gegenteil", "keingegenteil"],
    columns: &[125u32],
};

pub static DECL_255: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["nicht_dafür", "nichtdafuer"],
    columns: &[126u32],
};

pub static DECL_256: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Hilfe_nicht_gebrauchen", "hilfenichtgebrauchen"],
    columns: &[127u32],
};

pub static DECL_257: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["nicht_helfen_können", "nichthelfenkoennen"],
    columns: &[128u32],
};

pub static DECL_258: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["nicht_abgeneigt", "nichtabgeneigt"],
    columns: &[129u32],
};

pub static DECL_259: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["unmotivierbar"],
    columns: &[130u32],
};

pub static DECL_260: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["contra", "dagegen"],
    columns: &[15u32, 26u32],
};

pub static DECL_261: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Gegenteil", "gegenteil"],
    columns: &[100u32, 101u32, 222u32],
};

pub static DECL_262: PyDecl = PyDecl {
    main_aliases: &["Pro_Contra", "procontra", "dagegendafuer"],
    sub_aliases: &["Harmonie", "harmonie"],
    columns: &[102u32, 103u32],
};

pub static DECL_266: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["in_ReTa", "inreta"],
    columns: &[209u32, 210u32, 474u32, 475u32],
};

pub static DECL_267: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Vorzeichen", "vorzeichen"],
    columns: &[118u32, 119u32],
};

pub static DECL_268: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Primzahlen", "primzahlen", "vielfache", "vielfacher"],
    columns: &[19u32],
};

pub static DECL_269: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Anwendung_der_Sonnen_und_Monde", "anwendungdersonnenundmonde", "anwendungdersonnen", "anwendungenfuermonde"],
    columns: &[22u32],
};

pub static DECL_270: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Zählungen", "zählungen", "zaehlung", "zaehlungen", "zählung"],
    columns: &[25u32, 45u32, 169u32, 188u32, 386u32, 390u32],
};

pub static DECL_271: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Jura", "jura", "gesetzeslehre", "recht"],
    columns: &[34u32],
};

pub static DECL_272: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Vollkommenheit_des_Geistes", "vollkommenheit", "geist"],
    columns: &[35u32],
};

pub static DECL_273: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Gestirn", "gestirn", "mond", "sonne", "planet"],
    columns: &[64u32, 154u32],
};

pub static DECL_274: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Konjunktiv_Wurzelbildung", "konjunktiv", "wurzel"],
    columns: &[106u32],
};

pub static DECL_275: PyDecl = PyDecl {
    main_aliases: &["Bedeutung", "bedeutung"],
    sub_aliases: &["Mechanismen_der_Züchtung", "mechanismen", "wesen", "zuechtung", "züchtung", "züchten", "zuechten"],
    columns: &[107u32, 108u32, 109u32],
};

pub static DECL_280: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Religionen"],
    columns: &[36u32, 37u32],
};

pub static DECL_281: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Drei"],
    columns: &[452u32, 460u32],
};

pub static DECL_282: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Vier"],
    columns: &[453u32],
};

pub static DECL_283: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Fünf", "Fuenf"],
    columns: &[454u32],
};

pub static DECL_284: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Sechs"],
    columns: &[457u32],
};

pub static DECL_285: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Sieben"],
    columns: &[457u32],
};

pub static DECL_286: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Acht"],
    columns: &[458u32],
};

pub static DECL_287: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Neun"],
    columns: &[459u32],
};

pub static DECL_288: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Zehn"],
    columns: &[456u32],
};

pub static DECL_289: PyDecl = PyDecl {
    main_aliases: &["Symbole", "symbole"],
    sub_aliases: &["Zwölf", "Zwoelf"],
    columns: &[456u32],
};

pub static DECL_290: PyDecl = PyDecl {
    main_aliases: &["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"],
    sub_aliases: &["Weisheit_etc", "weisheit", "metaweisheit", "meta-weisheit", "idiot", "weise", "optimal", "optimum"],
    columns: &[112u32],
};

pub static DECL_296: PyDecl = PyDecl {
    main_aliases: &["Eigenschaften_1/n", "konzept2", "konzepte2"],
    sub_aliases: &["Filterart_Widrigkeit", "filterart", "widrigkeit"],
    columns: &[331u32, 335u32],
};

pub static DECL_320: PyDecl = PyDecl {
    main_aliases: &["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"],
    sub_aliases: &["ähnlich", "aehnlich"],
    columns: &[220u32],
};

pub static DECL_321: PyDecl = PyDecl {
    main_aliases: &["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"],
    sub_aliases: &["gut_böse_lieb_schlecht", "gut", "böse", "boese", "lieb", "schlecht"],
    columns: &[52u32, 53u32],
};

pub static DECL_322: PyDecl = PyDecl {
    main_aliases: &["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"],
    sub_aliases: &["Sinn_und_Zweck_des_Lebens", "sinn", "zweck", "bedeutung"],
    columns: &[88u32, 189u32],
};

pub static DECL_333: PyDecl = PyDecl {
    main_aliases: &["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"],
    sub_aliases: &["Egoismus_vs_Altruismus", "egoismus", "altruismus", "egoist", "altruist"],
    columns: &[136u32],
};

pub static DECL_338: PyDecl = PyDecl {
    main_aliases: &["Inkrementieren", "inkrementieren"],
    sub_aliases: &["um1"],
    columns: &[155u32],
};

pub static DECL_339: PyDecl = PyDecl {
    main_aliases: &["Inkrementieren", "inkrementieren"],
    sub_aliases: &["um2"],
    columns: &[156u32],
};

pub static DECL_340: PyDecl = PyDecl {
    main_aliases: &["Inkrementieren", "inkrementieren"],
    sub_aliases: &["um3"],
    columns: &[157u32],
};

pub static DECL_341: PyDecl = PyDecl {
    main_aliases: &["Inkrementieren", "inkrementieren"],
    sub_aliases: &["warum_Transzendentalie_=_Strukturgroesse_=_Charakter", "warumtranszendentaliezustrukturgroesseundcharakter"],
    columns: &[4u32, 5u32, 54u32, 165u32],
};

pub static DECL_342: PyDecl = PyDecl {
    main_aliases: &["Inkrementieren", "inkrementieren"],
    sub_aliases: &["warum_Transzendentalie_=_Komplexität_von_Michael_Commons", "warumtranszendentaliegleichkomplexitaet"],
    columns: &[5u32, 65u32, 166u32],
};

pub static DECL_343: PyDecl = PyDecl {
    main_aliases: &["Primvielfache", "primvielfache"],
    sub_aliases: &["Rahmen-Bedingungen", "rahmen"],
    columns: &[226u32],
};

pub static DECL_353: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["Q", "q", "Siebzehn"],
    columns: &[431u32, 432u32, 433u32, 434u32, 437u32, 441u32, 442u32, 443u32, 445u32, 450u32, 467u32, 468u32, 469u32, 487u32, 488u32],
};

pub static DECL_354: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["i", "I", "Neun"],
    columns: &[517u32],
};

pub static DECL_355: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["G", "g", "Sieben"],
    columns: &[518u32],
};

pub static DECL_356: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["J", "j", "Zehn"],
    columns: &[514u32],
};

pub static DECL_357: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["k", "K", "Elf"],
    columns: &[515u32],
};

pub static DECL_358: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["E", "e", "Fünf"],
    columns: &[511u32],
};

pub static DECL_359: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["L", "l", "Zwölf"],
    columns: &[506u32],
};

pub static DECL_360: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["Y", "y", "Fünfundzwanzig"],
    columns: &[507u32, 510u32],
};

pub static DECL_361: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["Kontinuen", "F", "f", "Sechs"],
    columns: &[508u32],
};

pub static DECL_362: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["F", "f", "Sechs", "Kontinuen"],
    columns: &[508u32],
};

pub static DECL_363: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["O", "o", "Fünfzehn"],
    columns: &[5u32],
};

pub static DECL_364: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["H", "h", "Acht"],
    columns: &[491u32],
};

pub static DECL_365: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["N", "n", "Vierzehn"],
    columns: &[492u32],
};

pub static DECL_366: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["M", "m", "Dreizehn"],
    columns: &[493u32],
};

pub static DECL_367: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["T", "t", "Zwanzig"],
    columns: &[486u32],
};

pub static DECL_368: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["P", "p", "Sechszehn"],
    columns: &[435u32],
};

pub static DECL_369: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["P5", "p5", "Sechszehn->Fünf"],
    columns: &[501u32],
};

pub static DECL_370: PyDecl = PyDecl {
    main_aliases: &["Multiversum", "multiversum"],
    sub_aliases: &["P5", "p5", "Sechszehn->Fünf"],
    columns: &[501u32],
};

pub static DECL_371: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["P", "p", "Sechszehn"],
    columns: &[435u32],
};

pub static DECL_372: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["X", "x", "Vierundzwanzig"],
    columns: &[25u32, 55u32, 386u32, 436u32],
};

pub static DECL_373: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["S", "s", "Neunzehn"],
    columns: &[504u32],
};

pub static DECL_374: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["R", "r", "Achtzehn"],
    columns: &[436u32, 451u32],
};

pub static DECL_375: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["A", "a", "Eins"],
    columns: &[446u32],
};

pub static DECL_376: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["B", "b", "Zwei"],
    columns: &[447u32],
};

pub static DECL_377: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["C", "c", "Drei"],
    columns: &[448u32],
};

pub static DECL_378: PyDecl = PyDecl {
    main_aliases: &["Kontinuum", "kontinuum"],
    sub_aliases: &["D", "d", "Vier"],
    columns: &[449u32],
};

pub static PY_DECLS: &[PyDecl] = &[
    DECL_0,
    DECL_1,
    DECL_2,
    DECL_3,
    DECL_4,
    DECL_5,
    DECL_6,
    DECL_7,
    DECL_8,
    DECL_9,
    DECL_10,
    DECL_11,
    DECL_12,
    DECL_13,
    DECL_14,
    DECL_15,
    DECL_16,
    DECL_17,
    DECL_19,
    DECL_20,
    DECL_21,
    DECL_22,
    DECL_23,
    DECL_24,
    DECL_25,
    DECL_26,
    DECL_27,
    DECL_28,
    DECL_29,
    DECL_30,
    DECL_31,
    DECL_32,
    DECL_33,
    DECL_34,
    DECL_35,
    DECL_36,
    DECL_37,
    DECL_38,
    DECL_39,
    DECL_40,
    DECL_41,
    DECL_42,
    DECL_43,
    DECL_44,
    DECL_45,
    DECL_46,
    DECL_47,
    DECL_48,
    DECL_49,
    DECL_50,
    DECL_51,
    DECL_52,
    DECL_53,
    DECL_54,
    DECL_55,
    DECL_56,
    DECL_57,
    DECL_58,
    DECL_59,
    DECL_60,
    DECL_61,
    DECL_62,
    DECL_63,
    DECL_64,
    DECL_65,
    DECL_66,
    DECL_67,
    DECL_68,
    DECL_69,
    DECL_70,
    DECL_71,
    DECL_72,
    DECL_73,
    DECL_74,
    DECL_75,
    DECL_76,
    DECL_77,
    DECL_78,
    DECL_79,
    DECL_80,
    DECL_81,
    DECL_82,
    DECL_83,
    DECL_84,
    DECL_85,
    DECL_86,
    DECL_87,
    DECL_88,
    DECL_108,
    DECL_109,
    DECL_110,
    DECL_111,
    DECL_112,
    DECL_113,
    DECL_114,
    DECL_115,
    DECL_116,
    DECL_117,
    DECL_118,
    DECL_119,
    DECL_120,
    DECL_121,
    DECL_122,
    DECL_123,
    DECL_124,
    DECL_125,
    DECL_126,
    DECL_127,
    DECL_128,
    DECL_129,
    DECL_130,
    DECL_131,
    DECL_132,
    DECL_133,
    DECL_134,
    DECL_135,
    DECL_136,
    DECL_137,
    DECL_138,
    DECL_139,
    DECL_140,
    DECL_141,
    DECL_142,
    DECL_143,
    DECL_144,
    DECL_145,
    DECL_146,
    DECL_147,
    DECL_148,
    DECL_149,
    DECL_150,
    DECL_151,
    DECL_152,
    DECL_153,
    DECL_154,
    DECL_155,
    DECL_156,
    DECL_157,
    DECL_158,
    DECL_159,
    DECL_160,
    DECL_161,
    DECL_162,
    DECL_163,
    DECL_164,
    DECL_165,
    DECL_166,
    DECL_167,
    DECL_168,
    DECL_169,
    DECL_170,
    DECL_171,
    DECL_172,
    DECL_173,
    DECL_174,
    DECL_175,
    DECL_176,
    DECL_177,
    DECL_178,
    DECL_179,
    DECL_180,
    DECL_181,
    DECL_182,
    DECL_183,
    DECL_184,
    DECL_185,
    DECL_186,
    DECL_187,
    DECL_188,
    DECL_189,
    DECL_190,
    DECL_191,
    DECL_192,
    DECL_193,
    DECL_194,
    DECL_195,
    DECL_196,
    DECL_197,
    DECL_198,
    DECL_199,
    DECL_200,
    DECL_201,
    DECL_202,
    DECL_203,
    DECL_204,
    DECL_205,
    DECL_206,
    DECL_207,
    DECL_208,
    DECL_209,
    DECL_210,
    DECL_211,
    DECL_212,
    DECL_213,
    DECL_214,
    DECL_215,
    DECL_216,
    DECL_217,
    DECL_218,
    DECL_219,
    DECL_220,
    DECL_221,
    DECL_222,
    DECL_223,
    DECL_224,
    DECL_225,
    DECL_226,
    DECL_227,
    DECL_228,
    DECL_229,
    DECL_230,
    DECL_231,
    DECL_232,
    DECL_233,
    DECL_234,
    DECL_235,
    DECL_236,
    DECL_237,
    DECL_238,
    DECL_239,
    DECL_240,
    DECL_241,
    DECL_242,
    DECL_243,
    DECL_244,
    DECL_245,
    DECL_246,
    DECL_247,
    DECL_248,
    DECL_249,
    DECL_250,
    DECL_251,
    DECL_252,
    DECL_253,
    DECL_254,
    DECL_255,
    DECL_256,
    DECL_257,
    DECL_258,
    DECL_259,
    DECL_260,
    DECL_261,
    DECL_262,
    DECL_266,
    DECL_267,
    DECL_268,
    DECL_269,
    DECL_270,
    DECL_271,
    DECL_272,
    DECL_273,
    DECL_274,
    DECL_275,
    DECL_280,
    DECL_281,
    DECL_282,
    DECL_283,
    DECL_284,
    DECL_285,
    DECL_286,
    DECL_287,
    DECL_288,
    DECL_289,
    DECL_290,
    DECL_296,
    DECL_320,
    DECL_321,
    DECL_322,
    DECL_333,
    DECL_338,
    DECL_339,
    DECL_340,
    DECL_341,
    DECL_342,
    DECL_343,
    DECL_353,
    DECL_354,
    DECL_355,
    DECL_356,
    DECL_357,
    DECL_358,
    DECL_359,
    DECL_360,
    DECL_361,
    DECL_362,
    DECL_363,
    DECL_364,
    DECL_365,
    DECL_366,
    DECL_367,
    DECL_368,
    DECL_369,
    DECL_370,
    DECL_371,
    DECL_372,
    DECL_373,
    DECL_374,
    DECL_375,
    DECL_376,
    DECL_377,
    DECL_378,
];

pub static EXACT_HTML_META: &[(u32, &str)] = &[
    (0u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Religionen,✗Religionen,✗Galaxie,, p2_p3_0_Wichtigste,p3_1_Sternpolygon,p3_2_der_Tierkreiszeichen,p3_3_Thomasevangelium,p3_4_, p4_3,0"),
    (1u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Galaxie,, p2_p3_0_Wichtigste,p3_1_babylonische_Tierkreiszeichen,p3_2_, p4_3,0"),
    (2u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Galaxie,, p2_p3_0_Wichtigste,p3_1_babylonische_Tierkreiszeichen,p3_2_, p4_3,0"),
    (3u32, "p1_✗Galaxie,, p2_p3_0_Thomasevangelium,p3_1_, p4_3,0"),
    (4u32, "p1_✗Wichtigstes_zum_verstehen,✗Grundstrukturen,✗Größenordnung,✗Universum,✗Inkrementieren,, p2_p3_0_Wichtigste,p3_1_Strukturgrösse,p3_2_Strukturgrösse,p3_3_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_4_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_5_, p4_3,4,0,5"),
    (5u32, "p1_✗Wichtigstes_zum_verstehen,✗Universum,✗Universum,✗Universum,✗Grundstrukturen,✗Multiversum,✗Inkrementieren,✗Inkrementieren,✗Kontinuum,, p2_p3_0_Wichtigste,p3_1_Transzendentalien,p3_2_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_3_warum_Transzendentalie_=_Komplexität_von_Michael_Commons,p3_4_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_5_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_6_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_7_warum_Transzendentalie_=_Komplexität_von_Michael_Commons,p3_8_O,p3_9_, p4_4,0"),
    (6u32, "p1_✗Religionen,, p2_p3_0_Sternpolygon,p3_1_, p4_3,0"),
    (7u32, "p1_✗Religionen,, p2_p3_0_Messias,p3_1_, p4_3,0"),
    (8u32, "p1_✗Wichtigstes_zum_verstehen,✗Menschliches,✗Grundstrukturen,, p2_p3_0_Wichtigste,p3_1_Liebe,p3_2_Liebe_(7),p3_3_, p4_0,5"),
    (9u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Liebe,p3_1_Liebe_(7),p3_2_, p4_0,5"),
    (10u32, "p1_✗Wichtigstes_zum_verstehen,✗Grundstrukturen,✗Menschliches,, p2_p3_0_Wichtigste,p3_1_Paradigmen_sind_Absichten_(13),p3_2_Motive,p3_3_, p4_3,0"),
    (11u32, "p1_✗Menschliches,, p2_p3_0_Errungenschaften,p3_1_, p4_3,0"),
    (12u32, "p1_✗Menschliches,, p2_p3_0_evolutionär_erwerben_und_Intelligenz_Kreativität,p3_1_, p4_3,0"),
    (13u32, "p1_✗Menschliches,✗Menschliches,, p2_p3_0_evolutionär_erwerben_und_Intelligenz_Kreativität,p3_1_brauchen,p3_2_, p4_3,0"),
    (14u32, "p1_✗Menschliches,, p2_p3_0_brauchen,p3_1_, p4_3,0"),
    (15u32, "p1_✗Pro_Contra,, p2_p3_0_contra,p3_1_, p4_3,4,0,5"),
    (16u32, "p1_✗Religionen,, p2_p3_0_gleichförmiges_Polygon,p3_1_, p4_3,1"),
    (17u32, "p1_✗Pro_Contra,, p2_p3_0_Pro,p3_1_, p4_3,4,0,5"),
    (18u32, "p1_✗Menschliches,, p2_p3_0_Motive,p3_1_, p4_3,0"),
    (19u32, "p1_✗Wichtigstes_zum_verstehen,✗Bedeutung,, p2_p3_0_Zweitwichtigste,p3_1_Primzahlen,p3_2_, p4_3,0"),
    (20u32, "p1_✗Größenordnung,✗Licht,, p2_p3_0_Licht,p3_1_,p3_2_, p4_3,4,0,5"),
    (21u32, "p1_✗Grundstrukturen,✗Größenordnung,, p2_p3_0_Strukturgrösse,p3_1_Strukturgrösse,p3_2_, p4_3,4,0,5"),
    (22u32, "p1_✗Bedeutung,, p2_p3_0_Anwendung_der_Sonnen_und_Monde,p3_1_, p4_3,0"),
    (23u32, "p1_✗Religionen,, p2_p3_0_Vertreter_höherer_Konzepte,p3_1_, p4_3,0"),
    (24u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_gegen_5,p3_1_Krankheit,p3_2_, p4_3,0"),
    (25u32, "p1_✗Universum,✗Bedeutung,✗Kontinuum,, p2_p3_0_Netzwerk,p3_1_Zählungen,p3_2_X,p3_3_, p4_4,0"),
    (26u32, "p1_✗Pro_Contra,, p2_p3_0_contra,p3_1_, p4_3,4,0,5"),
    (27u32, "p1_✗Größenordnung,✗Menschliches,✗Licht,, p2_p3_0_Licht,p3_1_evolutionär_erwerben_und_Intelligenz_Kreativität,p3_2_,p3_3_, p4_4,0"),
    (28u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Liebe,p3_1_Liebe_(7),p3_2_, p4_0,5"),
    (29u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Gefühle_(7),p3_1_Anführer,p3_2_, p4_3,0"),
    (30u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Größenordnung,✗Menschliches,, p2_p3_0_Zweitwichtigste,p3_1_Organisationen,p3_2_Berufe,p3_3_, p4_3,0"),
    (31u32, "p1_✗Menschliches,, p2_p3_0_Lösungen,p3_1_, p4_3,0"),
    (32u32, "p1_✗Menschliches,, p2_p3_0_evolutionär_erwerben_und_Intelligenz_Kreativität,p3_1_, p4_3,0"),
    (33u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_Musik,p3_2_, p4_3,0"),
    (34u32, "p1_✗Universum,✗Bedeutung,, p2_p3_0_universelles_Recht,p3_1_Jura,p3_2_, p4_3,0"),
    (35u32, "p1_✗Bedeutung,, p2_p3_0_Vollkommenheit_des_Geistes,p3_1_, p4_3,0"),
    (36u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Religionen,✗Religionen,✗Symbole,, p2_p3_0_Wichtigste,p3_1_Sternpolygon,p3_2_der_Tierkreiszeichen,p3_3_Religionen,p3_4_, p4_3,4,0,5"),
    (37u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Religionen,✗Symbole,, p2_p3_0_Wichtigste,p3_1_gleichförmiges_Polygon,p3_2_Religionen,p3_3_, p4_3,5,1,4"),
    (42u32, "p1_✗Grundstrukturen,✗Grundstrukturen,✗Menschliches,, p2_p3_0_Reziprokes,p3_1_Paradigmen_sind_Absichten_(13),p3_2_Motive,p3_3_, p4_3,1"),
    (43u32, "p1_✗Inkrementieren,, p2_p3_0_,p3_1_, p4_3,0"),
    (45u32, "p1_✗Bedeutung,, p2_p3_0_Zählungen,p3_1_, p4_3,0"),
    (46u32, "p1_✗Menschliches,, p2_p3_0_alpha_beta,p3_1_, p4_3,0"),
    (47u32, "p1_✗Menschliches,, p2_p3_0_evolutionär_erwerben_und_Intelligenz_Kreativität,p3_1_, p4_3,0"),
    (48u32, "p1_✗Pro_Contra,, p2_p3_0_Pro,p3_1_, p4_3,4,0,5"),
    (51u32, "p1_✗Menschliches,, p2_p3_0_dominierendes_Geschlecht,p3_1_, p4_3,0"),
    (52u32, "p1_✗Eigenschaften_n,, p2_p3_0_gut_böse_lieb_schlecht,p3_1_, p4_3,1,0"),
    (53u32, "p1_✗Eigenschaften_n,, p2_p3_0_gut_böse_lieb_schlecht,p3_1_, p4_3,1,0"),
    (54u32, "p1_✗Grundstrukturen,✗Größenordnung,✗Universum,✗Universum,✗Inkrementieren,✗Inkrementieren,, p2_p3_0_Strukturgrösse,p3_1_Strukturgrösse,p3_2_Transzendentalien,p3_3_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_4_,p3_5_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_6_, p4_3,4,0"),
    (55u32, "p1_✗Universum,✗Universum,✗Kontinuum,, p2_p3_0_Transzendentalien,p3_1_Netzwerk,p3_2_X,p3_3_, p4_4,0"),
    (57u32, "p1_✗Menschliches,, p2_p3_0_Angreifbarkeit,p3_1_, p4_3,0"),
    (58u32, "p1_✗Menschliches,, p2_p3_0_Angreifbarkeit,p3_1_, p4_3,1"),
    (59u32, "p1_✗Menschliches,, p2_p3_0_Glaube_Erkenntnis,p3_1_, p4_3,0"),
    (64u32, "p1_✗Wichtigstes_zum_verstehen,✗Bedeutung,, p2_p3_0_Drittwichtigste,p3_1_Gestirn,p3_2_, p4_3,0"),
    (65u32, "p1_✗Wichtigstes_zum_verstehen,✗Universum,✗Universum,✗Grundstrukturen,✗Universum,✗Multiversum,✗Inkrementieren,, p2_p3_0_Zweitwichtigste,p3_1_universelles_Recht,p3_2_warum_Transzendentalie_=_Komplexität_von_Michael_Commons,p3_3_Model_of_Hierarchical_Complexity,p3_4_Model_of_Hierarchical_Complexity,p3_5_Model_of_Hierarchical_Complexity,p3_6_warum_Transzendentalie_=_Komplexität_von_Michael_Commons,p3_7_, p4_4,0"),
    (68u32, "p1_✗Menschliches,, p2_p3_0_INCELs,p3_1_, p4_3,0"),
    (69u32, "p1_✗Grundstrukturen,✗Wirtschaft,, p2_p3_0_System,p3_1_System,p3_2_, p4_4,0"),
    (70u32, "p1_✗Grundstrukturen,✗Wirtschaft,, p2_p3_0_System,p3_1_System,p3_2_, p4_4,0"),
    (71u32, "p1_✗Wirtschaft,, p2_p3_0_Erklärung,p3_1_, p4_3,0"),
    (72u32, "p1_✗Religionen,, p2_p3_0_Religions-Gründer-Typ,p3_1_, p4_3,0"),
    (73u32, "p1_✗Menschliches,, p2_p3_0_irrationale_Zahlen_durch_Wurzelbildung,p3_1_, p4_3,0"),
    (74u32, "p1_✗Inkrementieren,, p2_p3_0_,p3_1_, p4_3,0"),
    (75u32, "p1_✗Grundstrukturen,✗Universum,✗Multiversum,, p2_p3_0_Model_of_Hierarchical_Complexity,p3_1_Model_of_Hierarchical_Complexity,p3_2_Model_of_Hierarchical_Complexity,p3_3_, p4_3,4,0"),
    (76u32, "p1_✗Operationen,, p2_p3_0_4,p3_1_, p4_3,4,0"),
    (77u32, "p1_✗Universum,✗Galaxie,✗Operationen,, p2_p3_0_Kugeln_Kreise,p3_1_Kugeln_Kreise,p3_2_4,p3_3_, p4_4,0"),
    (78u32, "p1_✗Operationen,, p2_p3_0_2,p3_1_, p4_3,0"),
    (79u32, "p1_✗Planet_(10_und_oder_12),✗Planet_(10_und_oder_12),✗Operationen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Gleichheit_Freiheit_Ordnung,p3_2_2,p3_3_Meta-Systeme_(12),p3_4_, p4_3,0"),
    (80u32, "p1_✗Planet_(10_und_oder_12),✗Planet_(10_und_oder_12),✗Operationen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Gleichheit_Freiheit_Ordnung,p3_2_2,p3_3_Meta-Systeme_(12),p3_4_, p4_4,0"),
    (81u32, "p1_✗Operationen,, p2_p3_0_4,p3_1_, p4_4,0"),
    (82u32, "p1_✗Größenordnung,, p2_p3_0_Organisationen,p3_1_, p4_3,0"),
    (83u32, "p1_✗Größenordnung,, p2_p3_0_politische_Systeme,p3_1_, p4_3,0"),
    (84u32, "p1_✗Grundstrukturen,✗Galaxie,✗Universum,, p2_p3_0_analytische_Ontologie,p3_1_analytische_Ontologie,p3_2_analytische_Ontologie,p3_3_, p4_4,0"),
    (86u32, "p1_✗Operationen,, p2_p3_0_Halbierung,p3_1_, p4_3,0"),
    (87u32, "p1_✗Religionen,, p2_p3_0_Sternpolygon_vs_gleichförmiges,p3_1_, p4_3,1,0"),
    (88u32, "p1_✗Menschliches,✗Eigenschaften_n,, p2_p3_0_Sinn_des_Lebens,p3_1_Sinn_und_Zweck_des_Lebens,p3_2_, p4_3,0"),
    (89u32, "p1_✗Wirtschaft,, p2_p3_0_Maschinen,p3_1_, p4_3,0"),
    (90u32, "p1_✗Galaxie,, p2_p3_0_Offenbarung_des_Johannes,p3_1_, p4_3,0"),
    (91u32, "p1_✗Galaxie,, p2_p3_0_chinesisches_Horoskop,p3_1_, p4_3,0"),
    (92u32, "p1_✗Operationen,, p2_p3_0_3,p3_1_, p4_3,0"),
    (93u32, "p1_✗Operationen,, p2_p3_0_3,p3_1_, p4_4,0"),
    (94u32, "p1_✗Operationen,, p2_p3_0_9,p3_1_, p4_4,0"),
    (95u32, "p1_✗Inkrementieren,, p2_p3_0_,p3_1_, p4_3,0"),
    (96u32, "p1_✗Operationen,, p2_p3_0_5,p3_1_, p4_3,0"),
    (99u32, "p1_✗Wirtschaft,, p2_p3_0_Organisationsform,p3_1_, p4_3,0"),
    (100u32, "p1_✗Pro_Contra,, p2_p3_0_Gegenteil,p3_1_, p4_3,4,0,5"),
    (101u32, "p1_✗Pro_Contra,, p2_p3_0_Gegenteil,p3_1_, p4_3,4,0,5"),
    (102u32, "p1_✗Pro_Contra,, p2_p3_0_Harmonie,p3_1_, p4_3,4,0,5"),
    (103u32, "p1_✗Pro_Contra,, p2_p3_0_Harmonie,p3_1_, p4_3,4,0,5"),
    (104u32, "p1_✗Operationen,, p2_p3_0_4,p3_1_, p4_4,0"),
    (105u32, "p1_✗Menschliches,, p2_p3_0_Gefühle,p3_1_, p4_3,0"),
    (106u32, "p1_✗Bedeutung,, p2_p3_0_Konjunktiv_Wurzelbildung,p3_1_, p4_3,0"),
    (107u32, "p1_✗Planet_(10_und_oder_12),✗Bedeutung,, p2_p3_0_Mechanismen,p3_1_Mechanismen_der_Züchtung,p3_2_, p4_4,0,5"),
    (108u32, "p1_✗Bedeutung,, p2_p3_0_Mechanismen_der_Züchtung,p3_1_, p4_3,0"),
    (109u32, "p1_✗Wirtschaft,✗Bedeutung,, p2_p3_0_BWL,p3_1_Mechanismen_der_Züchtung,p3_2_, p4_3,0"),
    (112u32, "p1_✗Eigenschaften_n,, p2_p3_0_Weisheit_etc,p3_1_, p4_3,0"),
    (113u32, "p1_✗Wirtschaft,, p2_p3_0_Pflanzen,p3_1_, p4_3,0"),
    (114u32, "p1_✗Pro_Contra,, p2_p3_0_Hilfe_erhalten,p3_1_, p4_3,4,0,5"),
    (115u32, "p1_✗Pro_Contra,, p2_p3_0_Helfen,p3_1_, p4_3,4,0,5"),
    (116u32, "p1_✗Pro_Contra,, p2_p3_0_Gegenposition,p3_1_, p4_3,4,0,5"),
    (117u32, "p1_✗Pro_Contra,, p2_p3_0_pro_nutzen,p3_1_, p4_3,4,0,5"),
    (118u32, "p1_✗Bedeutung,, p2_p3_0_Vorzeichen,p3_1_, p4_3,0"),
    (119u32, "p1_✗Bedeutung,, p2_p3_0_Vorzeichen,p3_1_, p4_3,0"),
    (120u32, "p1_✗Pro_Contra,, p2_p3_0_nervig,p3_1_, p4_3,4,0,5"),
    (123u32, "p1_✗Pro_Contra,, p2_p3_0_nicht_miteinander_auskommen,p3_1_, p4_3,4,0,5"),
    (124u32, "p1_✗Pro_Contra,, p2_p3_0_nicht_dagegen,p3_1_, p4_3,4,0,5"),
    (125u32, "p1_✗Pro_Contra,, p2_p3_0_kein_Gegenteil,p3_1_, p4_3,4,0,5"),
    (126u32, "p1_✗Pro_Contra,, p2_p3_0_nicht_dafür,p3_1_, p4_3,4,0,5"),
    (127u32, "p1_✗Pro_Contra,, p2_p3_0_Hilfe_nicht_gebrauchen,p3_1_, p4_3,4,0,5"),
    (128u32, "p1_✗Pro_Contra,, p2_p3_0_nicht_helfen_können,p3_1_, p4_3,4,0,5"),
    (129u32, "p1_✗Pro_Contra,, p2_p3_0_nicht_abgeneigt,p3_1_, p4_3,4,0,5"),
    (130u32, "p1_✗Pro_Contra,, p2_p3_0_unmotivierbar,p3_1_, p4_3,4,0,5"),
    (131u32, "p1_✗Universum,✗Grundstrukturen,✗Multiversum,✗Grundstrukturen,, p2_p3_0_Reziproke_von_Transzendentalien,p3_1_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_2_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_3_Reziprokes,p3_4_, p4_4,1"),
    (132u32, "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Ordnung_und_Filterung_12_und_1pro12,p3_3_, p4_4,0,5"),
    (135u32, "p1_✗Menschliches,, p2_p3_0_Wirkung,p3_1_, p4_3,4,0"),
    (136u32, "p1_✗Menschliches,✗Eigenschaften_n,, p2_p3_0_Egoismus,p3_1_Egoismus_vs_Altruismus,p3_2_, p4_3,0"),
    (137u32, "p1_✗Pro_Contra,, p2_p3_0_Gegenspieler,p3_1_, p4_3,4,0,5"),
    (138u32, "p1_✗Universum,✗Menschliches,, p2_p3_0_Gegentranszendentalien,p3_1_Gegentranszendentalien,p3_2_, p4_4,0"),
    (139u32, "p1_✗Menschliches,, p2_p3_0_Gegentranszendentalien,p3_1_, p4_3,0"),
    (140u32, "p1_✗Pro_Contra,, p2_p3_0_ergibt_Sinn,p3_1_, p4_3,4,0,5"),
    (141u32, "p1_✗Pro_Contra,, p2_p3_0_Vorteile,p3_1_, p4_3,4,0,5"),
    (142u32, "p1_✗Pro_Contra,, p2_p3_0_Veränderung,p3_1_, p4_3,4,0,5"),
    (143u32, "p1_✗Pro_Contra,, p2_p3_0_bändigen_kontrollieren,p3_1_, p4_3,4,0,5"),
    (144u32, "p1_✗Pro_Contra,, p2_p3_0_vereinen,p3_1_, p4_3,4,0,5"),
    (145u32, "p1_✗Universum,✗Galaxie,✗Operationen,, p2_p3_0_Kugeln_Kreise,p3_1_Kugeln_Kreise,p3_2_4,p3_3_, p4_3,4,0"),
    (146u32, "p1_✗Menschliches,, p2_p3_0_Denkweise_von_Lebewesen,p3_1_, p4_3,0"),
    (147u32, "p1_✗Menschliches,, p2_p3_0_Intelligenzprobleme,p3_1_, p4_3,0"),
    (148u32, "p1_✗Galaxie,, p2_p3_0_Modallogik,p3_1_, p4_3,1"),
    (149u32, "p1_✗Galaxie,✗Menschliches,, p2_p3_0_Transzendentalien_innen_außen,p3_1_Motive,p3_2_, p4_3,4,0"),
    (150u32, "p1_✗Universum,, p2_p3_0_Systemsachen,p3_1_, p4_3,4,0"),
    (153u32, "p1_✗Menschliches,, p2_p3_0_Manipulation,p3_1_, p4_3,0"),
    (154u32, "p1_✗Bedeutung,, p2_p3_0_Gestirn,p3_1_, p4_3,1,0"),
    (155u32, "p1_✗Inkrementieren,, p2_p3_0_um1,p3_1_, p4_3,4,0"),
    (156u32, "p1_✗Inkrementieren,, p2_p3_0_um2,p3_1_, p4_3,4,0"),
    (157u32, "p1_✗Inkrementieren,, p2_p3_0_um3,p3_1_, p4_3,4,0"),
    (158u32, "p1_✗Operationen,, p2_p3_0_Multiplikation,p3_1_, p4_4,0"),
    (165u32, "p1_✗Universum,✗Inkrementieren,, p2_p3_0_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_1_warum_Transzendentalie_=_Strukturgroesse_=_Charakter,p3_2_, p4_3,4,0"),
    (166u32, "p1_✗Universum,✗Inkrementieren,, p2_p3_0_warum_Transzendentalie_=_Komplexität_von_Michael_Commons,p3_1_warum_Transzendentalie_=_Komplexität_von_Michael_Commons,p3_2_, p4_3,4,0"),
    (167u32, "p1_✗Menschliches,, p2_p3_0_Motive,p3_1_, p4_3,1,0"),
    (168u32, "p1_✗Menschliches,, p2_p3_0_Motive,p3_1_, p4_3,1,0"),
    (169u32, "p1_✗Bedeutung,, p2_p3_0_Zählungen,p3_1_, p4_4,0"),
    (170u32, "p1_✗Menschliches,, p2_p3_0_Anführer,p3_1_, p4_3,0"),
    (183u32, "p1_✗Wichtigstes_zum_verstehen,✗Menschliches,✗Wirtschaft,, p2_p3_0_Zweitwichtigste,p3_1_Fachgebiete,p3_2_Fachgebiete,p3_3_, p4_3,0"),
    (188u32, "p1_✗Bedeutung,, p2_p3_0_Zählungen,p3_1_, p4_3,4,0"),
    (189u32, "p1_✗Menschliches,✗Eigenschaften_n,, p2_p3_0_Sinn_des_Lebens,p3_1_Sinn_und_Zweck_des_Lebens,p3_2_, p4_3,0"),
    (190u32, "p1_✗Universum,, p2_p3_0_keine_Nur-Paradigma-Religionen,p3_1_, p4_4,0"),
    (191u32, "p1_✗Universum,, p2_p3_0_keine_Nur-Paradigma-Religionen,p3_1_, p4_4,0"),
    (196u32, "p1_✗Universum,, p2_p3_0_keine_Nur-Paradigma-Religionen,p3_1_, p4_4,0"),
    (197u32, "p1_✗Grundstrukturen,✗Größenordnung,, p2_p3_0_Strukturgrösse,p3_1_Strukturgrösse,p3_2_, p4_3,5,1,4"),
    (198u32, "p1_✗Universum,, p2_p3_0_Transzendentalien,p3_1_, p4_4,0"),
    (201u32, "p1_✗Universum,, p2_p3_0_Reziproke_von_Transzendentalien,p3_1_, p4_4,1"),
    (202u32, "p1_✗Universum,✗Menschliches,, p2_p3_0_Gegentranszendentalien,p3_1_Gegentranszendentalien,p3_2_, p4_4,0"),
    (203u32, "p1_✗Grundstrukturen,✗Universum,✗Multiversum,, p2_p3_0_Model_of_Hierarchical_Complexity,p3_1_Model_of_Hierarchical_Complexity,p3_2_Model_of_Hierarchical_Complexity,p3_3_, p4_4,1"),
    (204u32, "p1_✗Universum,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Kategorie,p3_1_Reziprokes,p3_2_Reflektion_und_Kategorien_(1/15),p3_3_, p4_4,0,5"),
    (205u32, "p1_✗Universum,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Kategorie,p3_1_Reziprokes,p3_2_Reflektion_und_Kategorien_(1/15),p3_3_, p4_1,4,5"),
    (207u32, "p1_✗Wichtigstes_zum_gedanklich_einordnen,✗Religionen,, p2_p3_0_Wichtigste,p3_1_der_Tierkreiszeichen,p3_2_, p4_3,0"),
    (208u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Liebe,p3_1_Liebe_(7),p3_2_, p4_0,5"),
    (209u32, "p1_✗Bedeutung,, p2_p3_0_in_ReTa,p3_1_, p4_4,0"),
    (210u32, "p1_✗Bedeutung,, p2_p3_0_in_ReTa,p3_1_, p4_4,0"),
    (213u32, "p1_✗Planet_(10_und_oder_12),, p2_p3_0_Komplexität,p3_1_, p4_4,0,5"),
    (214u32, "p1_✗Planet_(10_und_oder_12),, p2_p3_0_Intelligenz,p3_1_, p4_4,0,5"),
    (215u32, "p1_✗Menschliches,, p2_p3_0_Moral,p3_1_, p4_3,0"),
    (216u32, "p1_✗Menschliches,, p2_p3_0_Moral,p3_1_, p4_3,4,1,0"),
    (217u32, "p1_✗Religionen,, p2_p3_0_Hinduismus,p3_1_, p4_3,0"),
    (218u32, "p1_✗Universum,✗Galaxie,, p2_p3_0_Raum-Missionen,p3_1_Raum-Missionen,p3_2_, p4_3,4,0"),
    (219u32, "p1_✗Inkrementieren,✗Universum,✗Teilchen-Meta-Physik,, p2_p3_0_Teilchen-Meta-Physik,p3_1_Teilchen-Meta-Physik,p3_2_das_Universelle_(15),p3_3_, p4_4,0"),
    (220u32, "p1_✗Eigenschaften_n,, p2_p3_0_ähnlich,p3_1_, p4_3,4,0"),
    (221u32, "p1_✗Grundstrukturen,, p2_p3_0_Liebe_(7),p3_1_, p4_"),
    (222u32, "p1_✗Pro_Contra,, p2_p3_0_Gegenteil,p3_1_, p4_3,4,0,5"),
    (223u32, "p1_✗Inkrementieren,✗Teilchen-Meta-Physik,, p2_p3_0_Teilchen-Meta-Physik,p3_1_die_Galaxie_Unterbereiche_(13),p3_2_, p4_4,0"),
    (226u32, "p1_✗Primvielfache,, p2_p3_0_Rahmen-Bedingungen,p3_1_, p4_3,4,0"),
    (229u32, "p1_✗Grundstrukturen,✗Multiversum,✗Grundstrukturen,✗Multiversum,✗Menschliches,✗Menschliches,, p2_p3_0_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_1_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_2_Geist_(15),p3_3_Geist_(15),p3_4_Motive,p3_5_Bewusstsein_und_Wahrnehmung,p3_6_, p4_4,0"),
    (230u32, "p1_✗Menschliches,✗Menschliches,, p2_p3_0_Gefühle,p3_1_Motive,p3_2_, p4_4,0"),
    (231u32, "p1_✗Grundstrukturen,✗Grundstrukturen,✗Multiversum,✗Menschliches,, p2_p3_0_Reziprokes,p3_1_Geist_(15),p3_2_Geist_(15),p3_3_Bewusstsein_und_Wahrnehmung,p3_4_, p4_4,1"),
    (232u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Meta-Systeme_(12),p3_2_, p4_0,5"),
    (233u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,, p2_p3_0_Wirklichkeiten_(10),p3_1_Wirklichkeiten_Wahrheit_Wahrnehmung_(10),p3_2_, p4_0,5"),
    (234u32, "p1_✗Grundstrukturen,, p2_p3_0_Modus_und_Sein_(8),p3_1_, p4_0,5"),
    (235u32, "p1_✗Menschliches,, p2_p3_0_(politische)_Richtungen_(7),p3_1_, p4_4,0,5"),
    (240u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Ansichten_Standpunkte_(18_17),p3_1_Ansichten_Standpunkte_(18_17),p3_2_, p4_4,0,5"),
    (241u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gesellschaftsschicht,p3_1_Klassen_(20),p3_2_, p4_3,0,5"),
    (242u32, "p1_✗Universum,✗Grundstrukturen,✗Grundstrukturen,✗Multiversum,, p2_p3_0_Geist__(15),p3_1_nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15),p3_2_Geist_(15),p3_3_Geist_(15),p3_4_, p4_4,0"),
    (243u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gefühle,p3_1_Gefühle_(7),p3_2_, p4_0,5"),
    (249u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Gedanken_sind_Positionen_(17),p3_1_Gedanken_sind_Positionen_(17),p3_2_, p4_0,5"),
    (250u32, "p1_✗Grundstrukturen,, p2_p3_0_Konkreta_und_Focus_(2),p3_1_, p4_0,5"),
    (251u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Impulse_(5),p3_1_Errungenschaften,p3_2_, p4_0,5"),
    (252u32, "p1_✗Grundstrukturen,, p2_p3_0_Verbundenheiten_(18),p3_1_, p4_0,5"),
    (253u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Konkreta_und_Focus_(2),p3_1_Impulse_(5),p3_2_, p4_0,5"),
    (254u32, "p1_✗Grundstrukturen,, p2_p3_0_Triebe_und_Bedürfnisse_(6),p3_1_, p4_0,5"),
    (255u32, "p1_✗Grundstrukturen,, p2_p3_0_Lust_(9),p3_1_, p4_0,5"),
    (256u32, "p1_✗Grundstrukturen,, p2_p3_0_Reflexe_(3),p3_1_, p4_0,5"),
    (257u32, "p1_✗Grundstrukturen,✗Grundstrukturen,✗Menschliches,, p2_p3_0_Reziprokes,p3_1_Impulse_(5),p3_2_Errungenschaften,p3_3_, p4_1,5"),
    (260u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_10_ist_Wirklichkeit_erkennen,p3_1_, p4_0,5"),
    (261u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_7_ist_Selbstlosigkeit,p3_1_, p4_0,5"),
    (262u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_6_ist_Vorteilsmaximierung,p3_1_, p4_0,5"),
    (263u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_17_ist_zu_meinen,p3_1_, p4_0,5"),
    (264u32, "p1_✗Grundstrukturen,, p2_p3_0_Funktionen_Vorstellungen_(16),p3_1_, p4_4,0,5"),
    (265u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,✗Menschliches,, p2_p3_0_Wirklichkeiten_(10),p3_1_Wirklichkeiten_Wahrheit_Wahrnehmung_(10),p3_2_Bewusstsein_und_Wahrnehmung,p3_3_, p4_0,5"),
    (266u32, "p1_✗Grundstrukturen,, p2_p3_0_Zeit_(4)_als_Wirklichkeit,p3_1_, p4_0,5"),
    (267u32, "p1_✗Grundstrukturen,, p2_p3_0_Zeit_(4)_als_Wirklichkeit,p3_1_, p4_0,5"),
    (268u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,, p2_p3_0_Wirklichkeiten_(10),p3_1_Wirklichkeiten_Wahrheit_Wahrnehmung_(10),p3_2_, p4_0,5"),
    (269u32, "p1_✗Grundstrukturen,, p2_p3_0_Konkreta_und_Focus_(2),p3_1_, p4_0,5"),
    (270u32, "p1_✗Grundstrukturen,, p2_p3_0_Achtung_(4),p3_1_, p4_0,5"),
    (271u32, "p1_✗Grundstrukturen,, p2_p3_0_Ziele_(19),p3_1_, p4_0,5"),
    (272u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_1/8,p3_1_, p4_0,5"),
    (273u32, "p1_✗Grundstrukturen,✗Grundstrukturen,✗Multiversum,, p2_p3_0_Reziprokes,p3_1_Geist_(15),p3_2_Geist_(15),p3_3_, p4_4,1"),
    (276u32, "p1_✗Menschliches,, p2_p3_0_Gedanken_sind_Positionen_(17),p3_1_, p4_0,5"),
    (281u32, "p1_✗Universum,✗Grundstrukturen,✗Grundstrukturen,✗Menschliches,, p2_p3_0_Kategorie,p3_1_Reziprokes,p3_2_Reflektion_und_Kategorien_(1/15),p3_3_Bewusstsein_und_Wahrnehmung,p3_4_, p4_0,5"),
    (282u32, "p1_✗Grundstrukturen,, p2_p3_0_Bewusstheit_statt_Bewusstsein_(1),p3_1_, p4_0,5"),
    (283u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gefühle,p3_1_Gefühle_(7),p3_2_, p4_0,5"),
    (284u32, "p1_✗Menschliches,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Gefühle,p3_1_Reziprokes,p3_2_Gefühle_(7),p3_3_, p4_1,5"),
    (285u32, "p1_✗Menschliches,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Gefühle,p3_1_Reziprokes,p3_2_Gefühle_(7),p3_3_, p4_1,5"),
    (286u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gefühle,p3_1_Gefühle_(7),p3_2_, p4_0,5"),
    (287u32, "p1_✗Grundstrukturen,, p2_p3_0_Energie_und_universelle_Eigenschaften_(30),p3_1_, p4_0,5"),
    (288u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Meta-Systeme_(12),p3_2_, p4_0,5"),
    (289u32, "p1_✗Grundstrukturen,, p2_p3_0_Klassen_(20),p3_1_, p4_0,5"),
    (290u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (293u32, "p1_✗Grundstrukturen,, p2_p3_0_Energie_und_universelle_Eigenschaften_(30),p3_1_, p4_0,5"),
    (294u32, "p1_✗Grundstrukturen,, p2_p3_0_Empathie_(37),p3_1_, p4_0,5"),
    (295u32, "p1_✗Grundstrukturen,, p2_p3_0_Garben_und_Verhalten_nachfühlen(31),p3_1_, p4_0,5"),
    (296u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (297u32, "p1_✗Grundstrukturen,✗Grundstrukturen,✗Multiversum,, p2_p3_0_nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15),p3_1_Geist_(15),p3_2_Geist_(15),p3_3_, p4_4,0"),
    (298u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_1/6_ist_Reinigung_und_Klarheit,p3_1_, p4_0,5"),
    (299u32, "p1_✗Grundstrukturen,, p2_p3_0_Verbundenheiten_(18),p3_1_, p4_0,5"),
    (300u32, "p1_✗Grundstrukturen,, p2_p3_0_Verbundenheiten_(18),p3_1_, p4_0,5"),
    (301u32, "p1_✗Grundstrukturen,, p2_p3_0_Verhalten_(11),p3_1_, p4_0,5"),
    (302u32, "p1_✗Grundstrukturen,, p2_p3_0_Verhalten_(11),p3_1_, p4_0,5"),
    (303u32, "p1_✗Galaxie,, p2_p3_0_Thomasevangelium,p3_1_, p4_3,0"),
    (304u32, "p1_✗Grundstrukturen,✗Multiversum,✗Menschliches,, p2_p3_0_Geist_(15),p3_1_Geist_(15),p3_2_Bewusstsein_und_Wahrnehmung,p3_3_, p4_4,0"),
    (305u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gefühle,p3_1_Gefühle_(7),p3_2_, p4_0,5"),
    (306u32, "p1_✗Grundstrukturen,, p2_p3_0_Bedeutung_(10),p3_1_, p4_0,5"),
    (307u32, "p1_✗Inkrementieren,✗Teilchen-Meta-Physik,, p2_p3_0_Teilchen-Meta-Physik,p3_1_die_Galaxie_Unterbereiche_(13),p3_2_, p4_3,0"),
    (308u32, "p1_✗Inkrementieren,✗Universum,✗Teilchen-Meta-Physik,, p2_p3_0_Teilchen-Meta-Physik,p3_1_Teilchen-Meta-Physik,p3_2_das_Universelle_(15),p3_3_, p4_4,0"),
    (309u32, "p1_✗Grundstrukturen,, p2_p3_0_Themen_(6),p3_1_, p4_0,5"),
    (310u32, "p1_✗Grundstrukturen,, p2_p3_0_Optimierung_(10),p3_1_, p4_0,5"),
    (311u32, "p1_✗Grundstrukturen,, p2_p3_0_Attraktionen_(36),p3_1_, p4_0,5"),
    (312u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_16_ist_zu_genügen,p3_1_, p4_0,5"),
    (313u32, "p1_✗Größenordnung,✗Licht,, p2_p3_0_Licht,p3_1_,p3_2_, p4_3,5,1,4"),
    (314u32, "p1_✗Menschliches,, p2_p3_0_Mensch-zu-Tier,p3_1_, p4_4,0,5"),
    (315u32, "p1_✗Operationen,, p2_p3_0_3,p3_1_, p4_3,0"),
    (316u32, "p1_✗Operationen,, p2_p3_0_3,p3_1_, p4_3,0"),
    (317u32, "p1_✗Grundstrukturen,, p2_p3_0_Gedanken_sind_Positionen_(17),p3_1_, p4_0,5"),
    (318u32, "p1_✗Galaxie,✗Universum,, p2_p3_0_Hochzüchten,p3_1_Hochzüchten,p3_2_, p4_3,4,0,5"),
    (319u32, "p1_✗Galaxie,✗Universum,, p2_p3_0_Hochzüchten,p3_1_Hochzüchten,p3_2_, p4_3,5,1,4"),
    (320u32, "p1_✗Universum,, p2_p3_0_sowas_wie_Kombinieren_Verknüpfen,p3_1_, p4_4,0"),
    (321u32, "p1_✗Grundstrukturen,, p2_p3_0_Koalitionen_(10),p3_1_, p4_0,5"),
    (322u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,, p2_p3_0_Wirklichkeiten_(10),p3_1_Wirklichkeiten_Wahrheit_Wahrnehmung_(10),p3_2_, p4_0,5"),
    (323u32, "p1_✗Grundstrukturen,, p2_p3_0_Gedanken_sind_Positionen_(17),p3_1_, p4_0,5"),
    (324u32, "p1_✗Planet_(10_und_oder_12),, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_, p4_0,5"),
    (325u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (326u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Reziprokes,p3_1_Stimmungen_Kombinationen_(14),p3_2_, p4_1,5"),
    (327u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Reziprokes,p3_1_Stimmungen_Kombinationen_(14),p3_2_, p4_1,5"),
    (328u32, "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Reziprokes,p3_3_Ordnung_und_Filterung_12_und_1pro12,p3_4_, p4_3,5,1,4"),
    (329u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Prinzipien(1/8),p3_1_Reziprokes,p3_2_, p4_3,4,1"),
    (330u32, "p1_✗Menschliches,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Liebe,p3_1_Reziprokes,p3_2_Liebe_(7),p3_3_, p4_1,5"),
    (331u32, "p1_✗Planet_(10_und_oder_12),✗Operationen,✗Menschliches,✗Grundstrukturen,✗Grundstrukturen,✗Eigenschaften_1/n,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_2,p3_2_Gleichheit_Freiheit,p3_3_Reziprokes,p3_4_Ordnung_und_Filterung_12_und_1pro12,p3_5_Filterart_Widrigkeit,p3_6_, p4_3,5,1,4"),
    (332u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Bestrebungen(1/5),p3_1_Reziprokes,p3_2_, p4_1,5"),
    (333u32, "p1_✗Inkrementieren,✗Teilchen-Meta-Physik,, p2_p3_0_Teilchen-Meta-Physik,p3_1_das_Gute_die_Richtung_(7),p3_2_, p4_0,5"),
    (334u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Reziprokes,p3_2_Meta-Systeme_(12),p3_3_, p4_1,5"),
    (335u32, "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,✗Grundstrukturen,✗Eigenschaften_1/n,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Reziprokes,p3_3_Ordnung_und_Filterung_12_und_1pro12,p3_4_Filterart_Widrigkeit,p3_5_, p4_3,5,1,4"),
    (336u32, "p1_✗Grundstrukturen,, p2_p3_0_Verbundenheiten_(18),p3_1_, p4_0,5"),
    (337u32, "p1_✗Grundstrukturen,, p2_p3_0_Modus_und_Sein_(8),p3_1_, p4_0,5"),
    (338u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Bedingung_und_Auslöser_(1/3),p3_1_Reziprokes,p3_2_, p4_0,5"),
    (339u32, "p1_✗Grundstrukturen,, p2_p3_0_relativer_Zeit-Betrag_(15_10_4_18_6),p3_1_, p4_0,5"),
    (340u32, "p1_✗Grundstrukturen,, p2_p3_0_Zahlenvergleich_(15_18_6),p3_1_, p4_0,5"),
    (341u32, "p1_✗Grundstrukturen,, p2_p3_0_Impulse_(5),p3_1_, p4_0,5"),
    (342u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Wirklichkeiten_Wahrheit_Wahrnehmung_(10),p3_1_Bewusstsein_und_Wahrnehmung,p3_2_, p4_1,5"),
    (343u32, "p1_✗Grundstrukturen,, p2_p3_0_Leidenschaften_(21),p3_1_, p4_0,5"),
    (344u32, "p1_✗Grundstrukturen,, p2_p3_0_Erwartungshaltungen_(26),p3_1_, p4_0,5"),
    (345u32, "p1_✗Grundstrukturen,, p2_p3_0_Funktionen_Vorstellungen_(16),p3_1_, p4_0,5"),
    (346u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Ansichten_Standpunkte_(18_17),p3_1_Ansichten_Standpunkte_(18_17),p3_2_, p4_1,4,5"),
    (347u32, "p1_✗Grundstrukturen,, p2_p3_0_Extremalien_(19),p3_1_, p4_0,5"),
    (348u32, "p1_✗Grundstrukturen,, p2_p3_0_Existenzialien_(3),p3_1_, p4_0,5"),
    (349u32, "p1_✗Grundstrukturen,, p2_p3_0_universeller_Komperativ_(18→15),p3_1_, p4_0,5"),
    (350u32, "p1_✗Grundstrukturen,, p2_p3_0_Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n),p3_1_, p4_0,5"),
    (351u32, "p1_✗Universum,, p2_p3_0_Programmier-Paradigmen,p3_1_, p4_4,0,5"),
    (352u32, "p1_✗Grundstrukturen,, p2_p3_0_Extremalien_(19),p3_1_, p4_1,5"),
    (353u32, "p1_✗Grundstrukturen,, p2_p3_0_Sollen_Frage_Vorgehensweise_(1/13),p3_1_, p4_0,5"),
    (354u32, "p1_✗Grundstrukturen,, p2_p3_0_Sollen_Frage_Vorgehensweise_(1/13),p3_1_, p4_0,5"),
    (356u32, "p1_✗Grundstrukturen,, p2_p3_0_Fundament_(1/19),p3_1_, p4_0,5"),
    (357u32, "p1_✗Grundstrukturen,, p2_p3_0_abhängige_Verbundenheit_(90),p3_1_, p4_0,5"),
    (370u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_13_ist_Helfen,p3_1_, p4_3,0,5"),
    (377u32, "p1_✗Grundstrukturen,, p2_p3_0_Karte_Filter_und_Unterscheidung_(1/12),p3_1_, p4_0,5"),
    (378u32, "p1_✗Grundstrukturen,, p2_p3_0_Prinzipien(1/8),p3_1_, p4_1,5"),
    (379u32, "p1_✗Grundstrukturen,, p2_p3_0_Absicht_1/8,p3_1_, p4_3,1,5"),
    (382u32, "p1_✗Universum,, p2_p3_0_universelles_Recht,p3_1_, p4_4,0"),
    (383u32, "p1_✗Universum,, p2_p3_0_Universelles_Verhältnis_gleicher_Zahlen,p3_1_, p4_4,1"),
    (384u32, "p1_✗Grundstrukturen,, p2_p3_0_Maßnahmen_(39),p3_1_, p4_0,5"),
    (385u32, "p1_✗Universum,✗Grundstrukturen,, p2_p3_0_Strategie_Taktik_(15m8),p3_1_Modus_und_Sein_(8),p3_2_, p4_4,0,5"),
    (386u32, "p1_✗Universum,✗Bedeutung,✗Kontinuum,, p2_p3_0_Netzwerk,p3_1_Zählungen,p3_2_X,p3_3_, p4_4,0"),
    (387u32, "p1_✗Inkrementieren,✗Teilchen-Meta-Physik,✗Grundstrukturen,, p2_p3_0_Teilchen-Meta-Physik,p3_1_Raum_und_Dimensionen_(8),p3_2_Modus_und_Sein_(8),p3_3_, p4_4,0,5"),
    (388u32, "p1_✗Inkrementieren,✗Teilchen-Meta-Physik,✗Multiversum,✗Grundstrukturen,, p2_p3_0_Teilchen-Meta-Physik,p3_1_das_Multiverselle_(16),p3_2_Meta-Physik-Teilchen_(1),p3_3_Funktionen_Vorstellungen_(16),p3_4_, p4_0,5"),
    (389u32, "p1_✗Multiversum,, p2_p3_0_Multiversalien_(16),p3_1_, p4_0,5"),
    (390u32, "p1_✗Universum,✗Universum,✗Bedeutung,, p2_p3_0_Transzendentalien,p3_1_Netzwerk,p3_2_Zählungen,p3_3_, p4_4,0"),
    (391u32, "p1_✗Grundstrukturen,, p2_p3_0_Lust_(9),p3_1_, p4_0,5"),
    (392u32, "p1_✗Grundstrukturen,, p2_p3_0_Triebe_und_Bedürfnisse_(6),p3_1_, p4_1,5"),
    (393u32, "p1_✗Grundstrukturen,, p2_p3_0_Achtung_(4),p3_1_, p4_0,5"),
    (394u32, "p1_✗Grundstrukturen,, p2_p3_0_Klassen_(20),p3_1_, p4_3,0,5"),
    (395u32, "p1_✗Grundstrukturen,, p2_p3_0_Klassen_(20),p3_1_, p4_3,0,5"),
    (396u32, "p1_✗Grundstrukturen,, p2_p3_0_Triebe_und_Bedürfnisse_(6),p3_1_, p4_0,5"),
    (397u32, "p1_✗Grundstrukturen,, p2_p3_0_Triebe_und_Bedürfnisse_(6),p3_1_, p4_0,5"),
    (398u32, "p1_✗Grundstrukturen,, p2_p3_0_innere_Werte_1/6_der_Reinigung_und_Klarheit,p3_1_, p4_0,5"),
    (399u32, "p1_✗Grundstrukturen,, p2_p3_0_innere_Werte_1/6_der_Reinigung_und_Klarheit,p3_1_, p4_0,5"),
    (400u32, "p1_✗Grundstrukturen,, p2_p3_0_innere_Werte_1/6_der_Reinigung_und_Klarheit,p3_1_, p4_1,5"),
    (401u32, "p1_✗Grundstrukturen,, p2_p3_0_innere_Werte_1/6_der_Reinigung_und_Klarheit,p3_1_, p4_1,5"),
    (402u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (403u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (404u32, "p1_✗Grundstrukturen,, p2_p3_0_Rechnen,p3_1_, p4_0,5"),
    (405u32, "p1_✗Grundstrukturen,, p2_p3_0_Lebensbereiche_Problemklassen_(28),p3_1_, p4_0,5"),
    (406u32, "p1_✗Inkrementieren,✗Teilchen-Meta-Physik,✗Grundstrukturen,, p2_p3_0_Teilchen-Meta-Physik,p3_1_das_Galaktische_(14),p3_2_Stimmungen_Kombinationen_(14),p3_3_, p4_0,5"),
    (407u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (408u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_0,5"),
    (409u32, "p1_✗Universum,, p2_p3_0_künstliches_Leben_(15),p3_1_, p4_4,0"),
    (410u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Paradigmen_sind_Absichten_(13),p3_2_Meta-Systeme_(12),p3_3_, p4_0,5"),
    (411u32, "p1_✗Planet_(10_und_oder_12),✗Teilchen-Meta-Physik,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_die_Tugendsortierung_(13_mit_14),p3_2_Paradigmen_sind_Absichten_(13),p3_3_Meta-Systeme_(12),p3_4_, p4_3,0,5"),
    (412u32, "p1_✗Teilchen-Meta-Physik,, p2_p3_0_die_Galaxie_Unterbereiche_(13),p3_1_, p4_0,5"),
    (413u32, "p1_✗Grundstrukturen,, p2_p3_0_Verhalten_(11),p3_1_, p4_0,5"),
    (414u32, "p1_✗Grundstrukturen,, p2_p3_0_Bestrebungen(1/5),p3_1_, p4_0,5"),
    (415u32, "p1_✗Grundstrukturen,, p2_p3_0_Lebensbereiche_Problemklassen_(28),p3_1_, p4_0,5"),
    (416u32, "p1_✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Lebensbereiche_Problemklassen_(28),p3_1_Reziprokes,p3_2_, p4_1,5"),
    (417u32, "p1_✗Grundstrukturen,, p2_p3_0_Netzwerk,p3_1_, p4_0,5"),
    (418u32, "p1_✗Teilchen-Meta-Physik,✗Grundstrukturen,, p2_p3_0_das_Multiverselle_(16),p3_1_Funktionen_Vorstellungen_(16),p3_2_, p4_0,5"),
    (419u32, "p1_✗Grundstrukturen,, p2_p3_0_mathematisches_Design_(32),p3_1_, p4_0,5"),
    (420u32, "p1_✗Planet_(10_und_oder_12),✗Teilchen-Meta-Physik,, p2_p3_0_Wirklichkeiten_(10),p3_1_Wirklichkeiten_(10),p3_2_, p4_0,5"),
    (421u32, "p1_✗Grundstrukturen,, p2_p3_0_Kontroverse_(51),p3_1_, p4_0,5"),
    (422u32, "p1_✗Universum,, p2_p3_0_Software-Lizenzen_akademische_Grade,p3_1_, p4_3,4,0,5"),
    (423u32, "p1_✗Grundstrukturen,, p2_p3_0_Triebe_und_Bedürfnisse_(6),p3_1_, p4_0,5"),
    (424u32, "p1_✗Grundstrukturen,, p2_p3_0_Taetigkeiten,p3_1_, p4_3,0,5"),
    (425u32, "p1_✗Grundstrukturen,✗Größenordnung,✗Größenordnung,, p2_p3_0_Strukturgrösse,p3_1_Strukturgrösse,p3_2_Organisationen,p3_3_, p4_0,5"),
    (426u32, "p1_✗Universum,✗Grundstrukturen,✗Multiversum,, p2_p3_0_Geist__(15),p3_1_Geist_(15),p3_2_Geist_(15),p3_3_, p4_4,0"),
    (427u32, "p1_✗Grundstrukturen,, p2_p3_0_Wohlbefinden_(7mit6),p3_1_, p4_0,5"),
    (428u32, "p1_✗Grundstrukturen,, p2_p3_0_Wohlbefinden_(7mit6),p3_1_, p4_1,5"),
    (429u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Anführer_Arten_(7),p3_1_Anführer,p3_2_, p4_3,0"),
    (430u32, "p1_✗Grundstrukturen,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_, p4_"),
    (431u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (432u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (433u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (434u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (435u32, "p1_✗Multiversum,✗Kontinuum,, p2_p3_0_P,p3_1_P,p3_2_, p4_0,5"),
    (436u32, "p1_✗Grundstrukturen,✗Kontinuum,✗Kontinuum,, p2_p3_0_Netzwerk,p3_1_X,p3_2_R,p3_3_, p4_0,5"),
    (437u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (438u32, "p1_✗Multiversum,, p2_p3_0_Struktur-Wissenschaften_(10),p3_1_, p4_0,5"),
    (439u32, "p1_✗Multiversum,, p2_p3_0_Muster-Wissenschaften_(20),p3_1_, p4_0,5"),
    (440u32, "p1_✗Grundstrukturen,✗Wirtschaft,, p2_p3_0_System,p3_1_System,p3_2_, p4_3,1"),
    (441u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (442u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (443u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (444u32, "p1_✗Religionen,✗Galaxie,✗Menschliches,✗Universum,, p2_p3_0_Superkräfte,p3_1_Superkräfte,p3_2_Superkräfte,p3_3_Farben,p3_4_, p4_4,0"),
    (445u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (446u32, "p1_✗Kontinuum,, p2_p3_0_A,p3_1_, p4_0,5"),
    (447u32, "p1_✗Kontinuum,, p2_p3_0_B,p3_1_, p4_0,5"),
    (448u32, "p1_✗Kontinuum,, p2_p3_0_C,p3_1_, p4_0,5"),
    (449u32, "p1_✗Kontinuum,, p2_p3_0_D,p3_1_, p4_0,5"),
    (450u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (451u32, "p1_✗Kontinuum,, p2_p3_0_R,p3_1_, p4_0,5"),
    (452u32, "p1_✗Symbole,, p2_p3_0_Drei,p3_1_, p4_0,5"),
    (453u32, "p1_✗Symbole,, p2_p3_0_Vier,p3_1_, p4_0,5"),
    (454u32, "p1_✗Symbole,, p2_p3_0_Fünf,p3_1_, p4_0,5"),
    (455u32, "p1_✗Grundstrukturen,✗Wirtschaft,✗Grundstrukturen,✗Menschliches,, p2_p3_0_System,p3_1_System,p3_2_Anführer_Arten_(7),p3_3_Anführer,p3_4_, p4_3,1"),
    (456u32, "p1_✗Symbole,✗Symbole,, p2_p3_0_Zehn,p3_1_Zwölf,p3_2_, p4_0,5"),
    (457u32, "p1_✗Symbole,✗Symbole,, p2_p3_0_Sechs,p3_1_Sieben,p3_2_, p4_0,5"),
    (458u32, "p1_✗Symbole,, p2_p3_0_Acht,p3_1_, p4_0,5"),
    (459u32, "p1_✗Symbole,, p2_p3_0_Neun,p3_1_, p4_0,5"),
    (460u32, "p1_✗Symbole,, p2_p3_0_Drei,p3_1_, p4_0,5"),
    (461u32, "p1_✗Menschliches,, p2_p3_0_Formationen,p3_1_, p4_0,5"),
    (462u32, "p1_✗Universum,, p2_p3_0_Zahlenarten,p3_1_, p4_4,0"),
    (463u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Bestrafung,p3_1_Bestrafung,p3_2_, p4_3,4,0"),
    (464u32, "p1_✗Menschliches,, p2_p3_0_weniger_am_Menschen,p3_1_, p4_3,4,0"),
    (465u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Erlösung,p3_1_Erlösung,p3_2_, p4_3,4,0"),
    (466u32, "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gewalt,p3_1_Gewalt,p3_2_, p4_3,4,0"),
    (467u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (468u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (469u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (470u32, "p1_✗Galaxie,, p2_p3_0_Lebewesen_Galaxie_am_Besten,p3_1_, p4_3,0"),
    (471u32, "p1_✗Galaxie,, p2_p3_0_Lebewesen_Galaxie_am_Besten,p3_1_, p4_3,0"),
    (472u32, "p1_✗Multiversum,, p2_p3_0_Raumzeit_Anordnung_mathematisch_universell,p3_1_, p4_3,0"),
    (473u32, "p1_✗Galaxie,, p2_p3_0_Lebewesen_Galaxie_am_Besten,p3_1_, p4_4,0,5"),
    (474u32, "p1_✗Bedeutung,, p2_p3_0_in_ReTa,p3_1_, p4_4,0"),
    (475u32, "p1_✗Bedeutung,, p2_p3_0_in_ReTa,p3_1_, p4_4,0"),
    (476u32, "p1_✗Grundstrukturen,✗Wirtschaft,, p2_p3_0_System,p3_1_System,p3_2_, p4_3,0"),
    (477u32, "p1_✗Religionen,, p2_p3_0_der_Tierkreiszeichen,p3_1_, p4_3,0"),
    (478u32, "p1_✗Religionen,, p2_p3_0_der_Tierkreiszeichen,p3_1_, p4_3,0"),
    (479u32, "p1_✗Grundstrukturen,, p2_p3_0_Gewalt,p3_1_, p4_4,0"),
    (480u32, "p1_✗Grundstrukturen,, p2_p3_0_Wirklichkeiten_Wahrheit_Wahrnehmung_(10),p3_1_, p4_4,0"),
    (481u32, "p1_✗Grundstrukturen,, p2_p3_0_Anführer_Arten_(7),p3_1_, p4_4,0"),
    (482u32, "p1_✗Grundstrukturen,, p2_p3_0_Anführer_Arten_(7),p3_1_, p4_4,0"),
    (483u32, "p1_✗Planet_(10_und_oder_12),✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Model_of_Hierarchical_Complexity,p3_2_Meta-Systeme_(12),p3_3_, p4_"),
    (484u32, "p1_✗Multiversum,, p2_p3_0_Muster-Wissenschaften_(20),p3_1_, p4_1,4,5"),
    (485u32, "p1_✗Grundstrukturen,, p2_p3_0_Klassen_(20),p3_1_, p4_0,5"),
    (486u32, "p1_✗Kontinuum,, p2_p3_0_T,p3_1_, p4_0,5"),
    (487u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (488u32, "p1_✗Kontinuum,, p2_p3_0_Q,p3_1_, p4_0,5"),
    (489u32, "p1_✗Universum,, p2_p3_0_Zusammenhang_Gehirn_Kosmos_Universum,p3_1_, p4_4,0,5"),
    (490u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Anführer_Arten_(7),p3_1_Anführer,p3_2_, p4_4,0,5"),
    (491u32, "p1_✗Grundstrukturen,✗Kontinuum,, p2_p3_0_Modus_und_Sein_(8),p3_1_H,p3_2_, p4_0,5"),
    (492u32, "p1_✗Grundstrukturen,✗Kontinuum,, p2_p3_0_Stimmungen_Kombinationen_(14),p3_1_N,p3_2_, p4_3,0,5"),
    (493u32, "p1_✗Grundstrukturen,✗Kontinuum,, p2_p3_0_Paradigmen_sind_Absichten_(13),p3_1_M,p3_2_, p4_3,0,5"),
    (494u32, "p1_✗Religionen,✗Galaxie,✗Menschliches,✗Grundstrukturen,, p2_p3_0_Superkräfte,p3_1_Superkräfte,p3_2_Superkräfte,p3_3_Paradigmen_sind_Absichten_(13),p3_4_, p4_0,5"),
    (495u32, "p1_✗Religionen,✗Menschliches,, p2_p3_0_Satan_Teufel,p3_1_Satan_Teufel,p3_2_, p4_3,4,0,5"),
    (496u32, "p1_✗Religionen,✗Galaxie,✗Menschliches,, p2_p3_0_Superkräfte,p3_1_Superkräfte,p3_2_Superkräfte,p3_3_, p4_0,5"),
    (497u32, "p1_✗Planet_(10_und_oder_12),✗Planet_(10_und_oder_12),✗Operationen,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Gleichheit_Freiheit_Ordnung,p3_2_2,p3_3_Anführer_Arten_(7),p3_4_Meta-Systeme_(12),p3_5_, p4_4,0,5"),
    (498u32, "p1_✗Planet_(10_und_oder_12),✗Planet_(10_und_oder_12),✗Operationen,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Gleichheit_Freiheit_Ordnung,p3_2_2,p3_3_Anführer_Arten_(7),p3_4_Meta-Systeme_(12),p3_5_, p4_4,0,5"),
    (499u32, "p1_✗Planet_(10_und_oder_12),✗Planet_(10_und_oder_12),✗Operationen,✗Grundstrukturen,✗Grundstrukturen,, p2_p3_0_Meta-Systeme_(12),p3_1_Gleichheit_Freiheit_Ordnung,p3_2_2,p3_3_Anführer_Arten_(7),p3_4_Meta-Systeme_(12),p3_5_, p4_1,4,5"),
    (500u32, "p1_✗Grundstrukturen,✗Multiversum,✗Universum,✗Menschliches,, p2_p3_0_Biologischer_Baum_(15),p3_1_Biologischer_Baum_(16_->_5),p3_2_Biologischer_Baum_(15),p3_3_Biologischer_Baum_(15),p3_4_, p4_4,0"),
    (501u32, "p1_✗Kontinuum,✗Multiversum,, p2_p3_0_P5,p3_1_P5,p3_2_, p4_0,5"),
    (502u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Anführer_Arten_(7),p3_1_Anführer,p3_2_, p4_0,5"),
    (503u32, "p1_✗Religionen,✗Religionen,✗Religionen,, p2_p3_0_Superkräfte,p3_1_Religions-Gründer-Typ,p3_2_Messias,p3_3_, p4_0,5"),
    (504u32, "p1_✗Kontinuum,, p2_p3_0_S,p3_1_, p4_0,5"),
    (506u32, "p1_✗Kontinuum,, p2_p3_0_L,p3_1_, p4_"),
    (507u32, "p1_✗Kontinuum,, p2_p3_0_Y,p3_1_, p4_0,5"),
    (508u32, "p1_✗Kontinuum,✗Kontinuum,, p2_p3_0_Kontinuen,p3_1_F,p3_2_, p4_0,5"),
    (509u32, "p1_✗Grundstrukturen,✗Menschliches,, p2_p3_0_Anführer_Arten_(7),p3_1_Anführer,p3_2_, p4_4,0,5"),
    (510u32, "p1_✗Kontinuum,, p2_p3_0_Y,p3_1_, p4_0,5"),
    (511u32, "p1_✗Kontinuum,, p2_p3_0_E,p3_1_, p4_0,5"),
    (512u32, "p1_✗Multiversum,✗Grundstrukturen,✗Universum,, p2_p3_0_Teilchen_anderes_Universum,p3_1_Teilchen_anderes_Universum,p3_2_Teilchen_anderes_Universum,p3_3_, p4_4,0,5"),
    (513u32, "p1_✗Grundstrukturen,✗Wirtschaft,, p2_p3_0_System,p3_1_System,p3_2_, p4_4,0,5"),
    (514u32, "p1_✗Kontinuum,, p2_p3_0_J,p3_1_, p4_0,5"),
    (515u32, "p1_✗Kontinuum,, p2_p3_0_k,p3_1_, p4_0,5"),
    (516u32, "p1_✗Grundstrukturen,, p2_p3_0_Klassen_(20),p3_1_, p4_0,5"),
    (517u32, "p1_✗Kontinuum,, p2_p3_0_i,p3_1_, p4_0,5"),
    (518u32, "p1_✗Kontinuum,, p2_p3_0_G,p3_1_, p4_0,5"),
    (519u32, "p1_✗Universum,✗Menschliches,, p2_p3_0_Evolution_vs_Design_intelligent,p3_1_Evolution_vs_Design_intelligent,p3_2_, p4_0,5"),
];




#[derive(Clone, Copy)]
struct GeneratedFamily {
    command: &'static str,
    seed_pairs: &'static [(&'static str, &'static str)],
    ober_aliases: &'static [&'static str],
    unter_aliases: &'static [&'static str],
}

const GENERATED_FAMILIES: &[GeneratedFamily] = &[
    GeneratedFamily {
        command: "primzahlkreuzprocontra",
        seed_pairs: &[
            ("Universum", "Primzahlkreuz"),
            ("Bedeutung", "Primzahlkreuz"),
            ("Pro_Contra", "Primzahlkreuz"),
            ("Grundstrukturen", "Primzahlkreuz"),
        ],
        ober_aliases: &["procontra", "bedeutung", "universum", "grundstrukturen"],
        unter_aliases: &["primzahlkreuz", "primzahlkreuzprocontra", "nachvollziehen"],
    },
    GeneratedFamily {
        command: "lovepolygon",
        seed_pairs: &[("Menschliches", "Liebe"), ("Grundstrukturen", "Liebe")],
        ober_aliases: &["menschliches", "grundstrukturen"],
        unter_aliases: &["liebe", "ethik"],
    },
    GeneratedFamily {
        command: "gleichheitfreiheit",
        seed_pairs: &[
            ("Planet", "Gleichheit"),
            ("Menschliches", "Gleichheit"),
            ("Grundstrukturen", "Gleichheit"),
        ],
        ober_aliases: &["planet", "menschliches", "grundstrukturen"],
        unter_aliases: &["gleichheit", "freiheit", "ordnung", "ordnen", "filterung", "dominieren", "ungleichheit", "gleichheitfreiheit"],
    },
    GeneratedFamily {
        command: "geistemotionenergiematerietopologie",
        seed_pairs: &[
            ("Universum", "Geist"),
            ("Multiversum", "Geist"),
            ("Grundstrukturen", "Geist"),
        ],
        ober_aliases: &["universum", "multiversum", "grundstrukturen"],
        unter_aliases: &["geist", "bewusstsein", "emotion", "emotionen", "gefuehl", "gefuehle", "gefühl", "gefühle", "energie", "materie", "topologie"],
    },
    GeneratedFamily {
        command: "primcreativitytype",
        seed_pairs: &[
            ("Wichtigstes_zum_verstehen", "Gestirn"),
            ("Bedeutung", "Gestirn"),
        ],
        ober_aliases: &["bedeutung", "wichtigsteszumverstehen", "wichtigsteverstehen"],
        unter_aliases: &["gestirn", "sonne", "planet", "evolution", "intelligenz", "kreativ", "kreativitaet", "kreativität", "lernen", "erwerben"],
    },
    GeneratedFamily {
        command: "mondexponzierenlogarithmustyp",
        seed_pairs: &[("Wichtigstes_zum_verstehen", "Mond"), ("Bedeutung", "Mond")],
        ober_aliases: &["bedeutung", "wichtigsteszumverstehen", "wichtigsteverstehen"],
        unter_aliases: &["mond", "logarithmus", "exponieren", "exponential", "exponentiell"],
    },
    GeneratedFamily {
        command: "vervielfachezeile",
        seed_pairs: &[
            ("Wichtigstes_zum_verstehen", "Primzahlen"),
            ("Bedeutung", "Primzahlen"),
            ("Galaxie", "Offenbarung_des_Johannes"),
        ],
        ober_aliases: &["bedeutung", "wichtigsteszumverstehen", "wichtigsteverstehen", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"],
        unter_aliases: &["primzahlen", "vielfache", "multis", "multiplikationen", "offenbarung", "offenbarungdesjohannes", "johannes", "bibel"],
    },
];

fn matches_generated_family(family: &GeneratedFamily, ober: &str, unter: &str) -> bool {
    let ober_n = normalize_key(ober);
    let unter_n = normalize_key(unter);
    family.ober_aliases.iter().any(|a| normalize_key(a) == ober_n)
        && family.unter_aliases.iter().any(|a| normalize_key(a) == unter_n)
}

pub fn generated_seed_pairs() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for family in GENERATED_FAMILIES {
        for (ober, unter) in family.seed_pairs {
            out.push(((*ober).to_string(), (*unter).to_string()));
        }
    }
    out.push(("Modallogik".to_string(), "Modallogik".to_string()));
    out.sort();
    out.dedup();
    out
}

pub fn combination_seed_pairs() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for unter in [
        "tiere",
        "berufe",
        "kreativität",
        "liebe",
        "männer",
        "persönlichkeit",
        "religion",
        "motive",
        "emotionen",
        "personen",
        "wirtschaftssysteme",
        "eigentum",
    ] {
        out.push(("KombinationGalaxie".to_string(), unter.to_string()));
    }
    for unter in [
        "tiere",
        "berufe",
        "transzendentalien",
        "primzahlkreuz",
        "persönlichkeit",
        "religion",
        "motive",
        "ontologie",
        "personen",
        "mechanismen",
        "gegentranszendentalien",
        "maschinen",
        "geist",
        "bewusstsein",
    ] {
        out.push(("KombinationUniversum".to_string(), unter.to_string()));
    }
    out.sort();
    out.dedup();
    out
}

pub fn multiplication_seed_pairs() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for ober in ["primvielfache", "multiplikationen"] {
        for unter in [
            "motivgleichfoermig",
            "strukturgleichfoermig",
            "motivstern",
            "strukturstern",
            "motivgebrstern",
            "strukgebrstern",
            "motivgebrgleichf",
            "strukgebrgleichf",
        ] {
            out.push((ober.to_string(), unter.to_string()));
        }
    }
    out.sort();
    out.dedup();
    out
}

pub fn exact_columns_for_pair(ober: &str, unter: &str) -> Vec<u32> {
    let ober_n = normalize_key(ober);
    let unter_n = normalize_key(unter);
    let mut out = Vec::new();
    for decl in PY_DECLS {
        let main_match = decl.main_aliases.iter().any(|a| normalize_key(a) == ober_n);
        let sub_match = decl.sub_aliases.iter().any(|a| normalize_key(a) == unter_n);
        if main_match && sub_match {
            out.extend_from_slice(decl.columns);
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub fn fuzzy_columns_for_pair(ober: &str, unter: &str) -> Vec<u32> {
    let ober_n = normalize_key(ober);
    let unter_n = normalize_key(unter);
    let mut out = Vec::new();
    for decl in PY_DECLS {
        let main_match = decl.main_aliases.iter().any(|a| normalize_key(a) == ober_n);
        let sub_match = decl.sub_aliases.iter().any(|a| normalize_key(a).contains(&unter_n));
        if main_match && sub_match {
            out.extend_from_slice(decl.columns);
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub fn exact_meta_for_column(col: u32) -> Option<String> {
    exact_decl_meta_for_column(col).map(|meta| meta.render())
}

fn legacy_exact_decl_meta_for_column(col: u32) -> Option<HtmlDeclMeta> {
    for (c, meta) in EXACT_HTML_META {
        if *c == col {
            return HtmlDeclMeta::parse(meta);
        }
    }
    None
}

pub fn exact_decl_meta_for_column(col: u32) -> Option<HtmlDeclMeta> {
    if let Some(meta) = typed_exact_decl_for_column(col) {
        return Some(meta);
    }
    if is_typed_exact_decl_column(col) {
        panic!("typed exact decl missing for column {}", col);
    }
    legacy_exact_decl_meta_for_column(col)
}

pub fn all_exact_decl_meta() -> Vec<(u32, HtmlDeclMeta)> {
    let mut out = all_typed_exact_decls();
    for (col, meta) in EXACT_HTML_META {
        if is_typed_exact_decl_column(*col) {
            continue;
        }
        if let Some(parsed) = HtmlDeclMeta::parse(meta) {
            out.push((*col, parsed));
        }
    }
    out
}


pub fn source_generated_inference_for_pair(ober: &str, unter: &str) -> Option<crate::domain::categories::GeneratedInference> {
    let mut generated_befehle = Vec::<String>::new();
    for family in GENERATED_FAMILIES {
        if matches_generated_family(family, ober, unter) {
            generated_befehle.push(family.command.to_string());
        }
    }

    generated_befehle.sort();
    generated_befehle.dedup();

    if generated_befehle.is_empty() {
        None
    } else {
        Some(crate::domain::categories::GeneratedInference {
            generated_befehle,
            required_columns: Vec::new(),
            direct_columns: Vec::new(),
        })
    }
}
