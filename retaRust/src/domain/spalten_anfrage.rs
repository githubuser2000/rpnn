//! overwrite scaffold
//! Ziel dieses Moduls:
//! - String-Normalisierung entfernen
//! - Ober-/Unterkategorien typisiert modellieren
//! - exaktes Python-kompatibles Parsing ermöglichen
//! - Fehlermeldungen deterministisch halten

// TODO: echten Inhalt aus dem aktuellen Repo-Stand einfügen.
// Erwartete Verantwortung:
// - Enums für StandardOberkategorie / Unterkategorien
// - parse(ober, unter) -> Result<SpaltenAnfrage, String>
// - ober_unter_cli_pair()
// - kein implizites De-Underscoring / Lowercasing / Alias-Magie
