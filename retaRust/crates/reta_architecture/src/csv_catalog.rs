//! Generated CSV asset catalog extracted from `python_arch_reference/csv`.
//!
//! Stage 19 makes the concrete CSV presheaf visible to Rust.  This module
//! carries CSV metadata, static `include_str!` accessors and a small CSV parser
//! so concat/Kombi/religion tables can be inspected without falling back to the
//! old Python runtime.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CsvLanguage {
    Base,
    English,
    Chinese,
    Vietnamese,
    Korean,
}

impl CsvLanguage {
    pub fn canonical(self) -> &'static str {
        match self {
            CsvLanguage::Base => "base",
            CsvLanguage::English => "en",
            CsvLanguage::Chinese => "cn",
            CsvLanguage::Vietnamese => "vn",
            CsvLanguage::Korean => "kr",
        }
    }

    pub fn from_language_value(value: &str) -> Option<Self> {
        let normalized = normalize_language_value(value);
        match normalized.as_str() {
            "" | "de" | "deutsch" | "german" | "base" => Some(CsvLanguage::Base),
            "en" | "english" | "englisch" => Some(CsvLanguage::English),
            "cn" | "chinese" | "chinesisch" | "中國人" => Some(CsvLanguage::Chinese),
            "vn" | "vietnamese" | "vietnamesisch" | "tiếngviệt" | "tiengviet" => {
                Some(CsvLanguage::Vietnamese)
            }
            "kr" | "korean" | "koreanisch" | "한국인" => Some(CsvLanguage::Korean),
            _ => None,
        }
    }

    pub fn from_cli_args<S: AsRef<str>>(args: &[S]) -> Self {
        csv_language_from_cli_args(args)
    }
}

pub fn normalize_language_value(value: &str) -> String {
    value
        .trim()
        .trim_matches('\'')
        .trim_matches('"')
        .replace([' ', '_', '-'], "")
        .to_lowercase()
}

pub fn language_value_from_cli_arg(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    let body = trimmed
        .strip_prefix("--")
        .or_else(|| trimmed.strip_prefix('-'))
        .unwrap_or(trimmed);
    for prefix in [
        "language=",
        "languages=",
        "sprache=",
        "sprachen=",
        "lang=",
    ] {
        if let Some(value) = body.strip_prefix(prefix) {
            return Some(value);
        }
    }
    None
}

