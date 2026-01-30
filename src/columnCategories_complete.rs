// file: columnCategories_complete.rs
use std::collections::{HashMap, HashSet};

// Öffentliche Struktur für Kategorien
#[derive(Debug, Clone)]
pub struct KategorieEintrag {
    pub oberkategorie: String,
    pub unterkategorie: String,
    pub spaltennummern: Vec<u32>,
}

impl KategorieEintrag {
    pub fn new(ober: &str, unter: &str, nummern: Vec<u32>) -> Self {
        Self {
            oberkategorie: ober.to_string(),
            unterkategorie: unter.to_string(),
            spaltennummern: nummern,
        }
    }
}

pub struct KategorieMap {
    pub hauptkategorien: HashMap<String, HashMap<String, Vec<u32>>>,
    pub alle_eintraege: Vec<KategorieEintrag>,
}

impl KategorieMap {
    pub fn new() -> Self {
        let mut instanz = Self {
            hauptkategorien: HashMap::new(),
            alle_eintraege: Vec::new(),
        };
        instanz.lade_kategorien();
        instanz
    }
    // In columnCategories_complete.rs, im impl KategorieMap:
pub fn finde_spaltennummern_fuer_kategorien(&self, ober: &str, unter: &str) -> Vec<u32> {
    let mut gefundene = Vec::new();
    
    for eintrag in &self.alle_eintraege {
        // Fallunabhängiger Vergleich
        if eintrag.oberkategorie.to_lowercase().contains(&ober.to_lowercase()) ||
           eintrag.oberkategorie.replace("_", "").to_lowercase().contains(&ober.to_lowercase()) {
            
            if eintrag.unterkategorie.to_lowercase().contains(&unter.to_lowercase()) ||
               eintrag.unterkategorie.replace("_", "").to_lowercase().contains(&unter.to_lowercase()) {
                
                gefundene.extend_from_slice(&eintrag.spaltennummern);
            }
        }
    }
    
    // Entferne Duplikate und sortiere
    gefundene.sort();
    gefundene.dedup();
    gefundene
}
    fn lade_kategorien(&mut self) {
        let mut main_to_sub = HashMap::new();
        let mut alle_eintraege_temp = Vec::new();
        let data = vec![
        (vec!["Wichtigstes_zum_verstehen", "wichtigsteverstehen"], vec!["Wichtigste", "wichtigste"], vec![10, 5, 4, 8]),
        (vec!["Menschliches", "menschliches"], vec!["Mensch-zu-Tier", "menschtier", "tiermensch"], vec![314]),
        (vec!["Religionen", "religionen", "religion"], vec!["Superkräfte", "Superkraefte"], vec![444, 494, 496, 503]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Superkräfte", "Superkraefte"], vec![444, 494, 496]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Evolution_vs_Design_intelligent"], vec![519]),
        (vec!["Menschliches", "menschliches"], vec!["Evolution_vs_Design_intelligent"], vec![519]),
        (vec!["Menschliches", "menschliches"], vec!["Superkräfte", "Superkraefte"], vec![444, 494, 496]),
        (vec!["Menschliches", "menschliches"], vec!["Formationen"], vec![461]),
        (vec!["Menschliches", "menschliches"], vec!["Ansichten_Standpunkte(18_17)", "ansichten"], vec![240, 346]),
        (vec!["Menschliches", "menschliches"], vec!["(politische)_Richtungen(7)", "richtungen", "politische"], vec![235]),
        (vec!["Planet(10_und_oder_12)", "planet"], vec!["Wirklichkeiten(10)", "wirklichkeit", "wirklichkeiten"], vec![233, 265, 268, 322, 420]),
        (vec!["Planet(10_und_oder_12)", "planet"], vec!["Meta-Systeme(12)", "metasysteme", "metasystem", "meta-systeme", "meta-system"], vec![232, 288, 334, 410, 411, 483, 79, 80, 497, 498, 499]),
        (vec!["Planet(10_und_oder_12)", "planet"], vec!["Intelligenz", "intelligenz"], vec![214]),
        (vec!["Planet(10_und_oder_12)", "planet"], vec!["Gleichheit_Freiheit_Ordnung", "gleichheit", "freiheit", "gleichheit"], vec![132, 324, 328, 79, 80, 331, 335, 497, 498, 499]),
        (vec!["Planet(10_und_oder_12)", "planet"], vec!["Komplexität", "komplexität", "komplexitaet"], vec![213]),
        (vec!["Planet(10_und_oder_12)", "planet"], vec!["Mechanismen", "mechanismen", "mechanismus"], vec![107]),
        (vec!["Wichtigstes_zum_verstehen", "wichtigsteverstehen"], vec!["Zweitwichtigste", "zweitwichtigste"], vec![19, 65, 183]),
        (vec!["Wichtigstes_zum_verstehen", "wichtigsteverstehen"], vec!["Drittwichtigste", "drittwichtigste"], vec![64]),
        (vec!["Wichtigstes_zum_verstehen", "wichtigsteverstehen"], vec!["Motive_Sternpolygone", "viertwichtigste"], vec![]),
        (vec!["Wichtigstes_zum_gedanklich_einordnen", "wichtigsteeinordnen"], vec!["Wichtigste", "wichtigstes"], vec![0, 1, 2, 36, 37, 207]),
        (vec!["Wichtigstes_zum_gedanklich_einordnen", "wichtigsteeinordnen"], vec!["Zweitwichtigste", "zweitwichtigste"], vec![30]),
        (vec!["Operationen", "operationen"], vec!["Halbierung", "halbierung", "halbierungen"], vec![86]),
        (vec!["Religionen", "religionen", "religion"], vec!["Religions-Gründer-Typ", "religionsgründertyp", "prophet", "archon", "religionsgruendertyp"], vec![72, 503]),
        (vec!["Religionen", "religionen", "religion"], vec!["Satan_Teufel"], vec![495]),
        (vec!["Menschliches", "menschliches"], vec!["Satan_Teufel"], vec![495]),
        (vec!["Religionen", "religionen", "religion"], vec!["Hinduismus", "hinduismus"], vec![217]),
        (vec!["Religionen", "religionen", "religion"], vec!["Sternpolygon", "sternpolygon"], vec![0, 6, 36]),
        (vec!["Religionen", "religionen", "religion"], vec!["der_Tierkreiszeichen", "dertierkreiszeichen", "babylon"], vec![0, 36, 207, 477, 478]),
        (vec!["Religionen", "religionen", "religion"], vec!["Sternpolygon_vs_gleichförmiges", "vergleich", "sternpolygonvsgleichfoermiges", "vergleichnvs1divn"], vec![87]),
        (vec!["Religionen", "religionen", "religion"], vec!["Messias", "messias", "heptagramm", "hund", "messiase", "messiasse"], vec![7, 503]),
        (vec!["Religionen", "religionen", "religion"], vec!["gleichförmiges_Polygon", "gleichförmigespolygon", "gleichfoermigespolygon", "nichtsternpolygon", "polygon"], vec![16, 37]),
        (vec!["Religionen", "religionen", "religion"], vec!["Vertreter_höherer_Konzepte", "vertreterhoehererkonzepte", "galaxien", "galaxie", "schwarzesonne", "schwarzesonnen", "universum", "universen", "kreis", "kreise", "kugel", "kugeln"], vec![23]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Lebewesen_Galaxie_am_Besten"], vec![470, 471, 473]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Offenbarung_des_Johannes", "offenbarung", "offenbarungdesjohannes", "johannes", "bibel", "offenbarungjohannes"], vec![90]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["Teilchen-Meta-Physik", "addition", "identitaet", "Identität"], vec![219, 223, 307, 308, 333, 387, 388, 406]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Hochzüchten", "hochzüchten", "hochzuechten"], vec![318, 319]),
        (vec!["Multiversum", "multiversum"], vec!["Teilchen_anderes_Universum"], vec![512]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Teilchen_anderes_Universum"], vec![512]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Teilchen_anderes_Universum"], vec![512]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Zusammenhang_Gehirn_Kosmos_Universum"], vec![489]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Zahlenarten"], vec![462]),
        (vec!["Menschliches", "menschliches"], vec!["Bestrafung"], vec![463]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Bestrafung"], vec![463]),
        (vec!["Menschliches", "menschliches"], vec!["weniger_am_Menschen"], vec![464]),
        (vec!["Menschliches", "menschliches"], vec!["Erlösung", "Erloesung"], vec![465]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Erlösung", "Erloesung"], vec![465]),
        (vec!["Menschliches", "menschliches"], vec!["Gewalt"], vec![466]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Gewalt"], vec![466, 479]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Farben"], vec![444]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["künstliches_Leben(15)", "künstlichesleben", "grosseki"], vec![409]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Software-Lizenzen_akademische_Grade", "softwarelizenz", "akademischeGrade"], vec![422]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Strategie_Taktik(15m8)", "strategie", "taktik"], vec![385]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Universelles_Verhältnis_gleicher_Zahlen", "verhaeltnisgleicherzahl"], vec![383]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["universelles_Recht", "recht", "jura"], vec![382, 34, 65]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["sowas_wie_Kombinieren_Verknüpfen", "kombinierenetc"], vec![320]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Hochzüchten", "hochzüchten", "hochzuechten"], vec![318, 319]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Teilchen-Meta-Physik"], vec![219, 308]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["das_Universelle(15)"], vec![219, 308]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["Wirklichkeiten(10)", "wirklichkeit", "wirklichkeiten"], vec![420]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["das_Galaktische(14)"], vec![406]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["das_Multiverselle(16)"], vec![388, 418]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["die_Tugendsortierung(13_mit_14)"], vec![411]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["die_Galaxie_Unterbereiche(13)"], vec![223, 307, 412]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["das_Gute_die_Richtung(7)"], vec![333]),
        (vec!["Teilchen-Meta-Physik", "teilchen"], vec!["Raum_und_Dimensionen(8)"], vec![387]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["keine_Nur-Paradigma-Religionen", "metaparadigmareligion"], vec![190, 191, 196]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Kugeln_Kreise", "kugelnkreise", "kugeln", "kreise"], vec![77, 145]),
        (vec!["Multiversum", "multiversum"], vec!["Raumzeit_Anordnung_mathematisch_universell"], vec![472]),
        (vec!["Multiversum", "multiversum"], vec!["Multiversalien(16)", "multiversalien"], vec![389]),
        (vec!["Multiversum", "multiversum"], vec!["Meta-Physik-Teilchen(1)", "teilchen"], vec![388]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Kugeln_Kreise", "kugelnkreise", "kugeln", "kreise"], vec![77, 145]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["chinesisches_Horoskop", "chinesischeshoroskop", "china"], vec![91]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["babylonische_Tierkreiszeichen", "tierkreiszeichen", "babylon"], vec![1, 2]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Thomasevangelium", "thomasevangelium", "thomas"], vec![0, 3, 303]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Netzwerk", "netzwerk"], vec![417, 436]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Kontroverse(51)", "kontroverse"], vec![421]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["mathematisches_Design(32)", "mathematischesdesign"], vec![419]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["analytische_Ontologie", "analytischeontologie", "ontologie"], vec![84]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["analytische_Ontologie", "analytischeontologie", "ontologie"], vec![84]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Transzendentalien_innen_außen", "innenaussenstrukur", "strukturalieninnenaußen", "strukturalieninnenaussen", "innenaußenstrukur", "transzendentalieninnenaußen", "transzendentalieninnenaussen"], vec![149]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Modallogik", "modallogik"], vec![148]),
        (vec!["Operationen", "operationen"], vec!["5", "fünf", "fünfer", "fünferstruktur", "fuenf", "fuenfer", "fuenferstruktur"], vec![96]),
        (vec!["Operationen", "operationen"], vec!["9", "neun", "neuner", "neunerstruktur"], vec![94]),
        (vec!["Operationen", "operationen"], vec!["3", "drei", "dreier", "dreierstruktur"], vec![92, 93, 315, 316]),
        (vec!["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"], vec!["Licht", "licht"], vec![20, 27, 313]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Strukturgrösse", "Größenordnung", "größe", "groesse", "gross", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße"], vec![4, 21, 54, 197, 425]),
        (vec!["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"], vec!["Strukturgrösse", "Größenordnung", "größe", "groesse", "gross", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße"], vec![4, 21, 54, 197, 425]),
        (vec!["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"], vec!["Organisationen", "organisationen", "organisation"], vec![30, 82, 425]),
        (vec!["Größenordnung", "groessenordnung", "strukturgroesse", "strukturgroeße", "strukturgrösse", "strukturgröße", "groesse", "stufe", "organisationen"], vec!["politische_Systeme", "politischesysteme", "politik"], vec![83]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["meta"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["konkret"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["Theorie", "theorie"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["Praxis", "praxis"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["Management", "management", "stau"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["verändernd", "veraendernd", "fluss"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["ganzheitlich", "mathematisch_diskret", "diskret"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["darüber_hinausgehend", "hinausgehend", "kontinuierlich"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["Universum_Strukturalien_Transzendentalien", "universum", "strukturalie", "strukturalien", "transzendentalien", "transzendentalie"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["Richtung_als_Richtung", "richtungrichtung"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["Galaxieabsicht", "absichtgalaxie", "absicht", "motive", "motiv", "absichten", "galaxie"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["Absicht_Reziproke_Galaxie", "absichtgalaxiereziproke", "absichtreziproke", "motivereziproke", "motivreziproke", "absichtenreziproke", "galaxiereziproke"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["Universum_Reziproke", "universumreziproke", "strukturaliereziproke", "strukturalienreziproke", "transzendentalienreziproke", "transzendentaliereziproke"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["Dagegen-Gegentranszendentalie", "dagegengegentranszendentalie", "dagegengegentranszendentalien", "dagegengegenstrukturalien", "dagegengegenstrukturalie"], vec![]),
        (vec!["Primzahlwirkung", "primzahlwirkung"], vec!["neutrale_Gegentranszendentalie", "neutralegegentranszendentalie", "neutralegegentranszendentalien", "neutralegegenstrukturalien", "neutralegegenstrukturalie"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["Unternehmung_Geschäft", "unternehmen", "unternehmung", "geschaeft", "geschäft"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["wertvoll", "wert"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["Beherrschen", "regieren", "beherrschen"], vec![]),
        (vec!["Meta_vs_Konkret(Universum)", "universummetakonkret"], vec!["Richtung", "richtung", "gut"], vec![]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["analytische_Ontologie", "analytischeontologie", "ontologie"], vec![84]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Gegentranszendentalien", "gegentranszendentalien", "gegentranszendentalie", "gegenstrukturalien", "gegenalien", "gegenuniversalien"], vec![138, 202]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Systemsachen", "systemsachen"], vec![150]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Transzendentalien", "transzendentalien", "transzendentalie", "strukturalien", "alien", "universalien"], vec![5, 54, 55, 198, 390]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Reziproke_von_Transzendentalien", "transzendentalienreziproke", "transzendentaliereziproke", "strukturalienreziproke", "alienreziproke", "universalienreziproke"], vec![131, 201]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Netzwerk", "netzwerk"], vec![25, 55, 386, 390]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter", "warumtranszendentaliezustrukturgroesseundcharakter"], vec![4, 54, 5, 165]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Kategorie", "kategorie"], vec![204, 205, 281]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Raum-Missionen", "weltall"], vec![218]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Programmier-Paradigmen", "programmierparadigmen"], vec![351]),
        (vec!["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"], vec!["Raum-Missionen", "weltall"], vec![218]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Geist_(15)", "geist"], vec![242, 426]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["warum_Transzendentalie_=_Komplexität_von_Michael_Commons", "warumtranszendentaliegleichkomplexitaet"], vec![65, 5, 166]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Model_of_Hierarchical_Complexity", "modelofhierarchicalcomplexity", "komplex", "komplexität", "komplexitaet", "complexity", "model", "abstraktion"], vec![65, 75, 203, 483]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Model_of_Hierarchical_Complexity", "modelofhierarchicalcomplexity", "komplex", "komplexität", "komplexitaet", "complexity", "model", "abstraktion"], vec![65, 75, 203]),
        (vec!["Multiversum", "multiversum"], vec!["Model_of_Hierarchical_Complexity", "modelofhierarchicalcomplexity", "komplex", "komplexität", "komplexitaet", "complexity", "model", "abstraktion"], vec![65, 75, 203]),
        (vec!["Operationen", "operationen"], vec!["2", "zwei", "gerade", "ungerade", "alternierung", "alternierend", "zweierstruktur"], vec![78, 79, 80, 331, 497, 498, 499]),
        (vec!["Operationen", "operationen"], vec!["Multiplikation", "multiplikation"], vec![158]),
        (vec!["Operationen", "operationen"], vec!["4", "vier", "viererstruktur", "viererabfolgen"], vec![76, 77, 81, 104, 145]),
        (vec!["Menschliches", "menschliches"], vec!["Gesellschaftsschicht", "klasse", "klassen"], vec![241]),
        (vec!["Menschliches", "menschliches"], vec!["Moral", "moral", "warummoral"], vec![215, 216]),
        (vec!["Menschliches", "menschliches"], vec!["Fachgebiete", "fachgebiete", "fachbereiche", "themen"], vec![183]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["Fachgebiete", "fachgebiete", "fachbereiche", "themen"], vec![183]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["Pflanzen", "pflanzen"], vec![113]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["Maschinen", "maschinen", "maschine", "gerät", "geräte", "geraete", "geraet"], vec![89]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["Organisationsform", "organisationsform", "organisationsart", "firma", "verein"], vec![99]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["System", "system"], vec![69, 70, 440, 455, 476, 513]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["System", "system"], vec![69, 70, 440, 455, 476, 513]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["Erklärung", "erklärung", "erklaerung"], vec![71]),
        (vec!["Wirtschaft", "wirtschaft"], vec!["BWL", "bwl"], vec![109]),
        (vec!["Menschliches", "menschliches"], vec!["Sinn_des_Lebens", "sinndeslebens", "lebenssinn", "sinn", "sinnsuche"], vec![88, 189]),
        (vec!["Menschliches", "menschliches"], vec!["Intelligenzprobleme", "intelligenzprobleme", "intelligenzmaengel", "intelligenzmängel"], vec![147]),
        (vec!["Menschliches", "menschliches"], vec!["Denkweise_von_Lebewesen", "lebewesendenkweise", "denkweise"], vec![146]),
        (vec!["Menschliches", "menschliches"], vec!["Gegentranszendentalien", "gegentranszendentalien", "gegenstrukturalien"], vec![138, 139, 202]),
        (vec!["Menschliches", "menschliches"], vec!["Gleichheit_Freiheit", "gleichheitfreiheit", "ungleichheit", "dominieren", "gleichheit", "freiheit"], vec![132, 328, 331, 335]),
        (vec!["Menschliches", "menschliches"], vec!["Gefühle", "emotionen", "gefuehle", "emotion", "gefühl", "gefuehl"], vec![105, 230, 243, 283, 284, 285, 286, 305]),
        (vec!["Menschliches", "menschliches"], vec!["Egoismus", "egoismus", "altruismus", "selbstlosigkeit"], vec![136]),
        (vec!["Menschliches", "menschliches"], vec!["Wirkung", "wirkung"], vec![135]),
        (vec!["Menschliches", "menschliches"], vec!["INCELs", "incel", "incels"], vec![68]),
        (vec!["Menschliches", "menschliches"], vec!["irrationale_Zahlen_durch_Wurzelbildung", "irrationalezahlendurchwurzelbildung", "ausgangslage"], vec![73]),
        (vec!["Menschliches", "menschliches"], vec!["dominierendes_Geschlecht", "dominierendesgeschlecht", "maennlich", "männlich", "weiblich"], vec![51]),
        (vec!["Menschliches", "menschliches"], vec!["Liebe", "liebe", "ethik"], vec![8, 9, 28, 208, 330]),
        (vec!["Menschliches", "menschliches"], vec!["Glaube_Erkenntnis", "glauben", "erkenntnis", "glaube"], vec![59]),
        (vec!["Menschliches", "menschliches"], vec!["Angreifbarkeit", "angreifbarkeit", "angreifbar"], vec![58, 57]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien(15)", "Transzendentalien", "transzendentalien", "transzendentalie", "strukturalien", "alien", "universalien", "meta-paradigmen"], vec![5, 229, 131]),
        (vec!["Multiversum", "multiversum"], vec!["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien(15)", "Transzendentalien", "transzendentalien", "transzendentalie", "strukturalien", "alien", "universalien", "meta-paradigmen"], vec![5, 229, 131]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Bedingung_und_Auslöser(1/3)", "bedingung", "bedingungen", "auslöser", "ausloeser"], vec![338]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Lebensbereiche_Problemklassen(28)", "lebensbereiche", "lebensfelder", "problemklassen"], vec![405, 415, 416]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Maßnahmen(39)", "massnahmen"], vec![384]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Relation_zueinander_reziprok_Universellen(18→n_vs._1/n)", "relativreziprokuniversell"], vec![350]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["universeller_Komperativ(18→15)", "universellerkomperativ"], vec![349]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Existenzialien(3)", "existenzialien"], vec![348]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Extremalien(19)", "extremalien"], vec![347, 352]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Erwartungshaltungen(26)", "erwartungen", "erwartungshaltungen"], vec![344]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Leidenschaften(21)", "leidenschaft", "leidenschaften"], vec![343]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["relativer_Zeit-Betrag(15_10_4_18_6)", "relativerzeitbetrag"], vec![339]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Zahlenvergleich(15_18_6)", "zahlenvergleich"], vec![340]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Bestrebungen(1/5)", "bestrebung", "bestrebungen"], vec![332, 414]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Prinzipien(1/8)", "prinzipien"], vec![329, 378]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Attraktionen(36)", "attraktionen"], vec![311]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Optimierung(10)", "optimierung"], vec![310]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Themen(6)", "themen", "thema"], vec![309]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Bedeutung(10)", "bedeutung"], vec![306]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Reziprokes", "reziproke", "reziprokes"], vec![42, 131, 204, 231, 273, 257, 284, 285, 205, 281, 326, 327, 328, 329, 330, 331, 332, 334, 335, 338, 416]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Achtung(4)", "achtung", "achten"], vec![270, 393]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Zeit(4)_als_Wirklichkeit", "zeit"], vec![266, 267]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_16_ist_zu_genügen", "absicht16"], vec![312]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_17_ist_zu_meinen", "absicht17"], vec![263]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_6_ist_Vorteilsmaximierung", "absicht6"], vec![262]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_7_ist_Selbstlosigkeit", "absicht7"], vec![261]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Bewusstheit_statt_Bewusstsein(1)", "bewusstheit"], vec![282]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Verhalten(11)", "verhalten"], vec![301, 302, 413]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Energie_und_universelle_Eigenschaften(30)", "energie", "universelleeigenschaften", "lebensenergie"], vec![287, 293]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Garben_und_Verhalten_nachfühlen(31)", "garben", "verhaltenfuehlen", "verhaltenfühlen"], vec![295]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus(15)", "nachvollziehen"], vec![242, 297]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Empathie(37)", "empathie", "mitgefuehl"], vec![294]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_1/6_ist_Reinigung_und_Klarheit", "absicht1/6", "absicht1pro6"], vec![298]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["innere_Werte_1/6_der_Reinigung_und_Klarheit", "innerewerte"], vec![398, 399, 400, 401]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_10_ist_Wirklichkeit_erkennen", "absicht10"], vec![260]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Wohlbefinden(7mit6)", "wohlbefinden"], vec![427, 428]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Geist(15)", "geist", "bewusstsein"], vec![229, 231, 242, 273, 297, 304, 426]),
        (vec!["Multiversum", "multiversum"], vec!["Geist(15)", "geist", "bewusstsein"], vec![229, 231, 242, 273, 297, 304, 426]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Reflexe(3)", "reflex", "reflexe"], vec![256]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Lust(9)", "lust", "einheiten"], vec![255, 391]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Paradigmen_sind_Absichten(13)", "paradigmen", "absichten"], vec![10, 42, 410, 411, 493, 494]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Wirklichkeiten_Wahrheit_Wahrnehmung(10)", "wirklichkeit", "wirklichkeiten", "wahrheit", "wahrnehmung"], vec![233, 265, 268, 322, 342, 480]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Rechnen", "rechnen"], vec![404]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Stimmungen_Kombinationen(14)", "stimmung", "stimmungen", "kombination", "kombinationen"], vec![33, 290, 296, 325, 326, 327, 402, 403, 406, 407, 408, 430, 492]),
        (vec!["Multiversum", "multiversum"], vec!["Struktur-Wissenschaften(10)"], vec![438]),
        (vec!["Multiversum", "multiversum"], vec!["Muster-Wissenschaften(20)"], vec![439, 484]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Anführer_Arten(7)"], vec![429, 455, 481, 482, 490, 497, 498, 499, 502, 509]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Klassen(20)", "klasse", "klassen"], vec![241, 289, 394, 395, 485, 516]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Ordnung_und_Filterung_12_und_1pro12", "ordnen", "ordnenundfiltern", "filtern"], vec![132, 328, 331, 335]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Meta-Systeme(12)", "metasysteme", "metasystem", "meta-systeme", "meta-system", "menge", "mengen"], vec![232, 288, 334, 410, 411, 483, 79, 80, 497, 498, 499]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_1/8", "absicht1pro8", "absicht1/8"], vec![272, 379]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Ziele(19)", "ziele", "maxima", "höhenvorstellungen"], vec![271]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Konkreta_und_Focus(2)", "konkreta", "focus", "fokus"], vec![250, 269, 253]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Gefühle(7)", "gefuehle", "emotionen", "emotion", "gefühle"], vec![29, 243, 283, 284, 285, 286, 305]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["abhängige_Verbundenheit(90)", "abhaengigkeit", "abhängigkeit"], vec![357]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Karte_Filter_und_Unterscheidung(1/12)", "karte", "filter", "unterscheidung"], vec![377]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Fundament(1/19)", "fundament"], vec![356]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Gedanken_sind_Positionen(17)", "positionen", "gedanken"], vec![249, 317, 323]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Funktionen_Vorstellungen(16)", "vorstellungen", "vorstellung", "funktionen"], vec![345, 264, 388, 418]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Sollen_Frage_Vorgehensweise(1/13)", "sollen", "frage", "vorgehensweise"], vec![353, 354]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Ansichten_Standpunkte(18_17)", "ansichten"], vec![240, 346]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Verbundenheiten(18)", "verbundenheiten"], vec![252, 299, 300, 336]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Absicht_13_ist_Helfen", "absicht13", "helfen"], vec![370]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Liebe(7)", "liebe"], vec![8, 9, 28, 208, 221, 330]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Koalitionen(10)", "koalitionen"], vec![321]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["gegen_5"], vec![24]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Impulse(5)", "impulse"], vec![251, 253, 257, 341]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Triebe_und_Bedürfnisse(6)", "trieb", "triebe", "bedürfnis", "bedürfnisse", "werte"], vec![254, 392, 396, 397, 423]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Taetigkeiten", "tätigkeiten", "taetigkeiten"], vec![424]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Reflektion_und_Kategorien(1/15)", "reflektion", "kategorien"], vec![204, 205, 281]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Modus_und_Sein(8)", "zustaende", "zustände", "modus", "modi", "sein"], vec![234, 337, 385, 387, 491]),
        (vec!["Menschliches", "menschliches"], vec!["Motive", "motive", "motivation", "motiv", "absicht", "absichten"], vec![10, 18, 42, 167, 168, 149, 229, 230]),
        (vec!["Menschliches", "menschliches"], vec!["Gedanken_sind_Positionen(17)", "positionen", "gedanken"], vec![249, 276]),
        (vec!["Menschliches", "menschliches"], vec!["Bewusstsein_und_Wahrnehmung", "bewusstsein", "wahrnehmung"], vec![265, 229, 231, 281, 304, 342]),
        (vec!["Menschliches", "menschliches"], vec!["Errungenschaften", "errungenschaften", "ziele", "erhalten"], vec![11, 257, 251]),
        (vec!["Menschliches", "menschliches"], vec!["evolutionär_erwerben_und_Intelligenz_Kreativität", "evolutionärerwerbenundintelligenz", "intelligenz", "erwerben", "erlernen", "lernen", "evolutionaer", "evolutionär", "kreativität", "kreativitaet", "kreativ"], vec![12, 47, 27, 13, 32]),
        (vec!["Menschliches", "menschliches"], vec!["brauchen", "benoetigen", "benötigen", "notwendig"], vec![13, 14]),
        (vec!["Menschliches", "menschliches"], vec!["Krankheit", "krankheit", "krankheiten", "pathologisch", "pathologie", "psychiatrisch"], vec![24]),
        (vec!["Menschliches", "menschliches"], vec!["alpha_beta", "alphabeta", "alpha", "beta", "omega", "sigma"], vec![46]),
        (vec!["Menschliches", "menschliches"], vec!["Anführer", "anfuehrer", "chef"], vec![29, 170, 429, 455, 490, 502, 509]),
        (vec!["Grundstrukturen", "grundstrukturen"], vec!["Biologischer_Baum(15)"], vec![500]),
        (vec!["Multiversum", "multiversum"], vec!["Biologischer_Baum(16_->_5)"], vec![500]),
        (vec!["Universum", "universum", "transzendentalien", "strukturalien", "kugel", "kugeln", "ball", "baelle", "bälle"], vec!["Biologischer_Baum(15)"], vec![500]),
        (vec!["Menschliches", "menschliches"], vec!["Biologischer_Baum(15)"], vec![500]),
        (vec!["Menschliches", "menschliches"], vec!["Manipulation", "manipulation"], vec![153]),
        (vec!["Menschliches", "menschliches"], vec!["Berufe", "berufe", "beruf"], vec![30]),
        (vec!["Menschliches", "menschliches"], vec!["Lösungen", "lösungen", "loesungen", "loesung", "lösungen"], vec![31]),
        (vec!["Menschliches", "menschliches"], vec!["Musik", "musik"], vec![33]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["ergibt_Sinn", "ergibtsinn", "machtsinn", "sinn"], vec![140]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Veränderung", "veraenderung", "veraendern", "veränderung", "verändern"], vec![142]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["bändigen_kontrollieren", "baendigenkontrollieren", "kontrollieren", "baendigen", "bändigen"], vec![143]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["vereinen", "einheit"], vec![144]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Vorteile", "vorteile", "veraenderungnutzen"], vec![141]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Gegenspieler", "gegenspieler", "antagonist"], vec![137]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["nervig"], vec![120]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["pro_nutzen", "pronutzen"], vec![117]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Gegenposition", "gegenposition"], vec![116]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Hilfe_erhalten", "hilfeerhalten"], vec![114]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Helfen", "helfen", "hilfe"], vec![115]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Pro", "pro", "dafür", "dafuer"], vec![17, 48]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["nicht_miteinander_auskommen", "nichtauskommen"], vec![123]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["nicht_dagegen", "nichtdagegen"], vec![124]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["kein_Gegenteil", "keingegenteil"], vec![125]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["nicht_dafür", "nichtdafuer"], vec![126]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Hilfe_nicht_gebrauchen", "hilfenichtgebrauchen"], vec![127]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["nicht_helfen_können", "nichthelfenkoennen"], vec![128]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["nicht_abgeneigt", "nichtabgeneigt"], vec![129]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["unmotivierbar"], vec![130]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["contra", "dagegen"], vec![15, 26]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Gegenteil", "gegenteil"], vec![100, 101, 222]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Harmonie", "harmonie"], vec![102, 103]),
        (vec!["Licht", "licht"], vec![""], vec![20, 27, 313]),
        (vec!["Pro_Contra", "procontra", "dagegendafuer"], vec!["Primzahlkreuz_pro_contra", "primzahlkreuz"], vec![]),
        (vec!["Bedeutung", "bedeutung"], vec!["Primzahlkreuz_pro_contra", "primzahlkreuz"], vec![]),
        (vec!["Bedeutung", "bedeutung"], vec!["in_ReTa", "inreta"], vec![209, 210, 474, 475]),
        (vec!["Bedeutung", "bedeutung"], vec!["Vorzeichen", "vorzeichen"], vec![118, 119]),
        (vec!["Bedeutung", "bedeutung"], vec!["Primzahlen", "primzahlen", "vielfache", "vielfacher"], vec![19]),
        (vec!["Bedeutung", "bedeutung"], vec!["Anwendung_der_Sonnen_und_Monde", "anwendungdersonnenundmonde", "anwendungdersonnen", "anwendungenfuermonde"], vec![22]),
        (vec!["Bedeutung", "bedeutung"], vec!["Zählungen", "zählungen", "zaehlung", "zaehlungen", "zählung"], vec![25, 45, 169, 188, 386, 390]),
        (vec!["Bedeutung", "bedeutung"], vec!["Jura", "jura", "gesetzeslehre", "recht"], vec![34]),
        (vec!["Bedeutung", "bedeutung"], vec!["Vollkommenheit_des_Geistes", "vollkommenheit", "geist"], vec![35]),
        (vec!["Bedeutung", "bedeutung"], vec!["Gestirn", "gestirn", "mond", "sonne", "planet"], vec![64, 154]),
        (vec!["Bedeutung", "bedeutung"], vec!["Konjunktiv_Wurzelbildung", "konjunktiv", "wurzel"], vec![106]),
        (vec!["Bedeutung", "bedeutung"], vec!["Mechanismen_der_Züchtung", "mechanismen", "wesen", "zuechtung", "züchtung", "züchten", "zuechten"], vec![107, 108, 109]),
        (vec!["gebrochen-rational_Galaxie_n/m", "gebrochengalaxie"], vec!["{'2'", "'3'", "'4'", "'5'", "'6'", "'7'", "'8'", "'9'", "'10'", "'11'", "'12'", "'13'", "'14'", "'15'", "'16'", "'17'", "'18'", "'19'", "'20'", "'21'", "'22'", "'23'}"], vec![]),
        (vec!["gebrochen-rational_Universum_n/m", "gebrochenuniversum"], vec!["{'2'", "'3'", "'4'", "'5'", "'6'", "'7'", "'8'", "'9'", "'10'", "'11'", "'12'", "'13'", "'14'", "'15'", "'16'", "'17'", "'18'", "'19'", "'20'", "'21'", "'22'", "'23'}"], vec![]),
        (vec!["gebrochen-rational_Gefuehle_n/m", "gebrochenemotion"], vec!["{'2'", "'3'", "'4'", "'5'", "'6'", "'7'", "'8'", "'9'", "'10'", "'11'", "'12'", "'13'", "'14'", "'15'", "'16'", "'17'", "'18'", "'19'", "'20'", "'21'", "'22'", "'23'}"], vec![]),
        (vec!["gebrochen-rational_Strukturgroesse_n/m", "gebrochengroesse"], vec!["{'2'", "'3'", "'4'", "'5'", "'6'", "'7'", "'8'", "'9'", "'10'", "'11'", "'12'", "'13'", "'14'", "'15'", "'16'", "'17'", "'18'", "'19'", "'20'", "'21'", "'22'", "'23'}"], vec![]),
        (vec!["Symbole", "symbole"], vec!["Religionen"], vec![36, 37]),
        (vec!["Symbole", "symbole"], vec!["Drei"], vec![452, 460]),
        (vec!["Symbole", "symbole"], vec!["Vier"], vec![453]),
        (vec!["Symbole", "symbole"], vec!["Fünf", "Fuenf"], vec![454]),
        (vec!["Symbole", "symbole"], vec!["Sechs"], vec![457]),
        (vec!["Symbole", "symbole"], vec!["Sieben"], vec![457]),
        (vec!["Symbole", "symbole"], vec!["Acht"], vec![458]),
        (vec!["Symbole", "symbole"], vec!["Neun"], vec![459]),
        (vec!["Symbole", "symbole"], vec!["Zehn"], vec![456]),
        (vec!["Symbole", "symbole"], vec!["Zwölf", "Zwoelf"], vec![456]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Weisheit_etc", "weisheit", "metaweisheit", "meta-weisheit", "idiot", "weise", "optimal", "optimum"], vec![112]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Dein_Recht_bekommen", "rechte", "recht", "selbstgerecht"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["unterlegen_überlegen", "unterlegen", "ueberlegen"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Ehrlichkeit_und_Streit", "streit", "ehrlichkeit"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Würdig", "wuerdig", "würdig"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Regel_vs_Ausnahme", "regel", "ausnahme"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Filterart_Widrigkeit", "filterart", "widrigkeit"], vec![331, 335]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Werte", "werte"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Gutartigkeits-Egoismus", "position", "gutesreziprok"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Reflektieren_Erkenntnis-Erkennen", "reflektieren", "erkenntnis"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Vertrauen_wollen", "vertrauenwollen"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["einklinken_vertrauen_anprangern", "einklinken", "vertrauenerhalten", "anprangern"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Ausrichten_Einrichten", "einrichten", "ausrichten"], vec![]),
        (vec!["Eigenschaften_1/n", "konzept2", "konzepte2"], vec!["Toleranz_Respekt_Akzeptanz_Willkommen", "toleranz", "respekt", "akzeptanz", "willkommen"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["familiebrauchen"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["ego", "bescheiden"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Selbstsucht_Ichsucht_etc", "selbstsucht", "ichsucht"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Forschen_Erfinden_Einklinken", "wissenschaft", "forschen", "einklinken", "erfinden"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Kooperation_vs_Arsch", "arschloch", "kooperation", "arsch"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Liebe_usw", "liebe", "zuneigung"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Selbstlosigkeit_Ichlosigkeit_etc", "selbstlos", "ichlos"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["variationsreich_eintönig", "eintönig", "eintoenig", "variationsreich"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Zuneigung_Abneigung", "abgeneigt", "zugewandt", "reserviert", "zugeneigt"], vec![]),
        (vec!["Menschliches", "menschliches"], vec!["ehrlich_vs_höflich", "ehrlich", "höflich", "hoeflich"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["ehrlich_vs_höflich", "ehrlich", "höflich", "hoeflich"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Tragweite", "tragweite"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["wertvoll", "wertlos"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Götter_Propheten_Familien_Freunde", "familiaer", "goettlich", "freunde", "propheten"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["sanft_vs_hart", "sanft", "hart"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["vereinen_vs_verbinden", "vereinenverbinden", "vereinen", "verbinden", "einheit", "verbindung"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["ähnlich", "aehnlich"], vec![220]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["gut_böse_lieb_schlecht", "gut", "böse", "boese", "lieb", "schlecht"], vec![52, 53]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Sinn_und_Zweck_des_Lebens", "sinn", "zweck", "bedeutung"], vec![88, 189]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Zeit_vs_Raum", "zeit", "raum", "zeitlich", "räumlich"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["egalitär_vs_autoritär", "egalitaerautoritaer", "egalitaer", "autoritaer", "egalitär", "autoritär"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Meinungen_und_Ruf", "meinungen", "anderemenschen", "ruf"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Meinungsintelligenz", "meinungsintelligenz", "ursprungsintelligenz"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Sittlichkeit", "sittlichkeit", "annaehrerung"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Führung", "führung", "fuehrung"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Durchleuchten", "durchleuchten", "erleuchten"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Fördern_Sensiblisieren_und_Gedeihen", "foerdern", "fördern", "begrenzen", "sensibilisieren", "gedeihen", "verderben"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Überheblichkeit", "überheblich", "ueberheblichkeit", "ueberheblich", "überheblichkeit"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Polung_der_Liebe", "liebepolung"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Egoismus_vs_Altruismus", "egoismus", "altruismus", "egoist", "altruist"], vec![136]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["kausal", "geltung", "genese"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Gleichheit", "gleich"], vec![]),
        (vec!["Eigenschaften_n", "eigenschaften", "eigenschaft", "konzept", "konzepte"], vec!["Überleben", "ueberleben"], vec![]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["set()"], vec![43, 54, 74, 95]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["um1"], vec![155]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["um2"], vec![156]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["um3"], vec![157]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter", "warumtranszendentaliezustrukturgroesseundcharakter"], vec![4, 54, 5, 165]),
        (vec!["Inkrementieren", "inkrementieren"], vec!["warum_Transzendentalie_=_Komplexität_von_Michael_Commons", "warumtranszendentaliegleichkomplexitaet"], vec![65, 5, 166]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Rahmen-Bedingungen", "rahmen"], vec![226]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Motive_gleichförmige_Polygone", "motivgleichfoermig"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Struktur_gleichförmige_Polygone", "strukturgleichfoermig"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Motive_Sternpolygone", "motivstern"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Struktur_Sternpolygone", "strukturstern"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Motiv_Sternpolygon_gebrochen-rational", "motivgebrstern"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Struktur_Sternpolyon_gebrochen-rational", "strukgebrstern"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Motiv_gleichförmige_Polygone_gebrochen-rational", "motivgebrgleichf"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["Struktur_gleichförmige_Polygone_gebrochen-rational", "strukgebrgleichf"], vec![]),
        (vec!["Multiplikationen", "multiplikationen"], vec!["beschrieben"], vec![]),
        (vec!["Kontinuum", "kontinuum"], vec!["Q", "q", "Siebzehn"], vec![431, 432, 433, 434, 437, 441, 442, 443, 445, 450, 467, 468, 469, 487, 488]),
        (vec!["Kontinuum", "kontinuum"], vec!["i", "I", "Neun"], vec![517]),
        (vec!["Kontinuum", "kontinuum"], vec!["G", "g", "Sieben"], vec![518]),
        (vec!["Kontinuum", "kontinuum"], vec!["J", "j", "Zehn"], vec![514]),
        (vec!["Kontinuum", "kontinuum"], vec!["k", "K", "Elf"], vec![515]),
        (vec!["Kontinuum", "kontinuum"], vec!["E", "e", "Fünf"], vec![511]),
        (vec!["Kontinuum", "kontinuum"], vec!["L", "l", "Zwölf"], vec![506]),
        (vec!["Kontinuum", "kontinuum"], vec!["Y", "y", "Fünfundzwanzig"], vec![507, 510]),
        (vec!["Kontinuum", "kontinuum"], vec!["Kontinuen", "F", "f", "Sechs"], vec![508]),
        (vec!["Kontinuum", "kontinuum"], vec!["F", "f", "Sechs", "Kontinuen"], vec![508]),
        (vec!["Kontinuum", "kontinuum"], vec!["O", "o", "Fünfzehn"], vec![5]),
        (vec!["Kontinuum", "kontinuum"], vec!["H", "h", "Acht"], vec![491]),
        (vec!["Kontinuum", "kontinuum"], vec!["N", "n", "Vierzehn"], vec![492]),
        (vec!["Kontinuum", "kontinuum"], vec!["M", "m", "Dreizehn"], vec![493]),
        (vec!["Kontinuum", "kontinuum"], vec!["T", "t", "Zwanzig"], vec![486]),
        (vec!["Multiversum", "multiversum"], vec!["P", "p", "Sechszehn"], vec![435]),
        (vec!["Kontinuum", "kontinuum"], vec!["P5", "p5", "Sechszehn->Fünf"], vec![501]),
        (vec!["Multiversum", "multiversum"], vec!["P5", "p5", "Sechszehn->Fünf"], vec![501]),
        (vec!["Kontinuum", "kontinuum"], vec!["P", "p", "Sechszehn"], vec![435]),
        (vec!["Kontinuum", "kontinuum"], vec!["X", "x", "Vierundzwanzig"], vec![25, 55, 436, 386]),
        (vec!["Kontinuum", "kontinuum"], vec!["S", "s", "Neunzehn"], vec![504]),
        (vec!["Kontinuum", "kontinuum"], vec!["R", "r", "Achtzehn"], vec![451, 436]),
        (vec!["Kontinuum", "kontinuum"], vec!["A", "a", "Eins"], vec![446]),
        (vec!["Kontinuum", "kontinuum"], vec!["B", "b", "Zwei"], vec![447]),
        (vec!["Kontinuum", "kontinuum"], vec!["C", "c", "Drei"], vec![448]),
        (vec!["Kontinuum", "kontinuum"], vec!["D", "d", "Vier"], vec![449]),
    ];
 
        // DEIN KOMPLETTER DATENSATZ HIER - ich zeige nur ein Beispiel
        for (main_categories, sub_categories, ids) in data {
            for &main_cat in &main_categories {
                for &sub_cat in &sub_categories {
                    Self::insert_entry(&mut main_to_sub, main_cat, sub_cat, ids.clone());
                    
                    // Auch in die flache Liste aufnehmen
                    alle_eintraege_temp.push(KategorieEintrag::new(
                        main_cat,
                        sub_cat,
                        ids.clone()
                    ));
                }
            }
        }

        self.hauptkategorien = main_to_sub;
        self.alle_eintraege = alle_eintraege_temp;
    }

    fn insert_entry(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
        main_category: &str,
        sub_category: &str,
        new_ids: Vec<u32>
    ) {
        let main_entry = main_to_sub
            .entry(main_category.to_string())
            .or_insert_with(HashMap::new);

        let existing_ids = main_entry
            .entry(sub_category.to_string())
            .or_insert_with(Vec::new);

        let mut all_ids: HashSet<u32> = existing_ids.iter().cloned().collect();
        for &id in &new_ids {
            all_ids.insert(id);
        }

        let mut sorted_ids: Vec<u32> = all_ids.into_iter().collect();
        sorted_ids.sort();
        *existing_ids = sorted_ids;
    }
    
    // Methode um Kategorien nach Spaltennummer zu filtern
    pub fn filtere_nach_spaltennummern(&self, nummern: &[usize]) -> Vec<&KategorieEintrag> {
        let nummern_set: HashSet<u32> = nummern.iter().map(|&n| n as u32).collect();
        
        self.alle_eintraege
            .iter()
            .filter(|eintrag| {
                eintrag.spaltennummern.iter().any(|num| nummern_set.contains(num))
            })
            .collect()
    }
   
    // In columnCategories_complete.rs, innerhalb des impl KategorieMap:

pub fn generiere_sql_inserts_nur(&self, 
                                 oberkategorie_name: &str, 
                                 unterkategorie_name: &str,
                                 spalten_filter: Option<&[usize]>) -> String {
    let mut output = String::new();
    
    // CREATE TABLE
    output.push_str("CREATE TABLE kategorien (\n");
    output.push_str("  id INTEGER PRIMARY KEY AUTOINCREMENT,\n");
    output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", oberkategorie_name));
    output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", unterkategorie_name));
    output.push_str("  spaltennummer INTEGER NOT NULL\n");
    output.push_str(");\n\n");
    
    // INSERT Statements
    output.push_str("INSERT INTO kategorien (");
    output.push_str(oberkategorie_name);
    output.push_str(", ");
    output.push_str(unterkategorie_name);
    output.push_str(", spaltennummer) VALUES\n");
    
    let mut first = true;
    let mut has_data = false;
    
    for eintrag in &self.alle_eintraege {
        // Filtern nach Spaltennummern falls gewünscht
        if let Some(filter) = spalten_filter {
            let filter_set: HashSet<u32> = filter.iter().map(|&n| n as u32).collect();
            let hat_treffer = eintrag.spaltennummern.iter().any(|num| filter_set.contains(num));
            if !hat_treffer {
                continue;
            }
        }
        
        for &spaltennummer in &eintrag.spaltennummern {
            if !first {
                output.push_str(",\n");
            }
            output.push_str(&format!("  ('{}', '{}', {})", 
                                   eintrag.oberkategorie, 
                                   eintrag.unterkategorie, 
                                   spaltennummer));
            first = false;
            has_data = true;
        }
    }
    
    if has_data {
        output.push_str(";\n");
    } else {
        // Falls keine Daten vorhanden sind, leeren INSERT vermeiden
        output = output.lines()
            .filter(|line| !line.contains("INSERT INTO"))
            .collect::<Vec<_>>()
            .join("\n");
    }
    
    output
}

    // Methode um SQL-SELECTs zu generieren
    pub fn generiere_sql_selects(&self, 
                                 oberkategorie_name: &str, 
                                 unterkategorie_name: &str,
                                 spalten_filter: Option<&[usize]>) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("-- SQL SELECTS für Kategorie-Datenbank\n"));
        output.push_str(&format!("-- Spaltennamen: {}, {}\n\n", 
                               oberkategorie_name, unterkategorie_name));
        
        // CREATE TABLE
        output.push_str("CREATE TABLE kategorien (\n");
        output.push_str("  id INTEGER PRIMARY KEY AUTOINCREMENT,\n");
        output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", oberkategorie_name));
        output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", unterkategorie_name));
        output.push_str("  spaltennummer INTEGER NOT NULL\n");
        output.push_str(");\n\n");
        
        // INSERT Statements
        output.push_str("INSERT INTO kategorien (");
        output.push_str(oberkategorie_name);
        output.push_str(", ");
        output.push_str(unterkategorie_name);
        output.push_str(", spaltennummer) VALUES\n");
        
        let mut first = true;
        for eintrag in &self.alle_eintraege {
            // Filtern nach Spaltennummern falls gewünscht
            if let Some(filter) = spalten_filter {
                let filter_set: HashSet<u32> = filter.iter().map(|&n| n as u32).collect();
                let hat_treffer = eintrag.spaltennummern.iter().any(|num| filter_set.contains(num));
                if !hat_treffer {
                    continue;
                }
            }
            
            for &spaltennummer in &eintrag.spaltennummern {
                if !first {
                    output.push_str(",\n");
                }
                output.push_str(&format!("  ('{}', '{}', {})", 
                                       eintrag.oberkategorie, 
                                       eintrag.unterkategorie, 
                                       spaltennummer));
                first = false;
            }
        }
        
        if !first {
            output.push_str(";\n\n");
        }
        
        // SELECT Examples
        output.push_str("-- Beispiele für SELECT-Abfragen:\n\n");
        output.push_str(&format!("-- 1. Alle eindeutigen {}s:\n", oberkategorie_name));
        output.push_str(&format!("SELECT DISTINCT {} FROM kategorien ORDER BY {};\n\n", 
                               oberkategorie_name, oberkategorie_name));
        
        output.push_str(&format!("-- 2. {}s für eine bestimmte {}:\n", 
                               unterkategorie_name, oberkategorie_name));
        output.push_str(&format!("SELECT DISTINCT {} FROM kategorien ", unterkategorie_name));
        output.push_str(&format!("WHERE {} = 'Menschliches' ", oberkategorie_name));
        output.push_str(&format!("ORDER BY {};\n\n", unterkategorie_name));
        
        output.push_str("-- 3. Spaltennummern für eine Kategorie-Kombination:\n");
        output.push_str(&format!("SELECT spaltennummer FROM kategorien "));
        output.push_str(&format!("WHERE {} = 'Universum' ", oberkategorie_name));
        output.push_str(&format!("AND {} = 'Transzendentalien';\n", unterkategorie_name));
        
        output
    }
}

// Öffentliche Funktion um die Kategorie-Map zu erhalten
pub fn lade_kategorie_map() -> KategorieMap {
    KategorieMap::new()
}
