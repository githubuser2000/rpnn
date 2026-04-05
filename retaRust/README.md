# multibin grundstruk stricter

Dieses Paket bleibt bei:
- **einem Cargo.toml**
- **zwei Binaries**
  - `reta`
  - `grundStrukHtml`

## Was strenger geworden ist

### `grundStrukHtml`
Kein bloßer Platzhalter mehr, sondern Python-nähere Shared-Logik mit:
- `cmp_before`
- `cmpx`
- `sorted`
- `merge_dicts`
- `traverseHierarchy`
- `myprint`
- `grundstruk_html_from_i18n`

Datenbasis:
- `wahl15Words` direkt aus Python ausgewertet
- `grundstrukturen`-Name direkt eingebunden

Zusätzlich jetzt:
- `tools/compare_grundstruk.py`
  - echte Python-vs-Rust-Diff-Schleife
  - für `normal` und `blank`

### `reta`
Bleibt im selben Paket mit gemeinsamer Shared-Logik.
In diesem Paket weiter als Teilmenge:
- `paraNdataMatrix`: **120 Einträge**
- `kombiParaNdataMatrix`: **12 Schlüssel**
- `kombiParaNdataMatrix2`: **14 Schlüssel**

## Nächster harter Schritt

- Diffs von `tools/compare_grundstruk.py` abarbeiten
- Zeichen für Zeichen an Python annähern, statt umzubauen