pub fn csv_language_from_cli_args<S: AsRef<str>>(args: &[S]) -> CsvLanguage {
    args.iter()
        .filter_map(|arg| language_value_from_cli_arg(arg.as_ref()))
        .filter_map(CsvLanguage::from_language_value)
        .last()
        .unwrap_or(CsvLanguage::Base)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CsvAssetKind {
    SymbolsAltCircleSphere,
    DualismTrinities,
    GebrochenRationalEmotionen,
    GebrochenRationalGalaxie,
    GebrochenRationalStrukturgroesse,
    GebrochenRationalUniversum,
    KombiGedankenAbsichtenBewusstsein,
    KombiMetaSysteme,
    KombiMeta,
    KombiUniverselleWirklichkeit,
    Kombi,
    KreisVomTyp18,
    MeaningOfLife,
    PrimeNumbers,
    Religion,
    SunMoonEtc,
    ThomasDecodedMotivesPurposes,
    Other,
}

impl CsvAssetKind {
    pub fn canonical(self) -> &'static str {
        match self {
            CsvAssetKind::SymbolsAltCircleSphere => "SymbolsAltCircleSphere",
            CsvAssetKind::DualismTrinities => "DualismTrinities",
            CsvAssetKind::GebrochenRationalEmotionen => "GebrochenRationalEmotionen",
            CsvAssetKind::GebrochenRationalGalaxie => "GebrochenRationalGalaxie",
            CsvAssetKind::GebrochenRationalStrukturgroesse => "GebrochenRationalStrukturgroesse",
            CsvAssetKind::GebrochenRationalUniversum => "GebrochenRationalUniversum",
            CsvAssetKind::KombiGedankenAbsichtenBewusstsein => "KombiGedankenAbsichtenBewusstsein",
            CsvAssetKind::KombiMetaSysteme => "KombiMetaSysteme",
            CsvAssetKind::KombiMeta => "KombiMeta",
            CsvAssetKind::KombiUniverselleWirklichkeit => "KombiUniverselleWirklichkeit",
            CsvAssetKind::Kombi => "Kombi",
            CsvAssetKind::KreisVomTyp18 => "KreisVomTyp18",
            CsvAssetKind::MeaningOfLife => "MeaningOfLife",
            CsvAssetKind::PrimeNumbers => "PrimeNumbers",
            CsvAssetKind::Religion => "Religion",
            CsvAssetKind::SunMoonEtc => "SunMoonEtc",
            CsvAssetKind::ThomasDecodedMotivesPurposes => "ThomasDecodedMotivesPurposes",
            CsvAssetKind::Other => "Other",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CsvDelimiter {
    Semicolon,
    Comma,
    Tab,
    Pipe,
}

impl CsvDelimiter {
    pub fn as_char(self) -> char {
        match self {
            CsvDelimiter::Semicolon => ';',
            CsvDelimiter::Comma => ',',
            CsvDelimiter::Tab => '\t',
            CsvDelimiter::Pipe => '|',
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct CsvAsset {
    pub name: &'static str,
    pub base_name: &'static str,
    pub language: CsvLanguage,
    pub kind: CsvAssetKind,
    pub delimiter: CsvDelimiter,
    pub row_count: usize,
    pub max_columns: usize,
    pub header_columns: usize,
    pub nonempty_cell_count: usize,
    pub byte_len: usize,
    pub header_preview: &'static str,
}

impl CsvAsset {
    pub fn text(self) -> Option<&'static str> {
        csv_text_by_name(self.name)
    }

    pub fn rows(self) -> Vec<Vec<String>> {
        self.text()
            .map(|text| parse_csv_text(text, self.delimiter))
            .unwrap_or_default()
    }

    pub fn owned(self) -> OwnedCsvAsset {
        OwnedCsvAsset {
            name: self.name.to_string(),
            base_name: self.base_name.to_string(),
            language: self.language,
            kind: self.kind,
            delimiter: self.delimiter,
            row_count: self.row_count,
            max_columns: self.max_columns,
            header_columns: self.header_columns,
            nonempty_cell_count: self.nonempty_cell_count,
            byte_len: self.byte_len,
            header_preview: self.header_preview.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedCsvAsset {
    pub name: String,
    pub base_name: String,
    pub language: CsvLanguage,
    pub kind: CsvAssetKind,
    pub delimiter: CsvDelimiter,
    pub row_count: usize,
    pub max_columns: usize,
    pub header_columns: usize,
    pub nonempty_cell_count: usize,
    pub byte_len: usize,
    pub header_preview: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedCsvCatalogBundle {
    pub assets: Vec<OwnedCsvAsset>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CsvCatalogBundle {
    pub assets: Vec<CsvAsset>,
}

impl CsvCatalogBundle {
    pub fn snapshot(&self) -> CsvCatalogSnapshot {
        CsvCatalogSnapshot {
            class: "CsvCatalogBundle".to_string(),
            asset_count: self.assets.len(),
            base_asset_count: self.assets.iter().filter(|asset| asset.language == CsvLanguage::Base).count(),
            language_variant_count: self.assets.iter().filter(|asset| asset.language != CsvLanguage::Base).count(),
            total_row_count: self.assets.iter().map(|asset| asset.row_count).sum(),
            total_nonempty_cell_count: self.assets.iter().map(|asset| asset.nonempty_cell_count).sum(),
            semicolon_asset_count: self.assets.iter().filter(|asset| asset.delimiter == CsvDelimiter::Semicolon).count(),
            comma_asset_count: self.assets.iter().filter(|asset| asset.delimiter == CsvDelimiter::Comma).count(),
            religion_row_count: csv_asset_by_name("religion.csv").map(|asset| asset.row_count).unwrap_or(0),
            kombi_meta_row_count: csv_asset_by_name("kombi-meta.csv").map(|asset| asset.row_count).unwrap_or(0),
        }
    }

    pub fn owned(&self) -> OwnedCsvCatalogBundle {
        OwnedCsvCatalogBundle {
            assets: self.assets.iter().copied().map(CsvAsset::owned).collect(),
        }
    }

    pub fn by_name(&self, name: &str) -> Option<CsvAsset> {
        self.assets.iter().copied().find(|asset| asset.name == name)
    }

    pub fn by_kind(&self, kind: CsvAssetKind) -> Vec<CsvAsset> {
        self.assets.iter().copied().filter(|asset| asset.kind == kind).collect()
    }

    pub fn by_language(&self, language: CsvLanguage) -> Vec<CsvAsset> {
        self.assets.iter().copied().filter(|asset| asset.language == language).collect()
    }

    pub fn rows_by_name(&self, name: &str) -> Option<Vec<Vec<String>>> {
        csv_rows_by_name(name)
    }

    pub fn cell_by_name(&self, name: &str, row_one_based: usize, column_one_based: usize) -> Option<String> {
        csv_cell_by_name(name, row_one_based, column_one_based)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CsvCatalogSnapshot {
    pub class: String,
    pub asset_count: usize,
    pub base_asset_count: usize,
    pub language_variant_count: usize,
    pub total_row_count: usize,
    pub total_nonempty_cell_count: usize,
    pub semicolon_asset_count: usize,
    pub comma_asset_count: usize,
    pub religion_row_count: usize,
    pub kombi_meta_row_count: usize,
}

pub const CSV_ASSETS: &[CsvAsset] = &[
    CsvAsset {
        name: "2024-07-06-symbols-alt-ak-circle-sphere-etc.csv",
        base_name: "2024-07-06-symbols-alt-ak-circle-sphere-etc.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::SymbolsAltCircleSphere,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 25,
        max_columns: 10,
        header_columns: 10,
        nonempty_cell_count: 211,
        byte_len: 3045,
        header_preview: "Sternpolygon (n) | gleichförmiges Polygon (1/n) | Star polygon (n) | uniform polygon (1/n) | Sternpolygon-Bestandteile \"1 bis 3\" | Star polygon components \"1 to 3\"",
    },
    CsvAsset {
        name: "cn-dualism-trinities-etc.csv",
        base_name: "dualism-trinities-etc.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::DualismTrinities,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 24,
        max_columns: 10,
        header_columns: 10,
        nonempty_cell_count: 78,
        byte_len: 2462,
        header_preview: "unipolar, monopolar | Dualismen | Tripel, Triniäten | 4 | 5 | 6",
    },
    CsvAsset {
        name: "cn-gebrochen-rational-emotionen.csv",
        base_name: "gebrochen-rational-emotionen.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::GebrochenRationalEmotionen,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 7,
        max_columns: 7,
        header_columns: 7,
        nonempty_cell_count: 27,
        byte_len: 955,
        header_preview: " | weich |  |  |  | ",
    },
    CsvAsset {
        name: "cn-gebrochen-rational-galaxie.csv",
        base_name: "gebrochen-rational-galaxie.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::GebrochenRationalGalaxie,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 21,
        header_columns: 21,
        nonempty_cell_count: 82,
        byte_len: 10084,
        header_preview: "Absicht (jegliche beliebige Absichten) | Möglich (Handlungsmöglichkeit = 4 / 2 = (1/4)/(1/2)= etwas schaffen pro Möglichkeit) | benötigen | schaffen | taugen wollen und deshalb streben | einschätzen",
    },
    CsvAsset {
        name: "cn-gebrochen-rational-strukturgroesse.csv",
        base_name: "gebrochen-rational-strukturgroesse.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::GebrochenRationalStrukturgroesse,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 16,
        max_columns: 16,
        header_columns: 16,
        nonempty_cell_count: 40,
        byte_len: 4425,
        header_preview: "Einzelperson (D.h. z.B. bei Altruismus, sich selbst gegenüber altruistisch sein, also kein Altruismus, wie wir ihn definieren. Damit soll nur gesagt sein, dass die Strukturgröße streng gedacht werden soll.) | Nation / Gruppierung, als viell",
    },
    CsvAsset {
        name: "cn-gebrochen-rational-universum.csv",
        base_name: "gebrochen-rational-universum.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::GebrochenRationalUniversum,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 19,
        header_columns: 19,
        nonempty_cell_count: 86,
        byte_len: 16959,
        header_preview: "Identität, z.B. mathematisch | 1 / 2 Art und Weise (fast sicher),2 / 4 die Gegebenheit (Ausgangslage) (sicher), 3/6 die Beschaffenheit (stimmt wahrscheinlich), 4/8 die Begebenheit (stimmt wahrscheinlich), 5/10 ganzer schlüssiger Wahrheitsum",
    },
    CsvAsset {
        name: "cn-kombi-gedanken17-absichten13-bewusstsein15.csv",
        base_name: "kombi-gedanken17-absichten13-bewusstsein15.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::KombiGedankenAbsichtenBewusstsein,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 10,
        max_columns: 4,
        header_columns: 4,
        nonempty_cell_count: 31,
        byte_len: 692,
        header_preview: "15:17 Gedanken (17) als Form des Geistes (15) | Paradigmen (13) also Absichten | universelle Strukturalie (15) oder Bewusstein (15) | Ergebnis der drei ersten Spalten oder menschliches",
    },
    CsvAsset {
        name: "cn-kombi-meta-systeme.csv",
        base_name: "kombi-meta-systeme.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::KombiMetaSysteme,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 4,
        byte_len: 383,
        header_preview: "noch unbekannt, was Vorzeichen meinen könnte | kombinierte Meta-Systeme",
    },
    CsvAsset {
        name: "cn-kombi-meta.csv",
        base_name: "kombi-meta.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::KombiMeta,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 262,
        max_columns: 20,
        header_columns: 20,
        nonempty_cell_count: 525,
        byte_len: 39132,
        header_preview: "Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gegentranszendentalien | Lebewesen (Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gege",
    },
    CsvAsset {
        name: "cn-kombi-universelle-wirklichkeit.csv",
        base_name: "kombi-universelle-wirklichkeit.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::KombiUniverselleWirklichkeit,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 3,
        header_columns: 3,
        nonempty_cell_count: 6,
        byte_len: 74,
        header_preview: "universelle Strukturalien | Wirklichkeits-Phänomene | Ergebnis",
    },
    CsvAsset {
        name: "cn-kombi.csv",
        base_name: "kombi.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::Kombi,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 261,
        max_columns: 18,
        header_columns: 18,
        nonempty_cell_count: 489,
        byte_len: 23641,
        header_preview: " | Lebewesen (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) | Beruf (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit",
    },
    CsvAsset {
        name: "cn-kreisVomTyp18.csv",
        base_name: "kreisVomTyp18.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::KreisVomTyp18,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 20,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 40,
        byte_len: 1003,
        header_preview: "Gegentranszendentalien, Gegenstrukturalien und Transzendentalien, Strukturalien (15 und -15) | Meinungsrelation (18 zu Angelegenheiten der 17)",
    },
    CsvAsset {
        name: "cn-meaningOfLife.csv",
        base_name: "meaningOfLife.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::MeaningOfLife,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 5,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 55,
        byte_len: 1945,
        header_preview: "allgemeiner Sinn | direkter Sinn (nicht allgemein genug) | Kodierung Farben | Kodierung Elemente | Beispiel Tätigkeiten | Warum",
    },
    CsvAsset {
        name: "cn-primenumbers.csv",
        base_name: "primenumbers.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::PrimeNumbers,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 101,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 67,
        byte_len: 9498,
        header_preview: "2 | 3 | 5 | 7 | 11 | 13",
    },
    CsvAsset {
        name: "cn-religion.csv",
        base_name: "religion.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::Religion,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 1025,
        max_columns: 745,
        header_columns: 745,
        nonempty_cell_count: 310613,
        byte_len: 3664706,
        header_preview: "Religionen der Föderation in unserer Galaxie Milchstraße (unabhängig vom Vorzeichen: das bedeutet nicht Antitranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) (14) (n, Sternpolygon) | Babylonische Tierkr",
    },
    CsvAsset {
        name: "cn-sunMoonEtc.csv",
        base_name: "sunMoonEtc.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::SunMoonEtc,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 114,
        max_columns: 6,
        header_columns: 6,
        nonempty_cell_count: 222,
        byte_len: 2581,
        header_preview: "Sonne |  |  |  |  | identisch sein, macht etwas einzigartig",
    },
    CsvAsset {
        name: "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        base_name: "thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        language: CsvLanguage::Chinese,
        kind: CsvAssetKind::ThomasDecodedMotivesPurposes,
        delimiter: CsvDelimiter::Comma,
        row_count: 325,
        max_columns: 13,
        header_columns: 5,
        nonempty_cell_count: 353,
        byte_len: 32836,
        header_preview: "内在的(不是外在的)(内在=直接而不是间接地想要它)星形多边形的情感意图或动机 | 范式(其中13 | 范式不仅是类型13 | 因为人性化)(有任何动机是类型1)(每个个体动机也源于人类由哪些范式和元范式组成 | 因此它始终是一个组合)意图(13)作为心灵的一种形式(15)",
    },
    CsvAsset {
        name: "dualism-trinities-etc.csv",
        base_name: "dualism-trinities-etc.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::DualismTrinities,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 24,
        max_columns: 10,
        header_columns: 10,
        nonempty_cell_count: 78,
        byte_len: 2462,
        header_preview: "unipolar, monopolar | Dualismen | Tripel, Triniäten | 4 | 5 | 6",
    },
    CsvAsset {
        name: "en-dualism-trinities-etc.csv",
        base_name: "dualism-trinities-etc.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::DualismTrinities,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 24,
        max_columns: 10,
        header_columns: 10,
        nonempty_cell_count: 78,
        byte_len: 2462,
        header_preview: "unipolar, monopolar | Dualismen | Tripel, Triniäten | 4 | 5 | 6",
    },
    CsvAsset {
        name: "en-gebrochen-rational-emotionen.csv",
        base_name: "gebrochen-rational-emotionen.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::GebrochenRationalEmotionen,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 7,
        max_columns: 7,
        header_columns: 7,
        nonempty_cell_count: 27,
        byte_len: 955,
        header_preview: " | weich |  |  |  | ",
    },
    CsvAsset {
        name: "en-gebrochen-rational-galaxie.csv",
        base_name: "gebrochen-rational-galaxie.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::GebrochenRationalGalaxie,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 21,
        header_columns: 21,
        nonempty_cell_count: 82,
        byte_len: 10084,
        header_preview: "Absicht (jegliche beliebige Absichten) | Möglich (Handlungsmöglichkeit = 4 / 2 = (1/4)/(1/2)= etwas schaffen pro Möglichkeit) | benötigen | schaffen | taugen wollen und deshalb streben | einschätzen",
    },
    CsvAsset {
        name: "en-gebrochen-rational-strukturgroesse.csv",
        base_name: "gebrochen-rational-strukturgroesse.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::GebrochenRationalStrukturgroesse,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 16,
        max_columns: 16,
        header_columns: 16,
        nonempty_cell_count: 40,
        byte_len: 4425,
        header_preview: "Einzelperson (D.h. z.B. bei Altruismus, sich selbst gegenüber altruistisch sein, also kein Altruismus, wie wir ihn definieren. Damit soll nur gesagt sein, dass die Strukturgröße streng gedacht werden soll.) | Nation / Gruppierung, als viell",
    },
    CsvAsset {
        name: "en-gebrochen-rational-universum.csv",
        base_name: "gebrochen-rational-universum.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::GebrochenRationalUniversum,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 19,
        header_columns: 19,
        nonempty_cell_count: 86,
        byte_len: 16959,
        header_preview: "Identität, z.B. mathematisch | 1 / 2 Art und Weise (fast sicher),2 / 4 die Gegebenheit (Ausgangslage) (sicher), 3/6 die Beschaffenheit (stimmt wahrscheinlich), 4/8 die Begebenheit (stimmt wahrscheinlich), 5/10 ganzer schlüssiger Wahrheitsum",
    },
    CsvAsset {
        name: "en-kombi-gedanken17-absichten13-bewusstsein15.csv",
        base_name: "kombi-gedanken17-absichten13-bewusstsein15.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::KombiGedankenAbsichtenBewusstsein,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 10,
        max_columns: 4,
        header_columns: 4,
        nonempty_cell_count: 31,
        byte_len: 692,
        header_preview: "15:17 Gedanken (17) als Form des Geistes (15) | Paradigmen (13) also Absichten | universelle Strukturalie (15) oder Bewusstein (15) | Ergebnis der drei ersten Spalten oder menschliches",
    },
    CsvAsset {
        name: "en-kombi-meta-systeme.csv",
        base_name: "kombi-meta-systeme.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::KombiMetaSysteme,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 4,
        byte_len: 383,
        header_preview: "noch unbekannt, was Vorzeichen meinen könnte | kombinierte Meta-Systeme",
    },
    CsvAsset {
        name: "en-kombi-meta.csv",
        base_name: "kombi-meta.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::KombiMeta,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 262,
        max_columns: 20,
        header_columns: 20,
        nonempty_cell_count: 525,
        byte_len: 39132,
        header_preview: "Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gegentranszendentalien | Lebewesen (Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gege",
    },
    CsvAsset {
        name: "en-kombi-universelle-wirklichkeit.csv",
        base_name: "kombi-universelle-wirklichkeit.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::KombiUniverselleWirklichkeit,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 3,
        header_columns: 3,
        nonempty_cell_count: 6,
        byte_len: 74,
        header_preview: "universelle Strukturalien | Wirklichkeits-Phänomene | Ergebnis",
    },
    CsvAsset {
        name: "en-kombi.csv",
        base_name: "kombi.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::Kombi,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 261,
        max_columns: 18,
        header_columns: 18,
        nonempty_cell_count: 489,
        byte_len: 23641,
        header_preview: " | Lebewesen (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) | Beruf (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit",
    },
    CsvAsset {
        name: "en-kreisVomTyp18.csv",
        base_name: "kreisVomTyp18.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::KreisVomTyp18,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 20,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 40,
        byte_len: 1003,
        header_preview: "Gegentranszendentalien, Gegenstrukturalien und Transzendentalien, Strukturalien (15 und -15) | Meinungsrelation (18 zu Angelegenheiten der 17)",
    },
    CsvAsset {
        name: "en-meaningOfLife.csv",
        base_name: "meaningOfLife.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::MeaningOfLife,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 5,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 55,
        byte_len: 1945,
        header_preview: "allgemeiner Sinn | direkter Sinn (nicht allgemein genug) | Kodierung Farben | Kodierung Elemente | Beispiel Tätigkeiten | Warum",
    },
    CsvAsset {
        name: "en-primenumbers.csv",
        base_name: "primenumbers.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::PrimeNumbers,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 101,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 67,
        byte_len: 9498,
        header_preview: "2 | 3 | 5 | 7 | 11 | 13",
    },
    CsvAsset {
        name: "en-religion.csv",
        base_name: "religion.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::Religion,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 1025,
        max_columns: 745,
        header_columns: 745,
        nonempty_cell_count: 310613,
        byte_len: 3664706,
        header_preview: "Religionen der Föderation in unserer Galaxie Milchstraße (unabhängig vom Vorzeichen: das bedeutet nicht Antitranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) (14) (n, Sternpolygon) | Babylonische Tierkr",
    },
    CsvAsset {
        name: "en-sunMoonEtc.csv",
        base_name: "sunMoonEtc.csv",
        language: CsvLanguage::English,
        kind: CsvAssetKind::SunMoonEtc,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 114,
        max_columns: 6,
        header_columns: 6,
        nonempty_cell_count: 222,
        byte_len: 2581,
        header_preview: "Sonne |  |  |  |  | identisch sein, macht etwas einzigartig",
    },
    CsvAsset {
        name: "gebrochen-rational-emotionen.csv",
        base_name: "gebrochen-rational-emotionen.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::GebrochenRationalEmotionen,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 7,
        max_columns: 7,
        header_columns: 7,
        nonempty_cell_count: 27,
        byte_len: 955,
        header_preview: " | weich |  |  |  | ",
    },
    CsvAsset {
        name: "gebrochen-rational-galaxie.csv",
        base_name: "gebrochen-rational-galaxie.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::GebrochenRationalGalaxie,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 21,
        header_columns: 21,
        nonempty_cell_count: 82,
        byte_len: 10084,
        header_preview: "Absicht (jegliche beliebige Absichten) | Möglich (Handlungsmöglichkeit = 4 / 2 = (1/4)/(1/2)= etwas schaffen pro Möglichkeit) | benötigen | schaffen | taugen wollen und deshalb streben | einschätzen",
    },
    CsvAsset {
        name: "gebrochen-rational-strukturgroesse.csv",
        base_name: "gebrochen-rational-strukturgroesse.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::GebrochenRationalStrukturgroesse,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 16,
        max_columns: 16,
        header_columns: 16,
        nonempty_cell_count: 40,
        byte_len: 4425,
        header_preview: "Einzelperson (D.h. z.B. bei Altruismus, sich selbst gegenüber altruistisch sein, also kein Altruismus, wie wir ihn definieren. Damit soll nur gesagt sein, dass die Strukturgröße streng gedacht werden soll.) | Nation / Gruppierung, als viell",
    },
    CsvAsset {
        name: "gebrochen-rational-universum.csv",
        base_name: "gebrochen-rational-universum.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::GebrochenRationalUniversum,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 19,
        header_columns: 19,
        nonempty_cell_count: 86,
        byte_len: 16959,
        header_preview: "Identität, z.B. mathematisch | 1 / 2 Art und Weise (fast sicher),2 / 4 die Gegebenheit (Ausgangslage) (sicher), 3/6 die Beschaffenheit (stimmt wahrscheinlich), 4/8 die Begebenheit (stimmt wahrscheinlich), 5/10 ganzer schlüssiger Wahrheitsum",
    },
    CsvAsset {
        name: "kombi-gedanken17-absichten13-bewusstsein15.csv",
        base_name: "kombi-gedanken17-absichten13-bewusstsein15.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::KombiGedankenAbsichtenBewusstsein,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 10,
        max_columns: 4,
        header_columns: 4,
        nonempty_cell_count: 31,
        byte_len: 692,
        header_preview: "15:17 Gedanken (17) als Form des Geistes (15) | Paradigmen (13) also Absichten | universelle Strukturalie (15) oder Bewusstein (15) | Ergebnis der drei ersten Spalten oder menschliches",
    },
    CsvAsset {
        name: "kombi-meta-systeme.csv",
        base_name: "kombi-meta-systeme.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::KombiMetaSysteme,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 4,
        byte_len: 383,
        header_preview: "noch unbekannt, was Vorzeichen meinen könnte | kombinierte Meta-Systeme",
    },
    CsvAsset {
        name: "kombi-meta.csv",
        base_name: "kombi-meta.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::KombiMeta,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 262,
        max_columns: 20,
        header_columns: 20,
        nonempty_cell_count: 525,
        byte_len: 39555,
        header_preview: "Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gegentranszendentalien | Lebewesen (Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gege",
    },
    CsvAsset {
        name: "kombi-universelle-wirklichkeit.csv",
        base_name: "kombi-universelle-wirklichkeit.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::KombiUniverselleWirklichkeit,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 3,
        header_columns: 3,
        nonempty_cell_count: 6,
        byte_len: 74,
        header_preview: "universelle Strukturalien | Wirklichkeits-Phänomene | Ergebnis",
    },
    CsvAsset {
        name: "kombi.csv",
        base_name: "kombi.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::Kombi,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 261,
        max_columns: 18,
        header_columns: 18,
        nonempty_cell_count: 489,
        byte_len: 23641,
        header_preview: " | Lebewesen (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) | Beruf (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit",
    },
    CsvAsset {
        name: "kr-dualism-trinities-etc.csv",
        base_name: "dualism-trinities-etc.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::DualismTrinities,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 24,
        max_columns: 10,
        header_columns: 10,
        nonempty_cell_count: 78,
        byte_len: 2462,
        header_preview: "unipolar, monopolar | Dualismen | Tripel, Triniäten | 4 | 5 | 6",
    },
    CsvAsset {
        name: "kr-gebrochen-rational-emotionen.csv",
        base_name: "gebrochen-rational-emotionen.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::GebrochenRationalEmotionen,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 7,
        max_columns: 7,
        header_columns: 7,
        nonempty_cell_count: 27,
        byte_len: 955,
        header_preview: " | weich |  |  |  | ",
    },
    CsvAsset {
        name: "kr-gebrochen-rational-galaxie.csv",
        base_name: "gebrochen-rational-galaxie.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::GebrochenRationalGalaxie,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 21,
        header_columns: 21,
        nonempty_cell_count: 82,
        byte_len: 10084,
        header_preview: "Absicht (jegliche beliebige Absichten) | Möglich (Handlungsmöglichkeit = 4 / 2 = (1/4)/(1/2)= etwas schaffen pro Möglichkeit) | benötigen | schaffen | taugen wollen und deshalb streben | einschätzen",
    },
    CsvAsset {
        name: "kr-gebrochen-rational-strukturgroesse.csv",
        base_name: "gebrochen-rational-strukturgroesse.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::GebrochenRationalStrukturgroesse,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 16,
        max_columns: 16,
        header_columns: 16,
        nonempty_cell_count: 40,
        byte_len: 4425,
        header_preview: "Einzelperson (D.h. z.B. bei Altruismus, sich selbst gegenüber altruistisch sein, also kein Altruismus, wie wir ihn definieren. Damit soll nur gesagt sein, dass die Strukturgröße streng gedacht werden soll.) | Nation / Gruppierung, als viell",
    },
    CsvAsset {
        name: "kr-gebrochen-rational-universum.csv",
        base_name: "gebrochen-rational-universum.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::GebrochenRationalUniversum,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 19,
        header_columns: 19,
        nonempty_cell_count: 86,
        byte_len: 16959,
        header_preview: "Identität, z.B. mathematisch | 1 / 2 Art und Weise (fast sicher),2 / 4 die Gegebenheit (Ausgangslage) (sicher), 3/6 die Beschaffenheit (stimmt wahrscheinlich), 4/8 die Begebenheit (stimmt wahrscheinlich), 5/10 ganzer schlüssiger Wahrheitsum",
    },
    CsvAsset {
        name: "kr-kombi-gedanken17-absichten13-bewusstsein15.csv",
        base_name: "kombi-gedanken17-absichten13-bewusstsein15.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::KombiGedankenAbsichtenBewusstsein,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 10,
        max_columns: 4,
        header_columns: 4,
        nonempty_cell_count: 31,
        byte_len: 692,
        header_preview: "15:17 Gedanken (17) als Form des Geistes (15) | Paradigmen (13) also Absichten | universelle Strukturalie (15) oder Bewusstein (15) | Ergebnis der drei ersten Spalten oder menschliches",
    },
    CsvAsset {
        name: "kr-kombi-meta-systeme.csv",
        base_name: "kombi-meta-systeme.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::KombiMetaSysteme,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 4,
        byte_len: 383,
        header_preview: "noch unbekannt, was Vorzeichen meinen könnte | kombinierte Meta-Systeme",
    },
    CsvAsset {
        name: "kr-kombi-meta.csv",
        base_name: "kombi-meta.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::KombiMeta,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 262,
        max_columns: 20,
        header_columns: 20,
        nonempty_cell_count: 525,
        byte_len: 39132,
        header_preview: "Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gegentranszendentalien | Lebewesen (Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gege",
    },
    CsvAsset {
        name: "kr-kombi-universelle-wirklichkeit.csv",
        base_name: "kombi-universelle-wirklichkeit.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::KombiUniverselleWirklichkeit,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 3,
        header_columns: 3,
        nonempty_cell_count: 6,
        byte_len: 74,
        header_preview: "universelle Strukturalien | Wirklichkeits-Phänomene | Ergebnis",
    },
    CsvAsset {
        name: "kr-kombi.csv",
        base_name: "kombi.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::Kombi,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 261,
        max_columns: 18,
        header_columns: 18,
        nonempty_cell_count: 489,
        byte_len: 23641,
        header_preview: " | Lebewesen (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) | Beruf (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit",
    },
    CsvAsset {
        name: "kr-kreisVomTyp18.csv",
        base_name: "kreisVomTyp18.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::KreisVomTyp18,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 20,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 40,
        byte_len: 1003,
        header_preview: "Gegentranszendentalien, Gegenstrukturalien und Transzendentalien, Strukturalien (15 und -15) | Meinungsrelation (18 zu Angelegenheiten der 17)",
    },
    CsvAsset {
        name: "kr-meaningOfLife.csv",
        base_name: "meaningOfLife.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::MeaningOfLife,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 5,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 55,
        byte_len: 1945,
        header_preview: "allgemeiner Sinn | direkter Sinn (nicht allgemein genug) | Kodierung Farben | Kodierung Elemente | Beispiel Tätigkeiten | Warum",
    },
    CsvAsset {
        name: "kr-primenumbers.csv",
        base_name: "primenumbers.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::PrimeNumbers,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 101,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 67,
        byte_len: 9498,
        header_preview: "2 | 3 | 5 | 7 | 11 | 13",
    },
    CsvAsset {
        name: "kr-religion.csv",
        base_name: "religion.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::Religion,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 1025,
        max_columns: 745,
        header_columns: 745,
        nonempty_cell_count: 310613,
        byte_len: 3664706,
        header_preview: "Religionen der Föderation in unserer Galaxie Milchstraße (unabhängig vom Vorzeichen: das bedeutet nicht Antitranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) (14) (n, Sternpolygon) | Babylonische Tierkr",
    },
    CsvAsset {
        name: "kr-sunMoonEtc.csv",
        base_name: "sunMoonEtc.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::SunMoonEtc,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 114,
        max_columns: 6,
        header_columns: 6,
        nonempty_cell_count: 222,
        byte_len: 2581,
        header_preview: "Sonne |  |  |  |  | identisch sein, macht etwas einzigartig",
    },
    CsvAsset {
        name: "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        base_name: "thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        language: CsvLanguage::Korean,
        kind: CsvAssetKind::ThomasDecodedMotivesPurposes,
        delimiter: CsvDelimiter::Comma,
        row_count: 325,
        max_columns: 9,
        header_columns: 9,
        nonempty_cell_count: 307,
        byte_len: 48802,
        header_preview: "13: 본질적 (외재적이지 않음)(본질적 = 직접적으로 원하고 간접적으로 원하지 않음) 별 다각형에 대한 감정적 의도 또는 동기 |  패러다임(다른 13개 중에서 패러다임은 인간화되었기 때문에 유형 13일 뿐만 아니라)(어떤 동기를 갖는 것이 유형 1임(각 개인의 동기 또한 인간 존재를 구성하는 패러다임과 메타 패러다임의 결과이므로 항상 구성입니다) 마음의 형태로(15)서의 의도 |  |  |  | ",
    },
    CsvAsset {
        name: "kreisVomTyp18.csv",
        base_name: "kreisVomTyp18.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::KreisVomTyp18,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 20,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 40,
        byte_len: 1003,
        header_preview: "Gegentranszendentalien, Gegenstrukturalien und Transzendentalien, Strukturalien (15 und -15) | Meinungsrelation (18 zu Angelegenheiten der 17)",
    },
    CsvAsset {
        name: "meaningOfLife.csv",
        base_name: "meaningOfLife.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::MeaningOfLife,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 5,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 55,
        byte_len: 1945,
        header_preview: "allgemeiner Sinn | direkter Sinn (nicht allgemein genug) | Kodierung Farben | Kodierung Elemente | Beispiel Tätigkeiten | Warum",
    },
    CsvAsset {
        name: "primenumbers.csv",
        base_name: "primenumbers.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::PrimeNumbers,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 101,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 67,
        byte_len: 9498,
        header_preview: "2 | 3 | 5 | 7 | 11 | 13",
    },
    CsvAsset {
        name: "religion.csv",
        base_name: "religion.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::Religion,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 1025,
        max_columns: 745,
        header_columns: 745,
        nonempty_cell_count: 310613,
        byte_len: 3664706,
        header_preview: "Religionen der Föderation in unserer Galaxie Milchstraße (unabhängig vom Vorzeichen: das bedeutet nicht Antitranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) (14) (n, Sternpolygon) | Babylonische Tierkr",
    },
    CsvAsset {
        name: "sunMoonEtc.csv",
        base_name: "sunMoonEtc.csv",
        language: CsvLanguage::Base,
        kind: CsvAssetKind::SunMoonEtc,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 114,
        max_columns: 6,
        header_columns: 6,
        nonempty_cell_count: 222,
        byte_len: 2581,
        header_preview: "Sonne |  |  |  |  | identisch sein, macht etwas einzigartig",
    },
    CsvAsset {
        name: "vn-dualism-trinities-etc.csv",
        base_name: "dualism-trinities-etc.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::DualismTrinities,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 24,
        max_columns: 10,
        header_columns: 10,
        nonempty_cell_count: 78,
        byte_len: 2462,
        header_preview: "unipolar, monopolar | Dualismen | Tripel, Triniäten | 4 | 5 | 6",
    },
    CsvAsset {
        name: "vn-gebrochen-rational-emotionen.csv",
        base_name: "gebrochen-rational-emotionen.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::GebrochenRationalEmotionen,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 7,
        max_columns: 7,
        header_columns: 7,
        nonempty_cell_count: 27,
        byte_len: 955,
        header_preview: " | weich |  |  |  | ",
    },
    CsvAsset {
        name: "vn-gebrochen-rational-galaxie.csv",
        base_name: "gebrochen-rational-galaxie.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::GebrochenRationalGalaxie,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 21,
        header_columns: 21,
        nonempty_cell_count: 82,
        byte_len: 10084,
        header_preview: "Absicht (jegliche beliebige Absichten) | Möglich (Handlungsmöglichkeit = 4 / 2 = (1/4)/(1/2)= etwas schaffen pro Möglichkeit) | benötigen | schaffen | taugen wollen und deshalb streben | einschätzen",
    },
    CsvAsset {
        name: "vn-gebrochen-rational-strukturgroesse.csv",
        base_name: "gebrochen-rational-strukturgroesse.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::GebrochenRationalStrukturgroesse,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 16,
        max_columns: 16,
        header_columns: 16,
        nonempty_cell_count: 40,
        byte_len: 4425,
        header_preview: "Einzelperson (D.h. z.B. bei Altruismus, sich selbst gegenüber altruistisch sein, also kein Altruismus, wie wir ihn definieren. Damit soll nur gesagt sein, dass die Strukturgröße streng gedacht werden soll.) | Nation / Gruppierung, als viell",
    },
    CsvAsset {
        name: "vn-gebrochen-rational-universum.csv",
        base_name: "gebrochen-rational-universum.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::GebrochenRationalUniversum,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 21,
        max_columns: 19,
        header_columns: 19,
        nonempty_cell_count: 86,
        byte_len: 16959,
        header_preview: "Identität, z.B. mathematisch | 1 / 2 Art und Weise (fast sicher),2 / 4 die Gegebenheit (Ausgangslage) (sicher), 3/6 die Beschaffenheit (stimmt wahrscheinlich), 4/8 die Begebenheit (stimmt wahrscheinlich), 5/10 ganzer schlüssiger Wahrheitsum",
    },
    CsvAsset {
        name: "vn-kombi-gedanken17-absichten13-bewusstsein15.csv",
        base_name: "kombi-gedanken17-absichten13-bewusstsein15.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::KombiGedankenAbsichtenBewusstsein,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 10,
        max_columns: 4,
        header_columns: 4,
        nonempty_cell_count: 31,
        byte_len: 692,
        header_preview: "15:17 Gedanken (17) als Form des Geistes (15) | Paradigmen (13) also Absichten | universelle Strukturalie (15) oder Bewusstein (15) | Ergebnis der drei ersten Spalten oder menschliches",
    },
    CsvAsset {
        name: "vn-kombi-meta-systeme.csv",
        base_name: "kombi-meta-systeme.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::KombiMetaSysteme,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 4,
        byte_len: 383,
        header_preview: "noch unbekannt, was Vorzeichen meinen könnte | kombinierte Meta-Systeme",
    },
    CsvAsset {
        name: "vn-kombi-meta.csv",
        base_name: "kombi-meta.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::KombiMeta,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 262,
        max_columns: 20,
        header_columns: 20,
        nonempty_cell_count: 525,
        byte_len: 39132,
        header_preview: "Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gegentranszendentalien | Lebewesen (Vorzeichen: das bedeutet nicht Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber, sondern Gege",
    },
    CsvAsset {
        name: "vn-kombi-universelle-wirklichkeit.csv",
        base_name: "kombi-universelle-wirklichkeit.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::KombiUniverselleWirklichkeit,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 2,
        max_columns: 3,
        header_columns: 3,
        nonempty_cell_count: 6,
        byte_len: 74,
        header_preview: "universelle Strukturalien | Wirklichkeits-Phänomene | Ergebnis",
    },
    CsvAsset {
        name: "vn-kombi.csv",
        base_name: "kombi.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::Kombi,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 261,
        max_columns: 18,
        header_columns: 18,
        nonempty_cell_count: 489,
        byte_len: 23641,
        header_preview: " | Lebewesen (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) | Beruf (Vorzeichen: das bedeutet nicht Gegentranszendentalien, sondern Gutartigkeit oder Bösartigkeit",
    },
    CsvAsset {
        name: "vn-kreisVomTyp18.csv",
        base_name: "kreisVomTyp18.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::KreisVomTyp18,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 20,
        max_columns: 2,
        header_columns: 2,
        nonempty_cell_count: 40,
        byte_len: 1003,
        header_preview: "Gegentranszendentalien, Gegenstrukturalien und Transzendentalien, Strukturalien (15 und -15) | Meinungsrelation (18 zu Angelegenheiten der 17)",
    },
    CsvAsset {
        name: "vn-meaningOfLife.csv",
        base_name: "meaningOfLife.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::MeaningOfLife,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 5,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 55,
        byte_len: 1945,
        header_preview: "allgemeiner Sinn | direkter Sinn (nicht allgemein genug) | Kodierung Farben | Kodierung Elemente | Beispiel Tätigkeiten | Warum",
    },
    CsvAsset {
        name: "vn-primenumbers.csv",
        base_name: "primenumbers.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::PrimeNumbers,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 101,
        max_columns: 11,
        header_columns: 11,
        nonempty_cell_count: 67,
        byte_len: 9498,
        header_preview: "2 | 3 | 5 | 7 | 11 | 13",
    },
    CsvAsset {
        name: "vn-religion.csv",
        base_name: "religion.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::Religion,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 1025,
        max_columns: 745,
        header_columns: 745,
        nonempty_cell_count: 310613,
        byte_len: 3664706,
        header_preview: "Religionen der Föderation in unserer Galaxie Milchstraße (unabhängig vom Vorzeichen: das bedeutet nicht Antitranszendentalien, sondern Gutartigkeit oder Bösartigkeit anderen Religionen gegenüber) (14) (n, Sternpolygon) | Babylonische Tierkr",
    },
    CsvAsset {
        name: "vn-sunMoonEtc.csv",
        base_name: "sunMoonEtc.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::SunMoonEtc,
        delimiter: CsvDelimiter::Semicolon,
        row_count: 114,
        max_columns: 6,
        header_columns: 6,
        nonempty_cell_count: 222,
        byte_len: 2581,
        header_preview: "Sonne |  |  |  |  | identisch sein, macht etwas einzigartig",
    },
    CsvAsset {
        name: "vn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        base_name: "thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        language: CsvLanguage::Vietnamese,
        kind: CsvAssetKind::ThomasDecodedMotivesPurposes,
        delimiter: CsvDelimiter::Comma,
        row_count: 325,
        max_columns: 10,
        header_columns: 4,
        nonempty_cell_count: 310,
        byte_len: 54086,
        header_preview: "Nội tại (không phải bên ngoài) (nội tại = muốn nó trực tiếp và không gián tiếp) ý dịnh hoặc dộng co cảm xúc dối với da giác sao |  mô hình (trong số 13 mô hình khác |  mô hình không chỉ thuộc loại 13 vì duợc nhân bản hóa) (Có bất kỳ dộng co",
    },
];

pub fn bootstrap_csv_catalog() -> CsvCatalogBundle {
    CsvCatalogBundle { assets: CSV_ASSETS.to_vec() }
}

pub fn csv_catalog_owned() -> OwnedCsvCatalogBundle {
    bootstrap_csv_catalog().owned()
}

pub fn csv_asset_records() -> Vec<OwnedCsvAsset> {
    CSV_ASSETS.iter().copied().map(CsvAsset::owned).collect()
}

pub fn csv_asset_count() -> usize {
    CSV_ASSETS.len()
}

pub fn csv_total_row_count() -> usize {
    CSV_ASSETS.iter().map(|asset| asset.row_count).sum()
}

pub fn csv_language_variant_count() -> usize {
    CSV_ASSETS.iter().filter(|asset| asset.language != CsvLanguage::Base).count()
}

pub fn csv_asset_by_name(name: &str) -> Option<CsvAsset> {
    CSV_ASSETS.iter().copied().find(|asset| asset.name == name)
}

pub fn csv_asset_by_base_and_language(base_name: &str, language: CsvLanguage) -> Option<CsvAsset> {
    CSV_ASSETS
        .iter()
        .copied()
        .find(|asset| asset.base_name == base_name && asset.language == language)
}

pub fn csv_base_asset(base_name: &str) -> Option<CsvAsset> {
    csv_asset_by_base_and_language(base_name, CsvLanguage::Base)
}

pub fn csv_asset_supports_columns(asset: CsvAsset, columns_zero_based: &[usize]) -> bool {
    columns_zero_based
        .iter()
        .all(|column| *column < asset.max_columns)
}

pub fn csv_asset_for_language_with_required_columns(
    base_name: &str,
    language: CsvLanguage,
    columns_zero_based: &[usize],
) -> Option<CsvAsset> {
    let language_asset = csv_asset_by_base_and_language(base_name, language);
    if let Some(asset) = language_asset {
        if columns_zero_based.is_empty() || csv_asset_supports_columns(asset, columns_zero_based) {
            return Some(asset);
        }
    }
    let base_asset = csv_base_asset(base_name);
    if let Some(asset) = base_asset {
        if columns_zero_based.is_empty() || csv_asset_supports_columns(asset, columns_zero_based) {
            return Some(asset);
        }
    }
    language_asset.or(base_asset)
}

pub fn csv_assets_by_kind(kind: CsvAssetKind) -> Vec<CsvAsset> {
    CSV_ASSETS.iter().copied().filter(|asset| asset.kind == kind).collect()
}

pub fn csv_assets_by_language(language: CsvLanguage) -> Vec<CsvAsset> {
    CSV_ASSETS.iter().copied().filter(|asset| asset.language == language).collect()
}

pub fn csv_text_by_name(name: &str) -> Option<&'static str> {
    match name {
        "2024-07-06-symbols-alt-ak-circle-sphere-etc.csv" => Some(include_str!("../../../python_arch_reference/csv/2024-07-06-symbols-alt-ak-circle-sphere-etc.csv")),
        "cn-dualism-trinities-etc.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-dualism-trinities-etc.csv")),
        "cn-gebrochen-rational-emotionen.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-gebrochen-rational-emotionen.csv")),
        "cn-gebrochen-rational-galaxie.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-gebrochen-rational-galaxie.csv")),
        "cn-gebrochen-rational-strukturgroesse.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-gebrochen-rational-strukturgroesse.csv")),
        "cn-gebrochen-rational-universum.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-gebrochen-rational-universum.csv")),
        "cn-kombi-gedanken17-absichten13-bewusstsein15.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-kombi-gedanken17-absichten13-bewusstsein15.csv")),
        "cn-kombi-meta-systeme.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-kombi-meta-systeme.csv")),
        "cn-kombi-meta.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-kombi-meta.csv")),
        "cn-kombi-universelle-wirklichkeit.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-kombi-universelle-wirklichkeit.csv")),
        "cn-kombi.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-kombi.csv")),
        "cn-kreisVomTyp18.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-kreisVomTyp18.csv")),
        "cn-meaningOfLife.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-meaningOfLife.csv")),
        "cn-primenumbers.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-primenumbers.csv")),
        "cn-religion.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-religion.csv")),
        "cn-sunMoonEtc.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-sunMoonEtc.csv")),
        "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv" => Some(include_str!("../../../python_arch_reference/csv/cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv")),
        "dualism-trinities-etc.csv" => Some(include_str!("../../../python_arch_reference/csv/dualism-trinities-etc.csv")),
        "en-dualism-trinities-etc.csv" => Some(include_str!("../../../python_arch_reference/csv/en-dualism-trinities-etc.csv")),
        "en-gebrochen-rational-emotionen.csv" => Some(include_str!("../../../python_arch_reference/csv/en-gebrochen-rational-emotionen.csv")),
        "en-gebrochen-rational-galaxie.csv" => Some(include_str!("../../../python_arch_reference/csv/en-gebrochen-rational-galaxie.csv")),
        "en-gebrochen-rational-strukturgroesse.csv" => Some(include_str!("../../../python_arch_reference/csv/en-gebrochen-rational-strukturgroesse.csv")),
        "en-gebrochen-rational-universum.csv" => Some(include_str!("../../../python_arch_reference/csv/en-gebrochen-rational-universum.csv")),
        "en-kombi-gedanken17-absichten13-bewusstsein15.csv" => Some(include_str!("../../../python_arch_reference/csv/en-kombi-gedanken17-absichten13-bewusstsein15.csv")),
        "en-kombi-meta-systeme.csv" => Some(include_str!("../../../python_arch_reference/csv/en-kombi-meta-systeme.csv")),
        "en-kombi-meta.csv" => Some(include_str!("../../../python_arch_reference/csv/en-kombi-meta.csv")),
        "en-kombi-universelle-wirklichkeit.csv" => Some(include_str!("../../../python_arch_reference/csv/en-kombi-universelle-wirklichkeit.csv")),
        "en-kombi.csv" => Some(include_str!("../../../python_arch_reference/csv/en-kombi.csv")),
        "en-kreisVomTyp18.csv" => Some(include_str!("../../../python_arch_reference/csv/en-kreisVomTyp18.csv")),
        "en-meaningOfLife.csv" => Some(include_str!("../../../python_arch_reference/csv/en-meaningOfLife.csv")),
        "en-primenumbers.csv" => Some(include_str!("../../../python_arch_reference/csv/en-primenumbers.csv")),
        "en-religion.csv" => Some(include_str!("../../../python_arch_reference/csv/en-religion.csv")),
        "en-sunMoonEtc.csv" => Some(include_str!("../../../python_arch_reference/csv/en-sunMoonEtc.csv")),
        "gebrochen-rational-emotionen.csv" => Some(include_str!("../../../python_arch_reference/csv/gebrochen-rational-emotionen.csv")),
        "gebrochen-rational-galaxie.csv" => Some(include_str!("../../../python_arch_reference/csv/gebrochen-rational-galaxie.csv")),
        "gebrochen-rational-strukturgroesse.csv" => Some(include_str!("../../../python_arch_reference/csv/gebrochen-rational-strukturgroesse.csv")),
        "gebrochen-rational-universum.csv" => Some(include_str!("../../../python_arch_reference/csv/gebrochen-rational-universum.csv")),
        "kombi-gedanken17-absichten13-bewusstsein15.csv" => Some(include_str!("../../../python_arch_reference/csv/kombi-gedanken17-absichten13-bewusstsein15.csv")),
        "kombi-meta-systeme.csv" => Some(include_str!("../../../python_arch_reference/csv/kombi-meta-systeme.csv")),
        "kombi-meta.csv" => Some(include_str!("../../../python_arch_reference/csv/kombi-meta.csv")),
        "kombi-universelle-wirklichkeit.csv" => Some(include_str!("../../../python_arch_reference/csv/kombi-universelle-wirklichkeit.csv")),
        "kombi.csv" => Some(include_str!("../../../python_arch_reference/csv/kombi.csv")),
        "kr-dualism-trinities-etc.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-dualism-trinities-etc.csv")),
        "kr-gebrochen-rational-emotionen.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-gebrochen-rational-emotionen.csv")),
        "kr-gebrochen-rational-galaxie.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-gebrochen-rational-galaxie.csv")),
        "kr-gebrochen-rational-strukturgroesse.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-gebrochen-rational-strukturgroesse.csv")),
        "kr-gebrochen-rational-universum.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-gebrochen-rational-universum.csv")),
        "kr-kombi-gedanken17-absichten13-bewusstsein15.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-kombi-gedanken17-absichten13-bewusstsein15.csv")),
        "kr-kombi-meta-systeme.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-kombi-meta-systeme.csv")),
        "kr-kombi-meta.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-kombi-meta.csv")),
        "kr-kombi-universelle-wirklichkeit.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-kombi-universelle-wirklichkeit.csv")),
        "kr-kombi.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-kombi.csv")),
        "kr-kreisVomTyp18.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-kreisVomTyp18.csv")),
        "kr-meaningOfLife.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-meaningOfLife.csv")),
        "kr-primenumbers.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-primenumbers.csv")),
        "kr-religion.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-religion.csv")),
        "kr-sunMoonEtc.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-sunMoonEtc.csv")),
        "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv" => Some(include_str!("../../../python_arch_reference/csv/kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv")),
        "kreisVomTyp18.csv" => Some(include_str!("../../../python_arch_reference/csv/kreisVomTyp18.csv")),
        "meaningOfLife.csv" => Some(include_str!("../../../python_arch_reference/csv/meaningOfLife.csv")),
        "primenumbers.csv" => Some(include_str!("../../../python_arch_reference/csv/primenumbers.csv")),
        "religion.csv" => Some(include_str!("../../../python_arch_reference/csv/religion.csv")),
        "sunMoonEtc.csv" => Some(include_str!("../../../python_arch_reference/csv/sunMoonEtc.csv")),
        "vn-dualism-trinities-etc.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-dualism-trinities-etc.csv")),
        "vn-gebrochen-rational-emotionen.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-gebrochen-rational-emotionen.csv")),
        "vn-gebrochen-rational-galaxie.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-gebrochen-rational-galaxie.csv")),
        "vn-gebrochen-rational-strukturgroesse.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-gebrochen-rational-strukturgroesse.csv")),
        "vn-gebrochen-rational-universum.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-gebrochen-rational-universum.csv")),
        "vn-kombi-gedanken17-absichten13-bewusstsein15.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-kombi-gedanken17-absichten13-bewusstsein15.csv")),
        "vn-kombi-meta-systeme.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-kombi-meta-systeme.csv")),
        "vn-kombi-meta.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-kombi-meta.csv")),
        "vn-kombi-universelle-wirklichkeit.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-kombi-universelle-wirklichkeit.csv")),
        "vn-kombi.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-kombi.csv")),
        "vn-kreisVomTyp18.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-kreisVomTyp18.csv")),
        "vn-meaningOfLife.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-meaningOfLife.csv")),
        "vn-primenumbers.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-primenumbers.csv")),
        "vn-religion.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-religion.csv")),
        "vn-sunMoonEtc.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-sunMoonEtc.csv")),
        "vn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv" => Some(include_str!("../../../python_arch_reference/csv/vn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv")),
        _ => None,
    }
}

