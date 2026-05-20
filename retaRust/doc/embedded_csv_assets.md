# Eingebettete CSV-Daten in der Runtime-Library

Die Runtime-Library bettet die kanonischen `csv/*.csv`-Dateien beim Kompilieren ein.
Das passiert in `build.rs`: die CSV-Dateien werden gescannt, als `include_bytes!`-Assets in
`OUT_DIR/reta_embedded_csv_assets.rs` generiert und von `src/shared/embedded_csv_assets.rs`
eingebunden.

Dadurch benötigen `rreta` und `retaPrompt` zur Laufzeit keinen `csv/`-Unterordner mehr im
aktuellen Arbeitsverzeichnis. Die Daten liegen in der gebauten Shared Library, also im normalen
Build in `libreta.so` und im Split-Build zusätzlich im Runtime-Pfad `libreta_runtime.so`.

Lade-Reihenfolge:

1. Wenn `RETA_CSV_PATH` gesetzt ist und die konkrete Datei dort existiert, wird diese Datei benutzt.
2. Sonst wird das eingebettete Asset aus der Library benutzt.
3. Falls ein Asset nicht eingebettet ist, bleibt der alte Dateisystem-Fallback aktiv.

Symlink- und Sprach-Aliasnamen wie `en-religion.csv`, `cn-religion.csv`, `kr-religion.csv`
und `vn-religion.csv` werden als Alias-Tabelle eingebettet und auf die kanonischen Assets
zurückgeführt. Dadurch wird der gleiche CSV-Inhalt nicht mehrfach in die `.so` kopiert. Die drei
speziellen Motive-Dateien für `cn`, `kr` und `vn` bleiben eigene Assets.

Die CSVs werden nicht als riesige Rust-Quelltext-Tabelle eingecheckt. Die Datenstruktur entsteht
beim Build und wird danach aus der `.so` gelesen. Geparst wird weiterhin lazy beim ersten Zugriff
und danach im bestehenden Cache gehalten.
