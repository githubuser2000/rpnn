# Stage 27 – Kategorien, Funktoren und natürliche Transformationen

Stage 27 baut direkt auf **Stage 26** auf. Es wurde nicht neu angefangen und es wurde kein Laufzeitverhalten bewusst geändert.

## Ausgangslage

Vor den Refactor-Stufen gab es keine eigene `reta_architecture`-Schicht. Die alte Architektur war hauptsächlich in großen, gemischten Dateien versteckt:

- `reta.py`
- `retaPrompt.py`
- `i18n/words.py`
- `libs/tableHandling.py`
- `libs/lib4tables_prepare.py`
- `libs/lib4tables_concat.py`
- `libs/lib4tables.py`

Nach Stage 26 sind Topologie, Prägarben, Garben, Morphismen, universelle Gluing-Knoten, Tabellen-Runtime und Tabellenzustandssektionen bereits explizit vorhanden.

## Geplanter Schritt

Der nächste geplante Paradigmenwechsel war nicht eine weitere große Verhaltensänderung, sondern eine kategoriale Metaschicht:

1. **Kategorien** für die vorhandenen Architekturgebiete benennen.
2. **Funktoren** zwischen diesen Kategorien benennen.
3. **Natürliche Transformationen** ergänzen, weil die Refactor-Invariante nicht nur heißt „A wird auf B abgebildet“, sondern „alternative Architekturpfade kommutieren beobachtbar“.

Funktoren allein reichen nicht. Beispiele:

- Raw CLI/Prompt → kanonische Parametersemantik muss mit Kontext-Restriktion verträglich sein.
- Prägarben → Garben muss mit lokalem Einschränken und globalem Gluing verträglich sein.
- Legacy-Fassade → Architektur-Fassade muss für repräsentative Kommandos dieselbe Ausgabe liefern.
- Mutable `Tables` → explizite `TableStateSections` muss dieselben mutierbaren Objekte spiegeln.

Das sind natürliche Transformationsbedingungen.

## Neue Schicht

### `reta_architecture/category_theory.py`

Neue, bewusst leichte Metadaten-Schicht:

- `CategoryObjectSpec`
- `CategoryMorphismSpec`
- `CategorySpec`
- `FunctorSpec`
- `NaturalTransformationSpec`
- `ParadigmTermSpec`
- `Stage27ArchitecturePlan`
- `CategoryTheoryBundle`
- `bootstrap_category_theory(...)`

Diese Schicht macht die vorhandene Architektur inspizierbar als:

```text
Topologie
Morphismen
universelle Eigenschaften
Prägarben
Garben
math Kategorien
Funktoren
natürliche Transformationen
```

Sie ist keine schwere Framework-Abstraktion und ersetzt keine Runtime-Funktionen. Sie benennt die Struktur, die bereits durch die früheren Stages entstanden ist.

## Registrierte Kategorien

Stage 27 registriert:

- `OpenRetaContextCategory`
- `LocalSectionCategory`
- `CanonicalSemanticSheafCategory`
- `UniversalConstructionCategory`
- `TableSectionCategory`
- `GeneratedColumnEndomorphismCategory`
- `OutputFormatCategory`
- `LegacyFacadeCategory`

## Registrierte Funktoren

Stage 27 registriert unter anderem:

- `SchemaToTopologyFunctor`
- `RawCommandPresheafFunctor`
- `CanonicalParameterSheafFunctor`
- `LocalDataPresheafFunctor`
- `GluedSemanticSheafFunctor`
- `TableGenerationGluingFunctor`
- `GeneratedColumnEndofunctorFamily`
- `OutputRenderingFunctorFamily`
- `NormalizedOutputFunctor`
- `LegacyRuntimeFunctor`
- `ArchitectureRuntimeFunctor`
- `MutableTableRuntimeFunctor`
- `ExplicitTableStateFunctor`

## Registrierte natürliche Transformationen

Stage 27 registriert:

- `RawToCanonicalParameterTransformation`
- `PresheafToSheafGluingTransformation`
- `TableGenerationGluingTransformation`
- `GeneratedColumnsSheafSyncTransformation`
- `TableRuntimeToStateSectionsTransformation`
- `RenderedOutputNormalizationTransformation`
- `LegacyToArchitectureTransformation`

## Architektur-Integration

`RetaArchitecture` besitzt jetzt zusätzlich:

- `category_theory`

Neue Methode:

```python
RetaArchitecture.bootstrap_category_theory(...)
```

`snapshot()` enthält jetzt:

```python
"category_theory": ...
```

## Probe-Werkzeug

Neues Probe-Kommando:

```bash
python -B -S reta_architecture_probe_py.py category-theory-json
```

Das JSON enthält:

- Paradigma
- Anzahl Kategorien/Funktoren/natürlicher Transformationen
- alle Kategorie-Spezifikationen
- alle Funktor-Spezifikationen
- alle natürlichen Transformationen
- Plan: geplant / jetzt umgesetzt / bereits umgesetzt

## Paket-Manifest

`reta_architecture/package_integrity.py` behandelt jetzt zusätzlich als Pflichtdatei:

- `reta_architecture/category_theory.py`

## Tests

Neue Regressionen prüfen:

- `CategoryTheoryBundle` ist explizit vorhanden.
- `RetaArchitecture.snapshot()` enthält `category_theory`.
- Kategorien, Funktoren und natürliche Transformationen sind registriert.
- Das neue Probe-Kommando ist ausführbar.
- Das Manifest enthält `reta_architecture/category_theory.py`.

## Geprüft

- `py_compile`: OK
- `reta_architecture_probe_py.py category-theory-json`: OK
- `tests.test_architecture_refactor`: **47 Tests, OK**
- `tests.test_command_parity`: **1 Paritätsmatrix-Test, OK**
- volle Unittest-Discovery: **48 Tests, OK**

Die Paritätsmatrix nutzt wieder das bereitgestellte Originalarchiv:

```text
/mnt/data/reta.todel.zip
```

Geprüfte repräsentative Fälle:

- Shell `Religionen/sternpolygon`
- Markdown `Religionen/sternpolygon`
- HTML `Religionen/sternpolygon` nach Normalisierung
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`

## Was bereits vor Stage 27 umgesetzt war

| Paradigma | Bereits vorhandene Schicht |
|---|---|
| Topologie | `reta_architecture/topology.py` |
| Prägarben | `reta_architecture/presheaves.py` |
| Garben | `reta_architecture/sheaves.py` |
| Morphismen | `morphisms.py`, `row_filtering.py`, `generated_columns.py`, `concat_csv.py`, `combi_join.py`, `table_output.py`, `table_wrapping.py`, `number_theory.py` |
| Universelle Eigenschaften / Gluing | `universal.py`, `table_generation.py`, `program_workflow.py` |
| Globale Tabellensektion | `table_runtime.py` |
| Explizite Tabellenzustandssektionen | `table_state.py` |

## Architekturgewinn

Vor Stage 27 war die Architektur bereits mathematisch motiviert, aber die kategoriale Ebene war nur implizit.

Nach Stage 27 ist die Struktur explizit:

```text
Open(Context)^op
    -> lokale Prägarben-Sektionen
    -> geklebte Garben
    -> globale Tabellen-Sektion
    -> Renderer-/Output-Sektionen
```

und zusätzlich:

```text
LegacyRuntimeFunctor
    => ArchitectureRuntimeFunctor
```

als natürliche Transformation für beobachtbare Kompatibilität.

Das ist der entscheidende Punkt: Die neue Architektur beschreibt nicht nur, **welche** Schichten existieren, sondern auch, **welche Pfade kommutieren sollen**.