pub fn csv_rows_by_name(name: &str) -> Option<Vec<Vec<String>>> {
    let asset = csv_asset_by_name(name)?;
    let text = csv_text_by_name(name)?;
    Some(parse_csv_text(text, asset.delimiter))
}

pub fn csv_cell_by_name(name: &str, row_one_based: usize, column_one_based: usize) -> Option<String> {
    if row_one_based == 0 || column_one_based == 0 {
        return None;
    }
    let rows = csv_rows_by_name(name)?;
    rows.get(row_one_based - 1)?.get(column_one_based - 1).cloned()
}

pub fn parse_csv_text(text: &str, delimiter: CsvDelimiter) -> Vec<Vec<String>> {
    parse_csv_text_with_delimiter(text, delimiter.as_char())
}

pub fn parse_csv_text_with_delimiter(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            if in_quotes && chars.peek() == Some(&'"') {
                cell.push('"');
                chars.next();
            } else {
                in_quotes = !in_quotes;
            }
        } else if ch == delimiter && !in_quotes {
            row.push(cell);
            cell = String::new();
        } else if (ch == '\n' || ch == '\r') && !in_quotes {
            if ch == '\r' && chars.peek() == Some(&'\n') {
                chars.next();
            }
            row.push(cell);
            cell = String::new();
            rows.push(row);
            row = Vec::new();
        } else {
            cell.push(ch);
        }
    }

    if !cell.is_empty() || !row.is_empty() || text.ends_with(delimiter) {
        row.push(cell);
        rows.push(row);
    }

    rows
}

pub fn select_csv_rows_one_based(rows: &[Vec<String>], selected: &[usize]) -> Vec<Vec<String>> {
    selected
        .iter()
        .filter_map(|line| line.checked_sub(1).and_then(|index| rows.get(index)).cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_current_religion_csv() {
        let asset = csv_asset_by_name("religion.csv").unwrap();
        assert_eq!(asset.kind, CsvAssetKind::Religion);
        assert_eq!(asset.row_count, 1025);
        assert!(asset.max_columns > 700);
    }

    #[test]
    fn catalog_knows_kombi_meta_csv() {
        let asset = csv_asset_by_name("kombi-meta.csv").unwrap();
        assert_eq!(asset.kind, CsvAssetKind::KombiMeta);
        assert_eq!(asset.row_count, 262);
    }

    #[test]
    fn parser_handles_quoted_delimiters_and_newlines() {
        let parsed = parse_csv_text_with_delimiter("a;\"b;c\";d\n1;\"two\nlines\";3", ';');
        assert_eq!(parsed[0], vec!["a".to_string(), "b;c".to_string(), "d".to_string()]);
        assert_eq!(parsed[1][1], "two\nlines");
    }

    #[test]
    fn asset_parser_matches_static_metadata_for_smoke_files() {
        for name in ["religion.csv", "kombi-meta.csv", "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv"] {
            let asset = csv_asset_by_name(name).unwrap();
            let rows = csv_rows_by_name(name).unwrap();
            assert_eq!(rows.len(), asset.row_count, "{name}");
            assert_eq!(rows.iter().map(Vec::len).max().unwrap_or(0), asset.max_columns, "{name}");
        }
    }

    #[test]
    fn language_asset_with_required_columns_uses_synced_variant_after_stage62() {
        let localized = csv_asset_for_language_with_required_columns("religion.csv", CsvLanguage::English, &[493, 744]).unwrap();
        assert_eq!(localized.name, "en-religion.csv");
        assert_eq!(localized.language, CsvLanguage::English);
        assert!(csv_asset_supports_columns(localized, &[493, 744]));
        let base = csv_asset_for_language_with_required_columns("religion.csv", CsvLanguage::Base, &[493, 744]).unwrap();
        assert_eq!(base.name, "religion.csv");
        assert_eq!(base.language, CsvLanguage::Base);
    }

    #[test]
    fn language_aliases_match_python_language_parameter() {
        assert_eq!(CsvLanguage::from_language_value("english"), Some(CsvLanguage::English));
        assert_eq!(CsvLanguage::from_language_value("englisch"), Some(CsvLanguage::English));
        assert_eq!(CsvLanguage::from_language_value("deutsch"), Some(CsvLanguage::Base));
        assert_eq!(CsvLanguage::from_language_value("vietnamese"), Some(CsvLanguage::Vietnamese));
        assert_eq!(CsvLanguage::from_language_value("chinesisch"), Some(CsvLanguage::Chinese));
        assert_eq!(CsvLanguage::from_language_value("korean"), Some(CsvLanguage::Korean));
    }

    #[test]
    fn csv_language_from_cli_args_uses_last_valid_language_switch() {
        let args = ["reta", "-language=english", "-sprache=deutsch", "--language=chinese"];
        assert_eq!(csv_language_from_cli_args(&args), CsvLanguage::Chinese);
        assert_eq!(language_value_from_cli_arg("-language=english"), Some("english"));
        assert_eq!(language_value_from_cli_arg("--sprache=englisch"), Some("englisch"));
    }
}
