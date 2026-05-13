from __future__ import annotations

"""Symbolic category/functor/natural-transformation layer for the Reta architecture.

Stage 27 does not change runtime behaviour.  It makes the mathematical
architecture that the earlier stages already introduced explicit and
inspectable:

* topology: open Reta contexts and refinements
* presheaves/sheaves: local sections and glued global semantics
* morphisms/universal properties: transitions and gluing nodes
* categories/functors/natural transformations: structure-preserving maps between
  the above layers and commutative compatibility requirements

The classes in this module are deliberately lightweight metadata objects.  They
name the categorical structure of the current Python architecture without
turning every runtime function into a heavy framework object.
"""

from dataclasses import dataclass
from typing import Mapping, Sequence


@dataclass(frozen=True)
class CategoryObjectSpec:
    """Named object inside a symbolic architecture category."""

    name: str
    code_owner: str
    role: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "code_owner": self.code_owner,
            "role": self.role,
        }


@dataclass(frozen=True)
class CategoryMorphismSpec:
    """Named morphism inside a symbolic architecture category."""

    name: str
    source: str
    target: str
    code_owner: str
    role: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "source": self.source,
            "target": self.target,
            "code_owner": self.code_owner,
            "role": self.role,
        }


@dataclass(frozen=True)
class CategorySpec:
    """Symbolic category used by the Reta architecture."""

    name: str
    description: str
    objects: Sequence[CategoryObjectSpec]
    morphisms: Sequence[CategoryMorphismSpec]
    implemented_by: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "description": self.description,
            "objects": [item.snapshot() for item in self.objects],
            "morphisms": [item.snapshot() for item in self.morphisms],
            "implemented_by": list(self.implemented_by),
        }


@dataclass(frozen=True)
class FunctorSpec:
    """Symbolic functor between architecture categories."""

    name: str
    source_category: str
    target_category: str
    variance: str
    object_map: Mapping[str, str]
    morphism_map: Mapping[str, str]
    code_owner: str
    description: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "source_category": self.source_category,
            "target_category": self.target_category,
            "variance": self.variance,
            "object_map": dict(self.object_map),
            "morphism_map": dict(self.morphism_map),
            "code_owner": self.code_owner,
            "description": self.description,
        }


@dataclass(frozen=True)
class NaturalTransformationSpec:
    """Symbolic natural transformation between two architecture functors."""

    name: str
    source_functor: str
    target_functor: str
    components: Mapping[str, str]
    naturality_condition: str
    code_owner: str
    description: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "source_functor": self.source_functor,
            "target_functor": self.target_functor,
            "components": dict(self.components),
            "naturality_condition": self.naturality_condition,
            "code_owner": self.code_owner,
            "description": self.description,
        }


@dataclass(frozen=True)
class ParadigmTermSpec:
    """Project-local interpretation of one mathematical architecture term."""

    term: str
    meaning: str
    implemented_by: Sequence[str]
    stage_status: str

    def snapshot(self) -> dict:
        return {
            "term": self.term,
            "meaning": self.meaning,
            "implemented_by": list(self.implemented_by),
            "stage_status": self.stage_status,
        }


@dataclass(frozen=True)
class Stage27ArchitecturePlan:
    """Readable bridge between the previous plan and the Stage-27 implementation."""

    planned_before_stage_27: Sequence[str]
    implemented_in_stage_27: Sequence[str]
    already_implemented_before_stage_27: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_before_stage_27": list(self.planned_before_stage_27),
            "implemented_in_stage_27": list(self.implemented_in_stage_27),
            "already_implemented_before_stage_27": list(self.already_implemented_before_stage_27),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class CategoryTheoryBundle:
    """Inspectable Stage-27 category-theory layer."""

    categories: Sequence[CategorySpec]
    functors: Sequence[FunctorSpec]
    natural_transformations: Sequence[NaturalTransformationSpec]
    paradigm_terms: Sequence[ParadigmTermSpec]
    plan: Stage27ArchitecturePlan

    def category_named(self, name: str) -> CategorySpec:
        return _find_by_name(self.categories, name, "category")

    def functor_named(self, name: str) -> FunctorSpec:
        return _find_by_name(self.functors, name, "functor")

    def natural_transformation_named(self, name: str) -> NaturalTransformationSpec:
        return _find_by_name(self.natural_transformations, name, "natural transformation")

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
            ],
            "counts": {
                "categories": len(self.categories),
                "functors": len(self.functors),
                "natural_transformations": len(self.natural_transformations),
                "paradigm_terms": len(self.paradigm_terms),
            },
            "categories": [item.snapshot() for item in self.categories],
            "functors": [item.snapshot() for item in self.functors],
            "natural_transformations": [item.snapshot() for item in self.natural_transformations],
            "paradigm_terms": [item.snapshot() for item in self.paradigm_terms],
            "plan": self.plan.snapshot(),
        }


def _find_by_name(items, name: str, kind: str):
    for item in items:
        if item.name == name:
            return item
    raise KeyError(f"Unknown {kind}: {name}")


def _obj(name: str, code_owner: str, role: str) -> CategoryObjectSpec:
    return CategoryObjectSpec(name=name, code_owner=code_owner, role=role)


def _mor(name: str, source: str, target: str, code_owner: str, role: str) -> CategoryMorphismSpec:
    return CategoryMorphismSpec(name=name, source=source, target=target, code_owner=code_owner, role=role)


def _categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="OpenRetaContextCategory",
            description="Kategorie der offenen Reta-Kontexte; Morphismen sind Verfeinerungen, Einschränkungen und Basisüberdeckungen.",
            objects=(
                _obj("ContextSelection", "reta_architecture.topology.ContextSelection", "symbolische offene Kontextmenge"),
                _obj("RetaContextTopology", "reta_architecture.topology.RetaContextTopology", "Topologie über Sprache, Parametern, Zeilen, Ausgabearten und Scopes"),
                _obj("ContextCover", "reta_architecture.topology.RetaContextTopology.cover_for_main", "Basisüberdeckung eines Hauptparameter-Kontexts"),
            ),
            morphisms=(
                _mor("refine", "ContextSelection", "ContextSelection", "ContextSelection.refine", "Kontextverfeinerung"),
                _mor("open_for", "RetaContextTopology", "ContextSelection", "RetaContextTopology.open_for", "Basisöffnung erzeugen"),
                _mor("cover_for_main", "RetaContextTopology", "ContextCover", "RetaContextTopology.cover_for_main", "lokale Überdeckung erzeugen"),
            ),
            implemented_by=("reta_architecture/topology.py",),
        ),
        CategorySpec(
            name="LocalSectionCategory",
            description="Kategorie lokaler Rohsektionen; Morphismen sind Restriktionen entlang kleinerer Kontexte.",
            objects=(
                _obj("LocalSection", "reta_architecture.presheaves.LocalSection", "lokale CSV-/Übersetzungs-/Asset-/Prompt-Sektion"),
                _obj("FilesystemPresheaf", "reta_architecture.presheaves.FilesystemPresheaf", "Prägarbe lokaler Dateien"),
                _obj("PromptStatePresheaf", "reta_architecture.presheaves.PromptStatePresheaf", "Prägarbe lokaler Prompt-Zustände"),
            ),
            morphisms=(
                _mor("add_section", "LocalSection", "FilesystemPresheaf", "Presheaf.add_section", "lokale Sektion registrieren"),
                _mor("restrict", "FilesystemPresheaf", "LocalSection", "Presheaf.restrict", "Sektion auf Kontext einschränken"),
                _mor("update_prompt_state", "PromptStatePresheaf", "LocalSection", "PromptStatePresheaf.update", "Prompt-Rohsektion aktualisieren"),
            ),
            implemented_by=("reta_architecture/presheaves.py",),
        ),
        CategorySpec(
            name="CanonicalSemanticSheafCategory",
            description="Kategorie geklebter kanonischer Semantik; Morphismen lösen Aliasnamen, Parameterpaare und Spaltenmengen auf.",
            objects=(
                _obj("ParameterSemanticsSheaf", "reta_architecture.sheaves.ParameterSemanticsSheaf", "kanonische Parametersemantik"),
                _obj("GeneratedColumnsSheaf", "reta_architecture.sheaves.GeneratedColumnsSheaf", "Garbe generierter Spaltenmetadaten"),
                _obj("TableOutputSheaf", "reta_architecture.sheaves.TableOutputSheaf", "Garbe synchronisierter Ausgabe-Sektionen"),
                _obj("HtmlReferenceSheaf", "reta_architecture.sheaves.HtmlReferenceSheaf", "Garbe der HTML-Referenzen"),
            ),
            morphisms=(
                _mor("canonicalize_pair", "ParameterSemanticsSheaf", "CanonicalParameterPair", "ParameterSemanticsSheaf.canonicalize_pair", "Aliasauflösung zu kanonischem Paar"),
                _mor("column_numbers_for_pair", "CanonicalParameterPair", "ColumnNumberSet", "ParameterSemanticsSheaf.column_numbers_for_pair", "kanonisches Paar zu Spaltenmenge"),
                _mor("sync_from_tables", "Tables", "GeneratedColumnsSheaf", "GeneratedColumnsSheaf.sync_from_tables", "globale Tabellen-Metadaten in Garbe spiegeln"),
            ),
            implemented_by=("reta_architecture/sheaves.py", "reta_architecture/semantics_builder.py"),
        ),
        CategorySpec(
            name="UniversalConstructionCategory",
            description="Kategorie der Gluing-/Normalisierungs-Knoten, die lokale Sektionen zu globaler Semantik zusammenführen.",
            objects=(
                _obj("ParameterDictionaryDiagram", "reta_architecture.universal.merge_parameter_dicts", "Diagramm lokaler Parameter-Dictionaries"),
                _obj("ColumnBucketDiagram", "reta_architecture.universal.normalize_column_buckets", "Diagramm positiver/negativer Spaltenauswahl"),
                _obj("TableSyncDiagram", "reta_architecture.universal.UniversalBundle.sync_tables", "Diagramm globaler Tabellen-Synchronisierung"),
            ),
            morphisms=(
                _mor("merge_parameter_dicts", "ParameterDictionaryDiagram", "ParameterSemanticsSheaf", "reta_architecture.universal.merge_parameter_dicts", "pushout-artiges Zusammenkleben lokaler Parameterdaten"),
                _mor("normalize_column_buckets", "ColumnBucketDiagram", "NormalizedColumnBuckets", "reta_architecture.universal.normalize_column_buckets", "kanonische Differenz-/Normalisierungskonstruktion"),
                _mor("sync_tables", "Tables", "SheafBundle", "UniversalBundle.sync_tables", "globale Tabellen-Sektion in Garben synchronisieren"),
            ),
            implemented_by=("reta_architecture/universal.py", "reta_architecture/table_generation.py", "reta_architecture/program_workflow.py"),
        ),
        CategorySpec(
            name="TableSectionCategory",
            description="Kategorie globaler Tabellen- und Tabellenzustandssektionen; Morphismen sind Prepare-, Concat-, Join-, Zeilenfilter- und Output-Übergänge.",
            objects=(
                _obj("Tables", "reta_architecture.table_runtime.Tables", "mutable globale Tabellensektion"),
                _obj("TableStateSections", "reta_architecture.table_state.TableStateSections", "explizite Zustandssektionen der globalen Tabelle"),
                _obj("TablePreparationBundle", "reta_architecture.table_preparation.TablePreparationBundle", "Prepare-Morphismen"),
                _obj("TableOutput", "reta_architecture.table_output.TableOutput", "Renderer-naher Output-Morphismus"),
            ),
            morphisms=(
                _mor("prepare_output_table", "Tables", "Tables", "reta_architecture.table_preparation.prepare_output_table", "Tabellen-Prepare-Morphismus"),
                _mor("filter_original_lines", "ParameterSection", "RowSet", "reta_architecture.row_filtering.filter_original_lines", "Zeilenfilter-Morphismus"),
                _mor("readConcatCsv", "LocalCsvSection", "Tables", "reta_architecture.concat_csv.readConcatCsv", "CSV-Prägarbensektion in globale Tabelle kleben"),
                _mor("render_table_output", "Tables", "RenderedOutput", "reta_architecture.table_output.TableOutput", "Tabellensektion rendern"),
            ),
            implemented_by=(
                "reta_architecture/table_runtime.py",
                "reta_architecture/table_state.py",
                "reta_architecture/table_preparation.py",
                "reta_architecture/row_filtering.py",
                "reta_architecture/concat_csv.py",
                "reta_architecture/table_output.py",
            ),
        ),
        CategorySpec(
            name="GeneratedColumnEndomorphismCategory",
            description="Kategorie der generierten Spalten; Morphismen sind Endomorphismen der globalen Tabellensektion.",
            objects=(
                _obj("GeneratedColumnSpec", "reta_architecture.generated_columns.GeneratedColumnSpec", "Beschreibung eines generierten Spaltenmorphismus"),
                _obj("GeneratedColumnRegistry", "reta_architecture.generated_columns.GeneratedColumnRegistry", "Registry der Tabellen-Endomorphismen"),
                _obj("GeneratedColumnSection", "reta_architecture.table_state.GeneratedColumnSection", "Zustandssektion generierter Spalten"),
            ),
            morphisms=(
                _mor("concat_love_polygon", "Tables", "Tables", "reta_architecture.generated_columns.concat_love_polygon", "generiert Love-/Polygon-Spalten"),
                _mor("concat_modallogik", "Tables", "Tables", "reta_architecture.generated_columns.concat_modallogik", "generiert Modallogik-Spalten"),
                _mor("create_spalte_gestirn", "Tables", "Tables", "reta_architecture.generated_columns.create_spalte_gestirn", "generiert Gestirn-Spalte"),
            ),
            implemented_by=("reta_architecture/generated_columns.py", "reta_architecture/table_state.py"),
        ),
        CategorySpec(
            name="OutputFormatCategory",
            description="Kategorie konkreter Ausgabeformate und normalisierbarer Output-Sektionen.",
            objects=(
                _obj("OutputSyntax", "reta_architecture.output_syntax.OutputSyntax", "abstrakte Ausgabesyntax"),
                _obj("OutputModeSpec", "reta_architecture.output_semantics.OutputModeSpec", "semantische Ausgabeart"),
                _obj("RenderedOutput", "reta_architecture.table_output.TableOutput", "gerenderte Ausgabe"),
                _obj("NormalizedOutput", "tests.test_command_parity", "vergleichbare normalisierte Ausgabe"),
            ),
            morphisms=(
                _mor("apply_output_mode", "Tables", "OutputModeApplication", "reta_architecture.output_semantics.RetaOutputSemantics.apply_mode_to_tables", "Ausgabemodus auf Tabelle anwenden"),
                _mor("render", "Tables", "RenderedOutput", "reta_architecture.table_output.TableOutput", "globale Tabelle rendern"),
                _mor("normalize_for_parity", "RenderedOutput", "NormalizedOutput", "tests.test_command_parity", "Output für Paritätstests normalisieren"),
            ),
            implemented_by=("reta_architecture/output_syntax.py", "reta_architecture/output_semantics.py", "reta_architecture/table_output.py"),
        ),
        CategorySpec(
            name="LegacyFacadeCategory",
            description="Kategorie alter Import-/Aufrufpfade; Morphismen sind Kompatibilitätsdelegationen in die Architekturschicht.",
            objects=(
                _obj("reta.py Program", "reta.py", "historischer CLI-Einstieg"),
                _obj("libs.tableHandling", "libs/tableHandling.py", "alte Tabellen-Fassade"),
                _obj("libs.lib4tables_prepare", "libs/lib4tables_prepare.py", "alte Prepare-Fassade"),
                _obj("libs.lib4tables_concat", "libs/lib4tables_concat.py", "alte Concat-Fassade"),
            ),
            morphisms=(
                _mor("bootstrap_program", "reta.py Program", "RetaArchitecture", "reta.Program.__init__", "Programmlauf über Architektur-Fassade erzeugen"),
                _mor("tableHandling_reexport", "libs.tableHandling", "TableSectionCategory", "libs/tableHandling.py", "alte Tabellenimporte delegieren"),
                _mor("prepare_delegation", "libs.lib4tables_prepare", "TableSectionCategory", "libs/lib4tables_prepare.py", "alte Prepare-Methoden delegieren"),
                _mor("concat_delegation", "libs.lib4tables_concat", "GeneratedColumnEndomorphismCategory", "libs/lib4tables_concat.py", "alte Concat-Methoden delegieren"),
            ),
            implemented_by=("reta.py", "libs/tableHandling.py", "libs/lib4tables_prepare.py", "libs/lib4tables_concat.py"),
        ),

        CategorySpec(
            name="CommutativeArchitectureContractCategory",
            description="Kategorie der Stage-29-Architekturverträge; Objekte sind kommutierende Diagramme, Kapselgrenzen und Refactor-Gesetze.",
            objects=(
                _obj("CommutativeDiagramSpec", "reta_architecture.architecture_contracts.CommutativeDiagramSpec", "prüfbarer kommutierender Architekturpfad"),
                _obj("CapsuleContractSpec", "reta_architecture.architecture_contracts.CapsuleContractSpec", "Grenzvertrag einer Architektur-Kapsel"),
                _obj("RefactorLawSpec", "reta_architecture.architecture_contracts.RefactorLawSpec", "menschenlesbare Refactor-Invariante"),
                _obj("ArchitectureContractsBundle", "reta_architecture.architecture_contracts.ArchitectureContractsBundle", "gebündelte Stage-29-Vertragsmetadaten"),
            ),
            morphisms=(
                _mor("bootstrap_architecture_contracts", "CategoryTheoryBundle", "ArchitectureContractsBundle", "reta_architecture.architecture_contracts.bootstrap_architecture_contracts", "Vertragsbundle aus Kategorien und Kapselkarte erzeugen"),
                _mor("validate_contract_references", "ArchitectureContractsBundle", "ContractValidationSpec", "reta_architecture.architecture_contracts._validation", "Referenzen gegen bekannte Kategorien/Funktoren/Transformationen prüfen"),
                _mor("render_contract_diagram", "ArchitectureContractsBundle", "RenderedContractDiagram", "reta_architecture_probe_py.py architecture-contracts-md", "Vertragsdiagramm als Markdown/Mermaid ausgeben"),
            ),
            implemented_by=("reta_architecture/architecture_contracts.py",),
        ),
        CategorySpec(
            name="ArchitectureValidationCategory",
            description="Kategorie der Stage-31-Validierung; Objekte sind Checks, Layer, Summary und das ausführbare Architekturvalidierungsbundle.",
            objects=(
                _obj("CategoryTheoryBundle", "reta_architecture.category_theory.CategoryTheoryBundle", "Quelle der Kategorien/Funktoren/natürlichen Transformationen"),
                _obj("ArchitectureMapBundle", "reta_architecture.architecture_map.ArchitectureMapBundle", "Quelle der Kapseln, Flüsse und Stage-Schritte"),
                _obj("ArchitectureContractsBundle", "reta_architecture.architecture_contracts.ArchitectureContractsBundle", "Quelle der kommutierenden Diagramme und Gesetze"),
                _obj("ArchitectureWitnessBundle", "reta_architecture.architecture_witnesses.ArchitectureWitnessBundle", "Quelle der Repository-Witnesses"),
                _obj("ArchitectureValidationBundle", "reta_architecture.architecture_validation.ArchitectureValidationBundle", "ausführbarer Stage-31-Audit"),
                _obj("ArchitectureValidationCheckSpec", "reta_architecture.architecture_validation.ArchitectureValidationCheckSpec", "einzelner Architekturcheck"),
                _obj("ArchitectureValidationSummarySpec", "reta_architecture.architecture_validation.ArchitectureValidationSummarySpec", "zusammengefasster Validierungsstatus"),
            ),
            morphisms=(
                _mor("bootstrap_architecture_validation", "ArchitectureWitnessBundle", "ArchitectureValidationBundle", "reta_architecture.architecture_validation.bootstrap_architecture_validation", "Kategorien, Karte, Verträge, Witnesses und Paketbaum zu einem Audit komponieren"),
                _mor("validate_stage31_references", "ArchitectureValidationBundle", "ArchitectureValidationSummarySpec", "ArchitectureValidationBundle.snapshot", "Stage-31-Prüfergebnis erzeugen"),
                _mor("render_validation_diagram", "ArchitectureValidationBundle", "RenderedValidationDiagram", "reta_architecture_probe_py.py architecture-validation-md", "Validierungsdiagramm als Markdown/Mermaid ausgeben"),
            ),
            implemented_by=("reta_architecture/architecture_validation.py",),
        ),
    )


def _functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec(
            name="SchemaToTopologyFunctor",
            source_category="CanonicalSemanticSheafCategory",
            target_category="OpenRetaContextCategory",
            variance="covariant",
            object_map={
                "RetaContextSchema": "RetaContextTopology",
                "ParameterFamilies": "ContextSelection",
            },
            morphism_map={
                "schema_family_membership": "open_for",
                "main_parameter_cover": "cover_for_main",
            },
            code_owner="RetaContextTopology.from_schema",
            description="Erzeugt aus der kanonischen Schema-/Words-Struktur die Topologie der offenen Reta-Kontexte.",
        ),
        FunctorSpec(
            name="RawCommandPresheafFunctor",
            source_category="OpenRetaContextCategory",
            target_category="LocalSectionCategory",
            variance="contravariant",
            object_map={
                "ContextSelection": "LocalSection",
                "PromptText": "PromptStatePresheaf",
            },
            morphism_map={
                "refine": "restrict",
                "open_for": "add_section",
            },
            code_owner="reta_architecture.presheaves.PromptStatePresheaf",
            description="Ordnet jedem offenen Kontext lokale, noch nicht kanonisierte Prompt-/Datei-Sektionen zu.",
        ),
        FunctorSpec(
            name="CanonicalParameterSheafFunctor",
            source_category="OpenRetaContextCategory",
            target_category="CanonicalSemanticSheafCategory",
            variance="contravariant",
            object_map={
                "ContextSelection": "ParameterSemanticsSheaf",
                "ContextCover": "ParameterSemanticsSheaf",
            },
            morphism_map={
                "refine": "canonicalize_pair",
                "cover_for_main": "column_numbers_for_pair",
            },
            code_owner="reta_architecture.sheaves.ParameterSemanticsSheaf",
            description="Ordnet offenen Kontexten die geklebte kanonische Parametersemantik zu.",
        ),
        FunctorSpec(
            name="LocalDataPresheafFunctor",
            source_category="OpenRetaContextCategory",
            target_category="LocalSectionCategory",
            variance="contravariant",
            object_map={
                "ContextSelection": "FilesystemPresheaf",
                "CsvContext": "LocalSection",
            },
            morphism_map={
                "refine": "restrict",
                "cover_for_main": "add_section",
            },
            code_owner="reta_architecture.presheaves.FilesystemPresheaf",
            description="Modelliert CSVs, Übersetzungen und Assets als lokale Prägarben-Sektionen.",
        ),
        FunctorSpec(
            name="GluedSemanticSheafFunctor",
            source_category="OpenRetaContextCategory",
            target_category="CanonicalSemanticSheafCategory",
            variance="contravariant",
            object_map={
                "ContextSelection": "SheafBundle",
                "CsvContext": "ParameterSemanticsSheaf",
            },
            morphism_map={
                "refine": "sync_program_semantics",
                "cover_for_main": "merge_parameter_dicts",
            },
            code_owner="reta_architecture.sheaves.SheafBundle.from_repo",
            description="Klebt lokale Daten zu global verwendbarer Semantik.",
        ),
        FunctorSpec(
            name="TableGenerationGluingFunctor",
            source_category="UniversalConstructionCategory",
            target_category="TableSectionCategory",
            variance="covariant",
            object_map={
                "ParameterDictionaryDiagram": "Tables",
                "ColumnBucketDiagram": "TableStateSections",
            },
            morphism_map={
                "merge_parameter_dicts": "create_tables",
                "normalize_column_buckets": "prepare_output_table",
            },
            code_owner="reta_architecture.table_generation.bootstrap_table_generation",
            description="Führt Parameter- und Spaltenauswahl über universelles Gluing zur globalen Tabellen-Sektion.",
        ),
        FunctorSpec(
            name="GeneratedColumnEndofunctorFamily",
            source_category="TableSectionCategory",
            target_category="TableSectionCategory",
            variance="covariant",
            object_map={
                "Tables": "Tables",
                "GeneratedColumnSection": "GeneratedColumnSection",
            },
            morphism_map={
                "concat_love_polygon": "concat_love_polygon",
                "concat_modallogik": "concat_modallogik",
                "create_spalte_gestirn": "create_spalte_gestirn",
            },
            code_owner="reta_architecture.generated_columns.GeneratedColumnRegistry",
            description="Familie von Tabellen-Endofunktoren, die abgeleitete Spalten erzeugen.",
        ),
        FunctorSpec(
            name="OutputRenderingFunctorFamily",
            source_category="TableSectionCategory",
            target_category="OutputFormatCategory",
            variance="covariant",
            object_map={
                "Tables": "RenderedOutput",
                "TableStateSections": "OutputModeSpec",
            },
            morphism_map={
                "prepare_output_table": "render",
                "apply_output_mode": "apply_output_mode",
            },
            code_owner="reta_architecture.table_output.TableOutput",
            description="Renderer-Funktoren von globalen Tabellensektionen in konkrete Ausgabeformate.",
        ),
        FunctorSpec(
            name="NormalizedOutputFunctor",
            source_category="OutputFormatCategory",
            target_category="OutputFormatCategory",
            variance="covariant",
            object_map={
                "RenderedOutput": "NormalizedOutput",
                "OutputSyntax": "OutputSyntax",
            },
            morphism_map={
                "render": "normalize_for_parity",
                "apply_output_mode": "apply_output_mode",
            },
            code_owner="tests.test_command_parity",
            description="Vergleichsfunktor, der verschiedene Renderer-Ausgaben in paritätsfähige Normalformen überführt.",
        ),
        FunctorSpec(
            name="LegacyRuntimeFunctor",
            source_category="LegacyFacadeCategory",
            target_category="TableSectionCategory",
            variance="covariant",
            object_map={
                "libs.tableHandling": "Tables",
                "libs.lib4tables_prepare": "TablePreparationBundle",
                "libs.lib4tables_concat": "GeneratedColumnRegistry",
            },
            morphism_map={
                "tableHandling_reexport": "create_tables",
                "prepare_delegation": "prepare_output_table",
                "concat_delegation": "GeneratedColumnEndofunctorFamily",
            },
            code_owner="libs compatibility facades",
            description="Beschreibt, wie alte Importpfade auf die neue Tabellenarchitektur abgebildet werden.",
        ),
        FunctorSpec(
            name="ArchitectureRuntimeFunctor",
            source_category="LegacyFacadeCategory",
            target_category="TableSectionCategory",
            variance="covariant",
            object_map={
                "reta.py Program": "RetaArchitecture",
                "libs.tableHandling": "Tables",
                "libs.lib4tables_prepare": "TablePreparationBundle",
                "libs.lib4tables_concat": "GeneratedColumnRegistry",
            },
            morphism_map={
                "bootstrap_program": "bootstrap_table_runtime",
                "tableHandling_reexport": "create_tables",
                "prepare_delegation": "bootstrap_table_preparation",
                "concat_delegation": "bootstrap_generated_columns",
            },
            code_owner="reta_architecture.facade.RetaArchitecture",
            description="Der neue strukturierte Runtime-Pfad über die Architektur-Fassade.",
        ),
        FunctorSpec(
            name="MutableTableRuntimeFunctor",
            source_category="TableSectionCategory",
            target_category="TableSectionCategory",
            variance="covariant",
            object_map={
                "Tables": "Tables",
                "TableOutput": "TableOutput",
            },
            morphism_map={
                "prepare_output_table": "prepare_output_table",
                "render_table_output": "render_table_output",
            },
            code_owner="reta_architecture.table_runtime.Tables",
            description="Historisch-mutable Sicht der globalen Tabellensektion.",
        ),
        FunctorSpec(
            name="ExplicitTableStateFunctor",
            source_category="TableSectionCategory",
            target_category="TableSectionCategory",
            variance="covariant",
            object_map={
                "Tables": "TableStateSections",
                "GeneratedColumnSection": "GeneratedColumnSection",
            },
            morphism_map={
                "prepare_output_table": "TableStateSections.snapshot",
                "render_table_output": "TableStateSections.snapshot",
            },
            code_owner="reta_architecture.table_state.TableStateBundle",
            description="Projiziert die globale Runtime in explizite, inspizierbare Tabellenzustandssektionen.",
        ),

        FunctorSpec(
            name="CategoryTheoryToContractFunctor",
            source_category="CommutativeArchitectureContractCategory",
            target_category="CommutativeArchitectureContractCategory",
            variance="covariant",
            object_map={
                "CategoryTheoryBundle": "ArchitectureContractsBundle",
                "NaturalTransformationSpec": "CommutativeDiagramSpec",
            },
            morphism_map={
                "bootstrap_category_theory": "bootstrap_architecture_contracts",
                "natural_transformation_named": "diagram_named",
            },
            code_owner="reta_architecture.architecture_contracts.bootstrap_architecture_contracts",
            description="Hebt die Stage-27-Kategorien/Funktoren/natürlichen Transformationen in Stage-29-Vertragsdiagramme.",
        ),
        FunctorSpec(
            name="ArchitectureMapToContractFunctor",
            source_category="CommutativeArchitectureContractCategory",
            target_category="CommutativeArchitectureContractCategory",
            variance="covariant",
            object_map={
                "ArchitectureMapBundle": "ArchitectureContractsBundle",
                "ArchitectureCapsuleSpec": "CapsuleContractSpec",
                "ArchitectureFlowSpec": "CommutativeDiagramSpec",
            },
            morphism_map={
                "bootstrap_architecture_map": "bootstrap_architecture_contracts",
                "capsule_named": "capsule_contract_named",
            },
            code_owner="reta_architecture.architecture_contracts.bootstrap_architecture_contracts",
            description="Macht aus der Stage-28-Kapselkarte explizite Kapselverträge und kommutierende Diagramme.",
        ),
        FunctorSpec(
            name="ContractToValidationFunctor",
            source_category="CommutativeArchitectureContractCategory",
            target_category="ArchitectureValidationCategory",
            variance="covariant",
            object_map={
                "CommutativeDiagramSpec": "ArchitectureValidationCheckSpec",
                "CapsuleContractSpec": "ArchitectureValidationCheckSpec",
                "RefactorLawSpec": "ArchitectureValidationCheckSpec",
                "ArchitectureContractsBundle": "ArchitectureValidationBundle",
            },
            morphism_map={
                "bootstrap_architecture_contracts": "bootstrap_architecture_validation",
                "validate_contract_references": "ContractReferenceValidationCheck",
            },
            code_owner="reta_architecture.architecture_validation.bootstrap_architecture_validation",
            description="Hebt Stage-29-Verträge in ausführbare Stage-31-Validierungschecks.",
        ),
        FunctorSpec(
            name="WitnessToValidationFunctor",
            source_category="CommutativeArchitectureContractCategory",
            target_category="ArchitectureValidationCategory",
            variance="covariant",
            object_map={
                "CommutativeDiagramSpec": "DiagramWitnessCoverageCheck",
                "RefactorLawSpec": "RefactorLawObligationCoverageCheck",
                "ArchitectureContractsBundle": "ArchitectureWitnessBundle",
            },
            morphism_map={
                "diagram_named": "diagram_witness_named",
                "validate_contract_references": "WitnessValidationCheck",
            },
            code_owner="reta_architecture.architecture_validation + reta_architecture.architecture_witnesses",
            description="Hebt Stage-30-Witnesses in ausführbare Stage-31-Validierungschecks.",
        ),
    )


def _natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec(
            name="RawToCanonicalParameterTransformation",
            source_functor="RawCommandPresheafFunctor",
            target_functor="CanonicalParameterSheafFunctor",
            components={
                "PromptText": "PromptMorphisms.split_command_words",
                "RawAlias": "AliasMorphisms.canonical_main / canonical_sub",
                "ParameterPair": "ParameterSemanticsSheaf.canonicalize_pair",
            },
            naturality_condition="Kontext zuerst einschränken und dann kanonisieren liefert dieselbe kanonische Semantik wie zuerst kanonisieren und anschließend auf den kleineren Kontext einschränken.",
            code_owner="reta_architecture.morphisms + reta_architecture.sheaves",
            description="Macht CLI-/Prompt-Rohtext zu kanonischer Parametersemantik.",
        ),
        NaturalTransformationSpec(
            name="PresheafToSheafGluingTransformation",
            source_functor="LocalDataPresheafFunctor",
            target_functor="GluedSemanticSheafFunctor",
            components={
                "CsvSections": "Presheaf.restrict -> merge_parameter_dicts",
                "TranslationSections": "SheafBundle.from_repo",
                "PromptSections": "PromptStatePresheaf.update -> sync_program_semantics",
            },
            naturality_condition="Lokale Sektionen über einer Überdeckung kleben zu derselben globalen Semantik, unabhängig davon, in welcher kompatiblen Reihenfolge die lokalen Restriktionen gelesen werden.",
            code_owner="reta_architecture.presheaves + reta_architecture.sheaves + reta_architecture.universal",
            description="Symbolische Sheafification der bereits vorhandenen Prägarben in die geklebten Garben.",
        ),
        NaturalTransformationSpec(
            name="TableGenerationGluingTransformation",
            source_functor="CanonicalParameterSheafFunctor",
            target_functor="TableGenerationGluingFunctor",
            components={
                "ParameterSemanticsSheaf": "ColumnSelectionBundle",
                "ColumnBuckets": "normalize_column_buckets",
                "ProgramWorkflow": "ProgramWorkflowBundle.run",
            },
            naturality_condition="Kanonische Parametersemantik, Spaltenauswahl und Tabellenbau bilden ein kommutatives Workflow-Diagramm: äquivalente Alias-/Kontextpfade erzeugen dieselbe globale Tabellensektion.",
            code_owner="reta_architecture.column_selection + reta_architecture.table_generation + reta_architecture.program_workflow",
            description="Verbindet die geklebte Parametersemantik mit der globalen Tabellen-Sektion.",
        ),
        NaturalTransformationSpec(
            name="GeneratedColumnsSheafSyncTransformation",
            source_functor="GeneratedColumnEndofunctorFamily",
            target_functor="ExplicitTableStateFunctor",
            components={
                "GeneratedColumnRegistry": "GeneratedColumnSection.parameters",
                "GeneratedColumnTags": "GeneratedColumnSection.tags",
                "GeneratedColumnsSheaf": "sync_generated_columns_from_tables",
            },
            naturality_condition="Ein generierter Spalten-Endofunktor und die anschließende State-/Sheaf-Synchronisierung kommutieren mit dem direkten Zugriff auf die explizite GeneratedColumnSection.",
            code_owner="reta_architecture.generated_columns + reta_architecture.table_state + reta_architecture.universal",
            description="Hält generierte Spalten als Runtime-Zustand und als Garbenmetadaten zusammen.",
        ),
        NaturalTransformationSpec(
            name="TableRuntimeToStateSectionsTransformation",
            source_functor="MutableTableRuntimeFunctor",
            target_functor="ExplicitTableStateFunctor",
            components={
                "Tables.generatedSpaltenParameter": "TableStateSections.generated_columns.parameters",
                "Tables.generatedSpaltenParameter_Tags": "TableStateSections.generated_columns.tags",
                "Tables.rowNumDisplay2rowNumOrig": "TableStateSections.row_display_to_original",
                "Tables.religionNumbers": "TableDisplayState.religion_numbers",
            },
            naturality_condition="Alte mutable Tabellenattribute und neue explizite Zustandssektionen referenzieren dieselben Objekte; Mutation über einen Pfad ist über den anderen Pfad sichtbar.",
            code_owner="reta_architecture.table_runtime + reta_architecture.table_state",
            description="Formalisiert den Stage-26-Übergang von `Tables` zu expliziten Tabellenzustandssektionen.",
        ),
        NaturalTransformationSpec(
            name="RenderedOutputNormalizationTransformation",
            source_functor="OutputRenderingFunctorFamily",
            target_functor="NormalizedOutputFunctor",
            components={
                "html": "HTML normalisieren",
                "markdown": "Markdown-Text vergleichen",
                "shell": "Shell-Text vergleichen",
                "csv": "CSV-Text vergleichen",
            },
            naturality_condition="Renderer-Ausgaben dürfen syntaktische Formatdetails haben, müssen nach zulässiger Normalisierung aber dieselbe semantische Paritätsaussage ergeben.",
            code_owner="reta_architecture.output_syntax + reta_architecture.output_semantics + tests.test_command_parity",
            description="Macht die existierende Renderer-Parität als natürliche Transformation sichtbar.",
        ),
        NaturalTransformationSpec(
            name="LegacyToArchitectureTransformation",
            source_functor="LegacyRuntimeFunctor",
            target_functor="ArchitectureRuntimeFunctor",
            components={
                "reta.py": "RetaArchitecture.bootstrap",
                "libs.tableHandling": "reta_architecture.table_runtime",
                "libs.lib4tables_prepare": "reta_architecture.table_preparation / row_filtering / table_wrapping",
                "libs.lib4tables_concat": "reta_architecture.generated_columns / concat_csv / combi_join",
            },
            naturality_condition="Jeder repräsentative alte Aufrufpfad und der entsprechende neue Architekturpfad müssen beobachtbar gleiche Ausgabe liefern.",
            code_owner="compatibility facades + tests.test_command_parity",
            description="Die zentrale Refactor-Invariante: Legacy-Fassaden und Architektur-Fassade kommutieren beobachtbar.",
        ),

        NaturalTransformationSpec(
            name="ContractedNaturalityTransformation",
            source_functor="CategoryTheoryToContractFunctor",
            target_functor="ArchitectureMapToContractFunctor",
            components={
                "NaturalTransformationSpec": "CommutativeDiagramSpec",
                "ArchitectureCapsuleSpec": "CapsuleContractSpec",
                "RefactorInvariant": "RefactorLawSpec",
                "ReferenceValidation": "ContractValidationSpec",
            },
            naturality_condition="Die aus Kategorie-Theorie und Kapselkarte abgeleiteten Vertragsdiagramme referenzieren dieselben bekannten Kapseln, Kategorien, Funktoren und natürlichen Transformationen.",
            code_owner="reta_architecture.architecture_contracts",
            description="Stage 29 macht natürliche Transformationen zu expliziten, validierten Architekturverträgen.",
        ),
        NaturalTransformationSpec(
            name="ContractWitnessValidationTransformation",
            source_functor="ContractToValidationFunctor",
            target_functor="WitnessToValidationFunctor",
            components={
                "CommutativeDiagramSpec": "DiagramWitnessCoverageCheck",
                "CapsuleContractSpec": "CapsuleContractCoverageCheck",
                "RefactorLawSpec": "RefactorLawObligationCoverageCheck",
                "NaturalTransformationSpec": "NaturalTransformationWitnessCoverageCheck",
                "RepositoryManifest": "PackageIntegrityValidationCheck",
            },
            naturality_condition="Direkte Vertragsvalidierung und Validierung über konkrete Witnesses müssen denselben Stage-31-Gesamtstatus liefern: alle referenzierten Kategorien, Kapseln, Diagramme, Gesetze und natürlichen Transformationen sind gedeckt.",
            code_owner="reta_architecture.architecture_validation",
            description="Stage 31 macht die Verträge und Witnesses zu einem ausführbaren Validierungsdiagramm.",
        ),
    )


def _paradigm_terms() -> tuple[ParadigmTermSpec, ...]:
    return (
        ParadigmTermSpec(
            term="Topologie",
            meaning="Reta-Kontexte als offene Mengen über Sprache, Parametern, Zeilen, Ausgabearten, Tags und Scopes.",
            implemented_by=("reta_architecture/topology.py",),
            stage_status="bereits seit früher Refactor-Stufe umgesetzt; Stage 27 macht die Kategorie der offenen Kontexte explizit",
        ),
        ParadigmTermSpec(
            term="Morphismus",
            meaning="Explizit benannte Übergänge zwischen Architektursektionen statt versteckter Methoden in Legacy-Klassen.",
            implemented_by=(
                "reta_architecture/morphisms.py",
                "reta_architecture/row_filtering.py",
                "reta_architecture/generated_columns.py",
                "reta_architecture/concat_csv.py",
                "reta_architecture/table_output.py",
            ),
            stage_status="bereits breit umgesetzt; Stage 27 ordnet Morphismen Kategorien und Funktoren zu",
        ),
        ParadigmTermSpec(
            term="universelle Eigenschaft",
            meaning="Kanonische Gluing-/Normalisierungs-Knoten, die lokale Daten kompatibel zusammenführen.",
            implemented_by=("reta_architecture/universal.py", "reta_architecture/table_generation.py", "reta_architecture/program_workflow.py"),
            stage_status="bereits umgesetzt; Stage 27 benennt die UniversalConstructionCategory",
        ),
        ParadigmTermSpec(
            term="Prägarbe",
            meaning="Lokale Rohsektionen, die über Kontext-Restriktionen gelesen werden.",
            implemented_by=("reta_architecture/presheaves.py",),
            stage_status="bereits umgesetzt; Stage 27 ergänzt Presheaf-Funktoren",
        ),
        ParadigmTermSpec(
            term="Garbe",
            meaning="Geklebte, global verwendbare Semantik aus lokalen Sektionen.",
            implemented_by=("reta_architecture/sheaves.py",),
            stage_status="bereits umgesetzt; Stage 27 ergänzt Sheafification als natürliche Transformation",
        ),
        ParadigmTermSpec(
            term="math Kategorie",
            meaning="Objekte und Morphismen der Architektur werden als inspizierbare Kategorien benannt.",
            implemented_by=("reta_architecture/category_theory.py",),
            stage_status="neu in Stage 27",
        ),
        ParadigmTermSpec(
            term="Funktor",
            meaning="Strukturerhaltende Abbildungen zwischen Kontext-, Sektionen-, Tabellen-, Output- und Legacy-Kategorien.",
            implemented_by=("reta_architecture/category_theory.py",),
            stage_status="neu in Stage 27",
        ),
        ParadigmTermSpec(
            term="natürliche Transformation",
            meaning="Kommutierende Verträglichkeitsfamilien zwischen Funktoren, z.B. Raw→Canonical, Presheaf→Sheaf, Legacy→Architecture.",
            implemented_by=("reta_architecture/category_theory.py",),
            stage_status="neu in Stage 27",
        ),
    )


def _plan() -> Stage27ArchitecturePlan:
    return Stage27ArchitecturePlan(
        planned_before_stage_27=(
            "Nicht noch eine große Verhaltensänderung, sondern die bereits extrahierten Topologie-/Prägarben-/Garben-/Morphismen-/Gluing-Schichten kategorial sichtbar machen.",
            "Kategorien für offene Kontexte, lokale Sektionen, geklebte Semantik, Tabellen-Sektionen, generierte Spalten, Ausgabeformate und Legacy-Fassaden benennen.",
            "Funktoren zwischen diesen Kategorien einführen, insbesondere Presheaf-/Sheaf-, Table-Generation-, Renderer- und Legacy-Kompatibilitätsfunktoren.",
            "Natürliche Transformationen ergänzen, weil die Refactor-Invariante nicht nur Funktorabbildung ist, sondern kommutierende Verträglichkeit zwischen alten/neuen und lokalen/globalen Pfaden.",
        ),
        implemented_in_stage_27=(
            "Neue Datei reta_architecture/category_theory.py mit CategorySpec, FunctorSpec, NaturalTransformationSpec, ParadigmTermSpec und CategoryTheoryBundle.",
            "RetaArchitecture bootstrapt und snapshottet jetzt die Kategorie-Theorie-Schicht.",
            "reta_architecture_probe_py.py besitzt category-theory-json.",
            "Paketmanifest und Regressionstests prüfen die neue Schicht.",
            "Dokumentation beschreibt geplant/jetzt/bereits umgesetzt.",
        ),
        already_implemented_before_stage_27=(
            "Topologie: reta_architecture/topology.py.",
            "Prägarben: reta_architecture/presheaves.py.",
            "Garben: reta_architecture/sheaves.py.",
            "Morphismen: morphisms.py, row_filtering.py, generated_columns.py, concat_csv.py, combi_join.py, table_output.py und weitere.",
            "Universelle Eigenschaften/Gluing: reta_architecture/universal.py, table_generation.py und program_workflow.py.",
            "Globale Tabellensektion und State-Sektionen: table_runtime.py und table_state.py.",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 27 ist eine explizite, inspizierbare Architektur- und Paradigmen-Schicht",
    )


def _stage32_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ArchitectureCoherenceCategory",
            description="Kategorie der kohärenten Metaarchitektur: Kapsel, Kategorie, Funktor, Transformation, Diagramm, Gesetz und Witness werden zusammengeführt.",
            objects=(
                _obj("ArchitectureCoherenceBundle", "reta_architecture.architecture_coherence", "Kohärenzmatrix über Kapseln und funktorielle Routen"),
                _obj("ArchitectureValidationBundle", "reta_architecture.architecture_validation", "ausführbarer Audit der Metaschichten"),
            ),
            morphisms=(
                _mor("validate", "ArchitectureCoherenceBundle", "ArchitectureValidationBundle", "bootstrap_architecture_validation", "Kohärenz wird validiert"),
            ),
            implemented_by=("reta_architecture/architecture_coherence.py", "reta_architecture/architecture_validation.py"),
        ),
        CategorySpec(
            name="ArchitectureTraceCategory",
            description="Kategorie navigierbarer Architekturspuren von alten reta-Besitzern bis zu Kapseln, Funktoren, Transformationen, Diagrammen und Witnesses.",
            objects=(
                _obj("ArchitectureTraceBundle", "reta_architecture.architecture_traces", "Trace-Index"),
                _obj("RetaComponentTraceSpec", "reta_architecture.architecture_traces", "alte Komponente als Spur"),
            ),
            morphisms=(
                _mor("trace", "RetaComponentTraceSpec", "ArchitectureTraceBundle", "bootstrap_architecture_traces", "alte reta-Komponente wird navigierbar"),
            ),
            implemented_by=("reta_architecture/architecture_traces.py",),
        ),
        CategorySpec(
            name="ArchitectureBoundaryCategory",
            description="Kategorie realer Modulbesitz- und Importgrenzen; Python-Importe werden als Kapsel-Morphismen klassifiziert.",
            objects=(
                _obj("ArchitectureBoundariesBundle", "reta_architecture.architecture_boundaries", "Boundary-Importgraph"),
                _obj("ModuleOwnershipSpec", "reta_architecture.architecture_boundaries", "Modulbesitz"),
                _obj("ImportEdgeSpec", "reta_architecture.architecture_boundaries", "Importkante"),
            ),
            morphisms=(
                _mor("classify_import", "ModuleOwnershipSpec", "ImportEdgeSpec", "bootstrap_architecture_boundaries", "Import wird Boundary-Morphismus"),
            ),
            implemented_by=("reta_architecture/architecture_boundaries.py",),
        ),
    )


def _stage32_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("CoherenceMatrixFunctor", "CommutativeArchitectureContractCategory", "ArchitectureCoherenceCategory", "covariant", {"ContractDiagram":"CoherenceRow"}, {"protects":"coheres"}, "reta_architecture/architecture_coherence.py", "Führt Verträge, Witnesses und Kapseln als Kohärenzmatrix zusammen."),
        FunctorSpec("CoherenceToTraceFunctor", "ArchitectureCoherenceCategory", "ArchitectureTraceCategory", "covariant", {"CoherenceRow":"TraceRow"}, {"coheres":"trace"}, "reta_architecture/architecture_traces.py", "Macht Kohärenzzeilen als navigierbare Trace-Routen sichtbar."),
        FunctorSpec("LegacyOwnershipTraceFunctor", "LegacyFacadeCategory", "ArchitectureTraceCategory", "covariant", {"LegacyOwner":"RetaComponentTraceSpec"}, {"delegates":"trace"}, "reta_architecture/architecture_traces.py", "Ordnet alte reta-Besitzer ihren neuen Kapselspuren zu."),
        FunctorSpec("CoherenceToBoundaryFunctor", "ArchitectureCoherenceCategory", "ArchitectureBoundaryCategory", "covariant", {"CapsuleCoherenceSpec":"CapsuleBoundarySpec"}, {"coheres":"classify_import"}, "reta_architecture/architecture_boundaries.py", "Projiziert Kohärenz auf konkrete Modul- und Importgrenzen."),
        FunctorSpec("LegacyImportBoundaryFunctor", "LegacyFacadeCategory", "ArchitectureBoundaryCategory", "covariant", {"LegacyModule":"ModuleOwnershipSpec"}, {"imports":"ImportEdgeSpec"}, "reta_architecture/architecture_boundaries.py", "Klassifiziert Legacy-Importe als sichtbare Kapselgrenzen."),
    )


def _stage32_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("CoherenceToTraceTransformation", "CoherenceMatrixFunctor", "CoherenceToTraceFunctor", {"Capsule":"CapsuleTraceSpec", "LegacyOwner":"RetaComponentTraceSpec"}, "Kohärenz und Trace-Navigation führen für jede Kapsel zum selben Diagramm-/Witness-Vertrag.", "reta_architecture/architecture_traces.py", "Verbindet Stage-31-Kohärenz mit Stage-32-Trace-Navigation."),
        NaturalTransformationSpec("CoherenceBoundaryValidationTransformation", "CoherenceToBoundaryFunctor", "LegacyImportBoundaryFunctor", {"Module":"ModuleOwnershipSpec", "Import":"ImportEdgeSpec"}, "Kapselgrenzen aus Kohärenz und reale Python-Importe werden zu demselben Boundary-Graphen klassifiziert.", "reta_architecture/architecture_boundaries.py", "Verbindet Kohärenzmatrix und reale Importkanten."),
    )



def _stage33_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ArchitectureImpactCategory",
            description="Kategorie der Stage-33 Impact-Routen: Trace- und Boundary-Informationen werden zu Regression-Gates und Migrationskandidaten verdichtet.",
            objects=(
                _obj("ArchitectureImpactBundle", "reta_architecture.architecture_impact", "Impact- und Migration-Gate-Bündel"),
                _obj("ImpactSourceSpec", "reta_architecture.architecture_impact", "alte/neue reta-Komponente als Impact-Quelle"),
                _obj("RegressionGateSpec", "reta_architecture.architecture_impact", "Probe-/Test-Gate"),
                _obj("MigrationCandidateSpec", "reta_architecture.architecture_impact", "guarded future extraction candidate"),
            ),
            morphisms=(
                _mor("compute_impact", "ImpactSourceSpec", "ArchitectureImpactBundle", "bootstrap_architecture_impact", "Trace/Boundary wird Impact-Route"),
                _mor("gate", "MigrationCandidateSpec", "RegressionGateSpec", "bootstrap_architecture_impact", "Migrationskandidat wird durch Gates geschützt"),
            ),
            implemented_by=("reta_architecture/architecture_impact.py",),
        ),
    )


def _stage33_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("TraceBoundaryImpactFunctor", "ArchitectureTraceCategory", "ArchitectureImpactCategory", "covariant", {"RetaComponentTraceSpec":"ImpactSourceSpec"}, {"trace":"compute_impact"}, "reta_architecture/architecture_impact.py", "Projiziert Trace-Routen auf Impact-Quellen und betroffene Verträge."),
        FunctorSpec("BoundaryImpactFunctor", "ArchitectureBoundaryCategory", "ArchitectureImpactCategory", "covariant", {"ImportEdgeSpec":"ImpactSourceSpec"}, {"classify_import":"compute_impact"}, "reta_architecture/architecture_impact.py", "Projiziert reale Importgrenzen auf Impact-Quellen."),
        FunctorSpec("ImpactGateValidationFunctor", "ArchitectureImpactCategory", "ArchitectureCoherenceCategory", "covariant", {"RegressionGateSpec":"CoherenceRow"}, {"gate":"validate"}, "reta_architecture/architecture_impact.py", "Führt Impact-Gates in die validierbare Kohärenzschicht zurück."),
        FunctorSpec("MigrationCandidateFunctor", "LegacyFacadeCategory", "ArchitectureImpactCategory", "covariant", {"LegacyOwner":"MigrationCandidateSpec"}, {"delegates":"gate"}, "reta_architecture/architecture_impact.py", "Liest Legacy-Owner als guarded Migrationskandidaten statt als neue Semantik-Owner."),
    )


def _stage33_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("TraceBoundaryImpactTransformation", "TraceBoundaryImpactFunctor", "BoundaryImpactFunctor", {"Owner":"ImpactSourceSpec", "Import":"ImpactSourceSpec"}, "Impact aus Trace-Route und Impact aus Boundary-Importgraph führen zu derselben betroffenen Kapsel-/Diagramm-/Gate-Lesart.", "reta_architecture/architecture_impact.py", "Verbindet Stage-32-Trace und Stage-32-Boundary als Stage-33-Impact."),
        NaturalTransformationSpec("ImpactGateValidationTransformation", "MigrationCandidateFunctor", "ImpactGateValidationFunctor", {"Candidate":"RegressionGateSpec", "Gate":"ImpactValidationSpec"}, "Migrationskandidaten und Gate-Validierung kommutieren: ein späterer Move ist nur zulässig, wenn seine Impact-Gates bestehen.", "reta_architecture/architecture_impact.py", "Verbindet Migrationskandidaten mit prüfbarer Gate-Validierung."),
    )



def _stage34_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ArchitectureMigrationCategory",
            description="Kategorie der Stage-34 Migrationsplanung: Impact-Kandidaten werden zu geordneten Wellen, Schritten, Gate-Bindings und Migrationsinvarianten.",
            objects=(
                _obj("ArchitectureMigrationBundle", "reta_architecture.architecture_migration", "geordnetes Migrationsplan-Bündel"),
                _obj("MigrationWaveSpec", "reta_architecture.architecture_migration", "kapselweise Migrationswelle"),
                _obj("MigrationStepSpec", "reta_architecture.architecture_migration", "konkreter, noch nicht ausgeführter guarded migration step"),
                _obj("MigrationGateBindingSpec", "reta_architecture.architecture_migration", "Bindung von Migrationsschritt an Regression-Gates"),
                _obj("MigrationInvariantSpec", "reta_architecture.architecture_migration", "Naturality-/Law-Invariante pro Migrationswelle"),
            ),
            morphisms=(
                _mor("plan", "MigrationCandidateSpec", "MigrationStepSpec", "bootstrap_architecture_migration", "Impact-Kandidat wird geplanter Migrationsschritt"),
                _mor("order", "MigrationStepSpec", "MigrationWaveSpec", "bootstrap_architecture_migration", "Schritt wird in eine Kapselwelle eingeordnet"),
                _mor("bind_gate", "MigrationStepSpec", "MigrationGateBindingSpec", "bootstrap_architecture_migration", "Schritt erhält konkrete Gates"),
                _mor("preserve_invariant", "MigrationWaveSpec", "MigrationInvariantSpec", "bootstrap_architecture_migration", "Welle erhält Natürlichkeits-/Gesetzespflicht"),
            ),
            implemented_by=("reta_architecture/architecture_migration.py",),
        ),
    )


def _stage34_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("ImpactToMigrationPlanFunctor", "ArchitectureImpactCategory", "ArchitectureMigrationCategory", "covariant", {"MigrationCandidateSpec":"MigrationStepSpec"}, {"gate":"plan"}, "reta_architecture/architecture_migration.py", "Projiziert Stage-33-Migrationskandidaten auf geordnete Stage-34-Schritte."),
        FunctorSpec("ImpactGateBindingFunctor", "ArchitectureImpactCategory", "ArchitectureMigrationCategory", "covariant", {"RegressionGateSpec":"MigrationGateBindingSpec"}, {"gate":"bind_gate"}, "reta_architecture/architecture_migration.py", "Projiziert Impact-Gates auf konkrete Gate-Bindings der Migrationsschritte."),
        FunctorSpec("MigrationWaveOrderingFunctor", "ArchitectureMigrationCategory", "ArchitectureMigrationCategory", "covariant", {"MigrationStepSpec":"MigrationWaveSpec"}, {"plan":"order"}, "reta_architecture/architecture_migration.py", "Endofunktor, der geplante Schritte stufenweise/kapselweise ordnet."),
        FunctorSpec("MigrationOrderingCoherenceFunctor", "ArchitectureMigrationCategory", "ArchitectureCoherenceCategory", "covariant", {"MigrationWaveSpec":"CoherenceRow"}, {"order":"coheres"}, "reta_architecture/architecture_migration.py", "Reflektiert Migrationswellen in die Kohärenzmatrix."),
        FunctorSpec("MigrationGateCoherenceFunctor", "ArchitectureMigrationCategory", "ArchitectureCoherenceCategory", "covariant", {"MigrationGateBindingSpec":"CoherenceRow"}, {"bind_gate":"validate"}, "reta_architecture/architecture_migration.py", "Reflektiert Gate-Bindings in die validierbare Kohärenzschicht."),
    )


def _stage34_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("ImpactGateMigrationTransformation", "ImpactToMigrationPlanFunctor", "ImpactGateBindingFunctor", {"Candidate":"MigrationStepSpec", "Gate":"MigrationGateBindingSpec"}, "Der direkte Pfad Impact-Kandidat→Migrationsschritt und der Pfad Impact-Gate→Gate-Binding beschreiben denselben erlaubten späteren Move.", "reta_architecture/architecture_migration.py", "Verbindet Stage-33 Impact-Kandidaten mit Stage-34 Gate-Bindings."),
        NaturalTransformationSpec("MigrationPlanCoherenceTransformation", "MigrationOrderingCoherenceFunctor", "MigrationGateCoherenceFunctor", {"Wave":"MigrationInvariantSpec", "GateBinding":"MigrationGateBindingSpec"}, "Wellenordnung und Gate-Kohärenz kommutieren: eine geplante Extraktion ist nur kohärent, wenn ihre Gates und Invarianten dieselbe Welle schützen.", "reta_architecture/architecture_migration.py", "Verbindet Stage-34 Migrationswellen mit Validierung/Kohärenz."),
    )



def _stage35_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ArchitectureRehearsalCategory",
            description="Kategorie der Stage-35 Trockenlauf-/Readiness-Schicht: Migrationswellen werden Rehearsal-Open-Sets, Schritte werden Refactor-Moves und Gate-Bindings werden prüfbare Gate-Suites.",
            objects=(
                _obj("ArchitectureRehearsalBundle", "reta_architecture.architecture_rehearsal", "Trockenlauf-/Readiness-Bündel"),
                _obj("RehearsalOpenSetSpec", "reta_architecture.architecture_rehearsal", "topologische Basisöffnung einer Migrationswelle"),
                _obj("RehearsalMoveSpec", "reta_architecture.architecture_rehearsal", "dry-run Refactor-Morphismus"),
                _obj("GateRehearsalSpec", "reta_architecture.architecture_rehearsal", "lokale Gate-Prüfsektion"),
                _obj("RehearsalCoverSpec", "reta_architecture.architecture_rehearsal", "universelles Readiness-Cover"),
            ),
            morphisms=(
                _mor("rehearse_step", "MigrationStepSpec", "RehearsalMoveSpec", "bootstrap_architecture_rehearsal", "geplanter Schritt wird Trockenlauf-Morphismus"),
                _mor("rehearse_gate", "MigrationGateBindingSpec", "GateRehearsalSpec", "bootstrap_architecture_rehearsal", "Gate-Binding wird Preflight/Postflight-Suite"),
                _mor("cover_wave", "RehearsalOpenSetSpec", "RehearsalCoverSpec", "bootstrap_architecture_rehearsal", "lokale Moves kleben zur globalen Readiness"),
            ),
            implemented_by=("reta_architecture/architecture_rehearsal.py",),
        ),
    )


def _stage35_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("MigrationStepRehearsalFunctor", "ArchitectureMigrationCategory", "ArchitectureRehearsalCategory", "covariant", {"MigrationStepSpec":"RehearsalMoveSpec"}, {"plan":"rehearse_step"}, "reta_architecture/architecture_rehearsal.py", "Projiziert Stage-34-Migrationsschritte auf Stage-35-Trockenlauf-Moves."),
        FunctorSpec("MigrationGateRehearsalFunctor", "ArchitectureMigrationCategory", "ArchitectureRehearsalCategory", "covariant", {"MigrationGateBindingSpec":"GateRehearsalSpec"}, {"bind_gate":"rehearse_gate"}, "reta_architecture/architecture_rehearsal.py", "Projiziert Stage-34-Gate-Bindings auf Preflight/Postflight-Rehearsal-Suites."),
        FunctorSpec("RehearsalCoverFunctor", "ArchitectureRehearsalCategory", "ArchitectureRehearsalCategory", "covariant", {"RehearsalOpenSetSpec":"RehearsalCoverSpec"}, {"rehearse_step":"cover_wave"}, "reta_architecture/architecture_rehearsal.py", "Klebt lokale Rehearsal-Sektionen zu einem Readiness-Cover."),
        FunctorSpec("RehearsalGateValidationFunctor", "ArchitectureRehearsalCategory", "ArchitectureValidationCategory", "covariant", {"GateRehearsalSpec":"ArchitectureValidationCheckSpec"}, {"rehearse_gate":"validate"}, "reta_architecture/architecture_validation.py", "Reflektiert Gate-Rehearsals in die Validierungsschicht."),
        FunctorSpec("RehearsalReadinessCoherenceFunctor", "ArchitectureRehearsalCategory", "ArchitectureCoherenceCategory", "covariant", {"RehearsalCoverSpec":"CoherenceRow"}, {"cover_wave":"coheres"}, "reta_architecture/architecture_coherence.py", "Reflektiert Stage-35-Readiness in die Kohärenzmatrix."),
    )


def _stage35_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("MigrationRehearsalNaturalityTransformation", "MigrationStepRehearsalFunctor", "MigrationGateRehearsalFunctor", {"Step":"RehearsalMoveSpec", "Gate":"GateRehearsalSpec"}, "Migrationsschritt und Gate-Binding führen zum selben trockenlaufgeschützten Move.", "reta_architecture/architecture_rehearsal.py", "Verbindet Stage-34-Migration mit Stage-35-Rehearsal."),
        NaturalTransformationSpec("RehearsalReadinessValidationTransformation", "RehearsalCoverFunctor", "RehearsalGateValidationFunctor", {"Cover":"RehearsalCoverSpec", "GateSuite":"ArchitectureValidationCheckSpec"}, "Readiness-Cover und Gate-Validierung kommutieren: lokale Gate-Suiten kleben zur gleichen globalen Readiness-Aussage.", "reta_architecture/architecture_rehearsal.py", "Verbindet Rehearsal-Covers mit Validierung/Kohärenz."),
    )



def _stage36_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ArchitectureActivationCategory",
            description="Kategorie der Stage-36 Aktivierungs-/Commit-Schicht: Rehearsal-Moves werden zu Aktivierungsfenstern, Commit-Gates, Rollback-Sektionen und Transaktionen, ohne Runtime-Verhalten zu bewegen.",
            objects=(
                _obj("ArchitectureActivationBundle", "reta_architecture.architecture_activation", "Aktivierungs-/Commit-/Rollback-Bündel"),
                _obj("ActivationWindowSpec", "reta_architecture.architecture_activation", "topologische Aktivierungsumgebung einer Rehearsal-Welle"),
                _obj("ActivationUnitSpec", "reta_architecture.architecture_activation", "nicht ausgeführter Commit-Morphismus über einem Rehearsal-Move"),
                _obj("ActivationGateSpec", "reta_architecture.architecture_activation", "lokale Preflight/Commit/Postflight/Rollback-Gate-Sektion"),
                _obj("ActivationRollbackSpec", "reta_architecture.architecture_activation", "Rollback-Sektion zum Schutz kommutierender Diagramme"),
                _obj("ActivationTransactionSpec", "reta_architecture.architecture_activation", "universelles Gluing lokaler Aktivierungseinheiten"),
            ),
            morphisms=(
                _mor("activate_move", "RehearsalMoveSpec", "ActivationUnitSpec", "bootstrap_architecture_activation", "Trockenlauf-Move wird Aktivierungsumschlag"),
                _mor("activate_gate", "GateRehearsalSpec", "ActivationGateSpec", "bootstrap_architecture_activation", "Gate-Rehearsal wird Commit-/Rollback-Gate"),
                _mor("rollback", "ActivationGateSpec", "ActivationRollbackSpec", "bootstrap_architecture_activation", "Commit-Gate erhält Rollback-Sektion"),
                _mor("commit_transaction", "ActivationWindowSpec", "ActivationTransactionSpec", "bootstrap_architecture_activation", "lokale Aktivierungen kleben zu einer Transaktion"),
            ),
            implemented_by=("reta_architecture/architecture_activation.py",),
        ),
    )


def _stage36_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("RehearsalActivationFunctor", "ArchitectureRehearsalCategory", "ArchitectureActivationCategory", "covariant", {"RehearsalMoveSpec":"ActivationUnitSpec"}, {"rehearse_step":"activate_move"}, "reta_architecture/architecture_activation.py", "Projiziert Stage-35-Rehearsal-Moves auf Stage-36-Aktivierungseinheiten."),
        FunctorSpec("GateActivationFunctor", "ArchitectureRehearsalCategory", "ArchitectureActivationCategory", "covariant", {"GateRehearsalSpec":"ActivationGateSpec"}, {"rehearse_gate":"activate_gate"}, "reta_architecture/architecture_activation.py", "Projiziert Gate-Rehearsals auf Commit-/Rollback-Gates."),
        FunctorSpec("ActivationTransactionFunctor", "ArchitectureActivationCategory", "ArchitectureActivationCategory", "covariant", {"ActivationWindowSpec":"ActivationTransactionSpec", "ActivationUnitSpec":"ActivationTransactionSpec"}, {"activate_move":"commit_transaction"}, "reta_architecture/architecture_activation.py", "Klebt lokale Aktivierungseinheiten zu einer Transaktion pro Fenster."),
        FunctorSpec("ActivationRollbackFunctor", "ArchitectureActivationCategory", "ArchitectureActivationCategory", "covariant", {"ActivationGateSpec":"ActivationRollbackSpec"}, {"activate_gate":"rollback"}, "reta_architecture/architecture_activation.py", "Ordnet jedem Commit-Gate eine Rollback-Sektion zu."),
        FunctorSpec("ActivationValidationFunctor", "ArchitectureActivationCategory", "ArchitectureValidationCategory", "covariant", {"ActivationTransactionSpec":"ArchitectureValidationCheckSpec"}, {"commit_transaction":"validate"}, "reta_architecture/architecture_validation.py", "Reflektiert Aktivierungsfenster, Gates und Rollback-Sektionen in die Validierung."),
        FunctorSpec("ActivationCoherenceFunctor", "ArchitectureActivationCategory", "ArchitectureCoherenceCategory", "covariant", {"ActivationTransactionSpec":"CoherenceRow"}, {"commit_transaction":"coheres"}, "reta_architecture/architecture_coherence.py", "Reflektiert Stage-36-Aktivierungstransaktionen in die Kohärenzmatrix."),
    )


def _stage36_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("RehearsalActivationNaturalityTransformation", "RehearsalActivationFunctor", "GateActivationFunctor", {"Move":"ActivationUnitSpec", "Gate":"ActivationGateSpec"}, "Aktivierung über den Rehearsal-Move und Aktivierung über die Gate-Suite beschreiben denselben commit-geschützten Umschlag.", "reta_architecture/architecture_activation.py", "Verbindet Stage-35-Rehearsal mit Stage-36-Aktivierung."),
        NaturalTransformationSpec("ActivationRollbackValidationTransformation", "ActivationTransactionFunctor", "ActivationValidationFunctor", {"Transaction":"ActivationTransactionSpec", "Rollback":"ActivationRollbackSpec"}, "Transaktionsgluing und Validierung kommutieren nur, wenn Rollback-Sektionen für alle lokalen Aktivierungen existieren.", "reta_architecture/architecture_activation.py", "Verbindet Commit-Transaktionen, Rollback-Sektionen und Validierung."),
    )


def _stage37_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ActivatedRowRangeCategory",
            description="Kategorie der Stage-37 aktivierten Zeilenbereichs-Morphismen: raw row-range text, Generator-Literale und v-Multiplikator-Sektionen werden zu endlichen Zeilenmengen expandiert; center.py bleibt nur noch Kompatibilitätsfassade.",
            objects=(
                _obj("RowRangeMorphismBundle", "reta_architecture.row_ranges", "aktivierter Architektur-Besitzer der Zeilenbereichsparser"),
                _obj("RowRangeSyntax", "reta_architecture.input_semantics.RowRangeSyntax", "Syntaxobjekt für Komma-Splitting, v-Präfix und Integer-/Bruch-Token"),
                _obj("RowRangeExpression", "libs.center.BereichToNumbers2", "legacy-kompatibler lokaler Rohtext"),
                _obj("RowIndexSet", "reta_architecture.row_ranges.range_to_numbers", "expandierte endliche Zeilensektion"),
            ),
            morphisms=(
                _mor("parse_generator_literal", "RowRangeExpression", "RowIndexSet", "reta_architecture.row_ranges.str_as_generator_to_set", "Generator-/Set-Literal wird lokale Zeilen-Sektion"),
                _mor("validate_row_range", "RowRangeExpression", "RowRangeSyntax", "reta_architecture.row_ranges.is_row_range", "raw token wird gegen die Zeilenbereichs-Topologie geprüft"),
                _mor("expand_row_range", "RowRangeExpression", "RowIndexSet", "reta_architecture.row_ranges.range_to_numbers", "Zeilenbereich wird zu Zeilenmenge expandiert"),
                _mor("delegate_center_wrappers", "libs.center", "RowRangeMorphismBundle", "libs.center.ROW_RANGE_MORPHISMS", "alte center-Funktionen delegieren an die aktivierte Architektur"),
            ),
            implemented_by=("reta_architecture/row_ranges.py", "libs/center.py"),
        ),
    )


def _stage37_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("RowRangeActivationFunctor", "ArchitectureActivationCategory", "ActivatedRowRangeCategory", "covariant", {"ActivationUnitSpec":"RowRangeMorphismBundle"}, {"activate_move":"delegate_center_wrappers"}, "reta_architecture/row_ranges.py", "Aktiviert den Stage-36-Commit-Umschlag für den M2-Input-Move: Row-Range-Logik wird realer Architekturbesitz."),
        FunctorSpec("CenterRowRangeCompatibilityFunctor", "LegacyFacadeCategory", "ActivatedRowRangeCategory", "covariant", {"libs.center.BereichToNumbers2":"RowRangeMorphismBundle"}, {"legacy_call":"expand_row_range"}, "libs/center.py", "Liest die alten center-Zeilenbereichsfunktionen als Fassaden über den neuen RowRangeMorphismBundle."),
        FunctorSpec("RowRangeInputFunctor", "ActivatedRowRangeCategory", "LocalSectionCategory", "covariant", {"RowRangeExpression":"LocalSection", "RowIndexSet":"LocalSection"}, {"expand_row_range":"restrict"}, "reta_architecture/row_ranges.py", "Transportiert aktivierte Zeilenbereichssektionen zurück in die lokale Input-/Prompt-Sektion."),
        FunctorSpec("RowRangeValidationFunctor", "ActivatedRowRangeCategory", "ArchitectureValidationCategory", "covariant", {"RowRangeMorphismBundle":"ArchitectureValidationCheckSpec"}, {"expand_row_range":"validate"}, "reta_architecture/architecture_validation.py", "Macht die aktivierte Row-Range-Migration prüfbar."),
    )


def _stage37_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("CenterRowRangeToArchitectureTransformation", "CenterRowRangeCompatibilityFunctor", "RowRangeActivationFunctor", {"BereichToNumbers2":"range_to_numbers", "isZeilenAngabe":"is_row_range", "strAsGeneratorToListOfNumStrs":"str_as_generator_to_set"}, "Erst über center.py aufrufen und dann expandieren ergibt dieselbe Zeilenmenge wie direkt über RowRangeMorphismBundle expandieren.", "reta_architecture/row_ranges.py + libs/center.py", "Stage 37 verbindet Legacy-center-API und aktivierte Architektur-API."),
        NaturalTransformationSpec("RowRangeValidationTransformation", "RowRangeInputFunctor", "RowRangeValidationFunctor", {"RowRangeMorphismBundle":"row-ranges-json", "RowIndexSet":"architecture-validation-json"}, "Row-Range-Ausdruck einschränken, expandieren und validieren kommutiert mit direkter Architekturvalidierung.", "reta_architecture/row_ranges.py", "Schützt die aktivierte Input-Morphismus-Kapsel gegen spätere center-Entkernungen."),
    )


def _stage38_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ActivatedArithmeticCategory",
            description="Kategorie der Stage-38 aktivierten center-Arithmetik: Zahlen, Zeilenbereichsausdrücke und Primfaktor-/Teilersektionen werden durch Architektur-Morphismen statt durch center.py besessen.",
            objects=(
                _obj("ArithmeticMorphismBundle", "reta_architecture.arithmetic", "aktivierter Architektur-Besitzer der center-Arithmetik"),
                _obj("ArithmeticExpression", "libs.center", "legacy-kompatible lokale Zahl- oder Bereichssektion"),
                _obj("FactorPairSet", "reta_architecture.arithmetic.factor_pairs", "endliche Sektion von Faktor-/Multiplikationspaaren"),
                _obj("PrimeFactorSection", "reta_architecture.arithmetic.prime_factors", "Primfaktorzerlegung als lokale Zahlensektion"),
                _obj("DivisorSection", "reta_architecture.arithmetic.divisor_range", "über Row-Range-Topologie geklebte Teilersektion"),
            ),
            morphisms=(
                _mor("factor_pairs", "ArithmeticExpression", "FactorPairSet", "reta_architecture.arithmetic.factor_pairs", "Zahl wird zu Multiplikations-/Faktor-Paaren expandiert"),
                _mor("prime_factorize", "ArithmeticExpression", "PrimeFactorSection", "reta_architecture.arithmetic.prime_factors", "Zahl wird in Primfaktoren zerlegt"),
                _mor("glue_divisor_range", "RowIndexSet", "DivisorSection", "reta_architecture.arithmetic.divisor_range", "Zeilenbereich wird über Faktorpaare zu Teilersektion geklebt"),
                _mor("delegate_center_arithmetic", "libs.center", "ArithmeticMorphismBundle", "libs.center.ARITHMETIC_MORPHISMS", "alte center-Arithmetikfunktionen delegieren an die aktivierte Architektur"),
            ),
            implemented_by=("reta_architecture/arithmetic.py", "libs/center.py"),
        ),
    )


def _stage38_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("ArithmeticActivationFunctor", "ArchitectureActivationCategory", "ActivatedArithmeticCategory", "covariant", {"ActivationUnitSpec":"ArithmeticMorphismBundle"}, {"activate_move":"delegate_center_arithmetic"}, "reta_architecture/arithmetic.py", "Aktiviert einen weiteren Stage-36-Commit-Umschlag: center-Arithmetik wird realer Architekturbesitz."),
        FunctorSpec("CenterArithmeticCompatibilityFunctor", "LegacyFacadeCategory", "ActivatedArithmeticCategory", "covariant", {"libs.center.multiples":"ArithmeticMorphismBundle", "libs.center.teiler":"ArithmeticMorphismBundle"}, {"legacy_call":"factor_pairs"}, "libs/center.py", "Liest alte center-Arithmetiknamen als Fassaden über ArithmeticMorphismBundle."),
        FunctorSpec("ArithmeticRowRangeGluingFunctor", "ActivatedRowRangeCategory", "ActivatedArithmeticCategory", "covariant", {"RowIndexSet":"DivisorSection"}, {"expand_row_range":"glue_divisor_range"}, "reta_architecture/arithmetic.py", "Verwendet den Stage-37 RowRangeMorphismBundle als Topologie für Stage-38 Teiler-/Faktor-Gluing."),
        FunctorSpec("ArithmeticValidationFunctor", "ActivatedArithmeticCategory", "ArchitectureValidationCategory", "covariant", {"ArithmeticMorphismBundle":"ArchitectureValidationCheckSpec"}, {"factor_pairs":"validate"}, "reta_architecture/architecture_validation.py", "Macht die aktivierte Arithmetik-Migration prüfbar."),
    )


def _stage38_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("CenterArithmeticToArchitectureTransformation", "CenterArithmeticCompatibilityFunctor", "ArithmeticActivationFunctor", {"multiples":"factor_pairs", "teiler":"divisor_range", "primfaktoren":"prime_factors", "primRepeat":"prime_repeat_legacy", "textHatZiffer":"has_digit"}, "Erst über center.py aufrufen und dann arithmetisch expandieren ergibt dasselbe Ergebnis wie der direkte ArithmeticMorphismBundle-Pfad.", "reta_architecture/arithmetic.py + libs/center.py", "Stage 38 verbindet Legacy-center-Arithmetik und aktivierte Architektur-API."),
        NaturalTransformationSpec("ArithmeticRowRangeGluingTransformation", "ArithmeticRowRangeGluingFunctor", "ArithmeticValidationFunctor", {"RowIndexSet":"DivisorSection", "ArithmeticMorphismBundle":"arithmetic-json"}, "Row-Range-Expansion und arithmetisches Teiler-Gluing kommutieren mit der Architekturvalidierung.", "reta_architecture/arithmetic.py", "Schützt die Abhängigkeit der Stage-38-Arithmetik von der Stage-37-Row-Range-Topologie."),
    )


def _stage39_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ActivatedConsoleIOCategory",
            description="Kategorie der Stage-39 aktivierten center-Console-/Help-/Utility-Morphismen: Hilfetexte, Terminal-Wrapping, Debug-Ausgabe, CLI-Output und finite Utility-Sektionen werden durch Architektur-Morphismen statt durch center.py besessen.",
            objects=(
                _obj("ConsoleIOMorphismBundle", "reta_architecture.console_io", "aktivierter Architektur-Besitzer der center-Console-/Utility-Helfer"),
                _obj("HelpMarkdownSection", "reta_architecture.console_io.reta_help_text", "lokale Dokumentationssektion aus doc/readme-Dateien"),
                _obj("ConsoleOutputSection", "reta_architecture.console_io.cli_output", "sichtbarer Console-Ausgabeeffekt"),
                _obj("FiniteUtilitySection", "reta_architecture.console_io.chunks / unique_everseen / DefaultOrderedDict", "endliche Hilfssektion für Tabellen- und Generated-Relation-Code"),
            ),
            morphisms=(
                _mor("load_help_section", "HelpMarkdownSection", "ConsoleOutputSection", "reta_architecture.console_io.reta_help_text", "Dokumentationsdatei wird lokale Help-Sektion"),
                _mor("render_cli_output", "ConsoleOutputSection", "ConsoleOutputSection", "reta_architecture.console_io.cli_output", "Textsektion wird Console-Ausgabe"),
                _mor("discover_text_wrap_runtime", "ConsoleOutputSection", "FiniteUtilitySection", "reta_architecture.console_io.get_text_wrap_things", "Terminalkontext wird Wrapping-Sektion"),
                _mor("delegate_center_console_io", "libs.center", "ConsoleIOMorphismBundle", "libs.center.CONSOLE_IO_MORPHISMS", "alte center-Hilfe-/Output-/Utilityfunktionen delegieren an die aktivierte Architektur"),
            ),
            implemented_by=("reta_architecture/console_io.py", "libs/center.py"),
        ),
    )


def _stage39_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("ConsoleIOActivationFunctor", "ArchitectureActivationCategory", "ActivatedConsoleIOCategory", "covariant", {"ActivationUnitSpec":"ConsoleIOMorphismBundle"}, {"activate_move":"delegate_center_console_io"}, "reta_architecture/console_io.py", "Aktiviert den nächsten Stage-36-Commit-Umschlag: center-Console-/Help-/Utility-Logik wird realer Architekturbesitz."),
        FunctorSpec("CenterConsoleIOCompatibilityFunctor", "LegacyFacadeCategory", "ActivatedConsoleIOCategory", "covariant", {"libs.center.cliout":"ConsoleIOMorphismBundle", "libs.center.getTextWrapThings":"ConsoleIOMorphismBundle", "libs.center.retaHilfe":"ConsoleIOMorphismBundle"}, {"legacy_call":"render_cli_output"}, "libs/center.py", "Liest alte center-Console-/Help-/Utilitynamen als Fassaden über ConsoleIOMorphismBundle."),
        FunctorSpec("ConsoleIOOutputRenderingFunctor", "ActivatedConsoleIOCategory", "OutputFormatCategory", "covariant", {"ConsoleOutputSection":"RenderedOutput"}, {"render_cli_output":"render"}, "reta_architecture/console_io.py", "Transportiert aktivierte Console-Ausgabesektionen in die bestehende Output-Rendering-Kategorie."),
        FunctorSpec("ConsoleIOValidationFunctor", "ActivatedConsoleIOCategory", "ArchitectureValidationCategory", "covariant", {"ConsoleIOMorphismBundle":"ArchitectureValidationCheckSpec"}, {"render_cli_output":"validate"}, "reta_architecture/architecture_validation.py", "Macht die aktivierte Console-/Utility-Migration prüfbar."),
    )


def _stage39_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("CenterConsoleIOToArchitectureTransformation", "CenterConsoleIOCompatibilityFunctor", "ConsoleIOActivationFunctor", {"cliout":"cli_output", "getTextWrapThings":"get_text_wrap_things", "retaHilfe":"reta_help_text", "unique_everseen":"unique_everseen"}, "Erst über center.py aufrufen und dann rendern/zerlegen ergibt dieselbe sichtbare Ausgabe bzw. endliche Hilfssektion wie der direkte ConsoleIOMorphismBundle-Pfad.", "reta_architecture/console_io.py + libs/center.py", "Stage 39 verbindet Legacy-center-Console-API und aktivierte Architektur-API."),
        NaturalTransformationSpec("ConsoleIOOutputValidationTransformation", "ConsoleIOOutputRenderingFunctor", "ConsoleIOValidationFunctor", {"ConsoleIOMorphismBundle":"console-io-json", "ConsoleOutputSection":"architecture-validation-json"}, "Console-Output rendern und Console-Output validieren kommutieren mit der bestehenden Output-Rendering-Kategorie.", "reta_architecture/console_io.py", "Schützt die aktivierte Output-/Utility-Kapsel gegen spätere center-Entkernungen."),
    )


def _stage40_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ActivatedWordCompletionCategory",
            description="Kategorie der Stage-40 aktivierten Word-Completion-Morphismen: Wortlisten, Cursor-Präfixe und Completion-Kandidaten werden durch Architektur-Morphismen statt durch libs/word_completerAlx.py besessen.",
            objects=(
                _obj("WordCompletionMorphismBundle", "reta_architecture.completion_word", "aktivierter Architektur-Besitzer der prompt_toolkit-WordCompleter-Logik"),
                _obj("CompletionWordSection", "reta_architecture.completion_word.resolve_words", "lokale Liste oder Callable-Sektion möglicher Wörter"),
                _obj("CursorPrefixOpenSet", "reta_architecture.completion_word.word_before_cursor", "topologische Einschränkung des Dokuments auf Text vor dem Cursor"),
                _obj("CompletionCandidateSection", "reta_architecture.completion_word.iter_word_completions", "lokale Completion-Ausgabesektion"),
            ),
            morphisms=(
                _mor("resolve_completion_words", "CompletionWordSection", "CompletionWordSection", "reta_architecture.completion_word.resolve_words", "statische/callable Wortquelle wird lokale Completion-Sektion"),
                _mor("restrict_to_cursor_prefix", "Document", "CursorPrefixOpenSet", "reta_architecture.completion_word.word_before_cursor", "Dokument wird auf Cursor-Präfix-Open-Set eingeschränkt"),
                _mor("match_completion_word", "CompletionWordSection", "CompletionCandidateSection", "reta_architecture.completion_word.word_completion_matches", "Wortsektion wird gegen Prefix/Middle-Match geprüft"),
                _mor("delegate_word_completer", "libs.word_completerAlx", "WordCompletionMorphismBundle", "libs/word_completerAlx.py", "alte WordCompleter-Importfläche delegiert an die aktivierte Architekturklasse"),
            ),
            implemented_by=("reta_architecture/completion_word.py", "libs/word_completerAlx.py"),
        ),
    )


def _stage40_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("WordCompletionActivationFunctor", "ArchitectureActivationCategory", "ActivatedWordCompletionCategory", "covariant", {"ActivationUnitSpec":"WordCompletionMorphismBundle"}, {"activate_move":"delegate_word_completer"}, "reta_architecture/completion_word.py", "Aktiviert den nächsten Stage-36-Commit-Umschlag: word_completerAlx wird reale Architektur-Completion-Logik."),
        FunctorSpec("LegacyWordCompleterCompatibilityFunctor", "LegacyFacadeCategory", "ActivatedWordCompletionCategory", "covariant", {"libs.word_completerAlx.WordCompleter":"ArchitectureWordCompleter"}, {"legacy_call":"iter_word_completions"}, "libs/word_completerAlx.py", "Liest den alten WordCompleter-Import als Fassade über die Architektur-Completion-Klasse."),
        FunctorSpec("WordCompletionPromptFunctor", "ActivatedWordCompletionCategory", "LocalSectionCategory", "covariant", {"CompletionCandidateSection":"PromptCompletionSection"}, {"iter_word_completions":"prompt_completion"}, "reta_architecture/completion_word.py", "Transportiert Completion-Kandidaten zurück in die Prompt-/Input-Kapsel."),
        FunctorSpec("WordCompletionValidationFunctor", "ActivatedWordCompletionCategory", "ArchitectureValidationCategory", "covariant", {"WordCompletionMorphismBundle":"ArchitectureValidationCheckSpec"}, {"iter_word_completions":"validate"}, "reta_architecture/architecture_validation.py", "Macht die aktivierte Word-Completion-Migration prüfbar."),
    )


def _stage40_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("WordCompleterToArchitectureTransformation", "LegacyWordCompleterCompatibilityFunctor", "WordCompletionActivationFunctor", {"WordCompleter":"ArchitectureWordCompleter", "get_completions":"iter_word_completions"}, "Erst über libs.word_completerAlx.WordCompleter instanziieren und dann Completion-Kandidaten erzeugen ergibt dieselbe Kandidatensektion wie der direkte WordCompletionMorphismBundle-Pfad.", "reta_architecture/completion_word.py + libs/word_completerAlx.py", "Stage 40 verbindet die alte WordCompleter-Fassade mit der aktivierten Architektur-Completion-API."),
        NaturalTransformationSpec("WordCompletionValidationTransformation", "WordCompletionPromptFunctor", "WordCompletionValidationFunctor", {"CompletionCandidateSection":"word-completion-json", "WordCompletionMorphismBundle":"architecture-validation-json"}, "Prompt-Completion und Word-Completion-Validierung kommutieren über derselben Completion-Kandidatensektion.", "reta_architecture/completion_word.py", "Schützt die aktivierte Completion-Schicht gegen spätere nestedAlx-/retaPrompt-Entkernungen."),
    )



def _stage41_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ActivatedNestedCompletionCategory",
            description="Kategorie der Stage-41 aktivierten nestedAlx-Hierarchiecompletion: Reta-Anfang, Hauptparameter, Subparameter, Gleichheits-/Kommawerte und Prompt-Kandidatensektionen werden durch Architektur-Morphismen statt durch libs/nestedAlx.py besessen.",
            objects=(
                _obj("NestedCompletionMorphismBundle", "reta_architecture.completion_nested", "aktivierter Architektur-Besitzer der nestedAlx-Hierarchiecompletion"),
                _obj("NestedCompletionOpenSet", "ArchitectureNestedCompleter.matchTextAlx", "topologische Einschränkung des Prompt-Dokuments auf eine Completion-Situation"),
                _obj("NestedOptionSection", "ArchitectureNestedCompleter.options", "lokale Completion-Sektion aus Befehlen, Parametern oder Werten"),
                _obj("NestedCompletionCandidateSection", "ArchitectureNestedCompleter.get_completions", "aus prompt_toolkit Completion-Objekten bestehende Ausgabesektion"),
            ),
            morphisms=(
                _mor("select_nested_open_set", "PromptDocument", "NestedCompletionOpenSet", "reta_architecture.completion_nested.ArchitectureNestedCompleter.matchTextAlx", "wählt die passende Completion-Situation aus"),
                _mor("glue_equality_value_options", "NestedOptionSection", "NestedOptionSection", "gleichKommaSpalten/Zeilen/Kombi/Ausg", "klebt runtime vocabulary, i18n und Werteoptionen zur lokalen Wertsektion"),
                _mor("yield_nested_candidates", "NestedOptionSection", "NestedCompletionCandidateSection", "ArchitectureNestedCompleter.get_completions", "erzeugt Completion-Kandidaten für den eingeschränkten Prompt-Kontext"),
                _mor("delegate_nested_completer", "libs.nestedAlx", "NestedCompletionMorphismBundle", "libs/nestedAlx.py", "alte nestedAlx-Importfläche delegiert an die aktivierte Architekturklasse"),
            ),
            implemented_by=("reta_architecture/completion_nested.py", "libs/nestedAlx.py"),
        ),
    )


def _stage41_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("NestedCompletionActivationFunctor", "ArchitectureActivationCategory", "ActivatedNestedCompletionCategory", "covariant", {"ActivationUnitSpec":"NestedCompletionMorphismBundle"}, {"activate_move":"delegate_nested_completer"}, "reta_architecture/completion_nested.py", "Aktiviert den nächsten Stage-36-Commit-Umschlag: nestedAlx wird reale Architektur-Completion-Logik."),
        FunctorSpec("LegacyNestedCompleterCompatibilityFunctor", "LegacyFacadeCategory", "ActivatedNestedCompletionCategory", "covariant", {"libs.nestedAlx.NestedCompleter":"ArchitectureNestedCompleter", "libs.nestedAlx.ComplSitua":"ComplSitua"}, {"legacy_call":"get_completions"}, "libs/nestedAlx.py", "Liest den alten nestedAlx-Import als Fassade über die Architektur-Nested-Completion-Klasse."),
        FunctorSpec("NestedCompletionPromptFunctor", "ActivatedNestedCompletionCategory", "LocalSectionCategory", "covariant", {"NestedCompletionCandidateSection":"PromptCompletionSection"}, {"yield_nested_candidates":"prompt_completion"}, "reta_architecture/completion_nested.py", "Transportiert hierarchische Completion-Kandidaten zurück in die Prompt-/Input-Kapsel."),
        FunctorSpec("NestedCompletionValidationFunctor", "ActivatedNestedCompletionCategory", "ArchitectureValidationCategory", "covariant", {"NestedCompletionMorphismBundle":"ArchitectureValidationCheckSpec"}, {"yield_nested_candidates":"validate"}, "reta_architecture/architecture_validation.py", "Macht die aktivierte nestedAlx-Migration prüfbar."),
    )


def _stage41_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("NestedCompleterToArchitectureTransformation", "LegacyNestedCompleterCompatibilityFunctor", "NestedCompletionActivationFunctor", {"NestedCompleter":"ArchitectureNestedCompleter", "ComplSitua":"ComplSitua", "get_completions":"yield_nested_candidates"}, "Erst über libs.nestedAlx.NestedCompleter instanziieren und dann hierarchisch completieren ergibt dieselbe Kandidatensektion wie der direkte NestedCompletionMorphismBundle-Pfad.", "reta_architecture/completion_nested.py + libs/nestedAlx.py", "Stage 41 verbindet die alte nestedAlx-Fassade mit der aktivierten Architektur-Nested-Completion-API."),
        NaturalTransformationSpec("NestedCompletionValidationTransformation", "NestedCompletionPromptFunctor", "NestedCompletionValidationFunctor", {"NestedCompletionCandidateSection":"nested-completion-json", "NestedCompletionMorphismBundle":"architecture-validation-json"}, "Nested Prompt Completion und Nested-Completion-Validierung kommutieren über derselben Completion-Kandidatensektion.", "reta_architecture/completion_nested.py", "Schützt die aktivierte hierarchische Completion-Schicht gegen spätere retaPrompt-/LibRetaPrompt-Entkernungen."),
    )


def _stage43_categories() -> tuple[CategorySpec, ...]:
    return (
        CategorySpec(
            name="ExecutionNetworkCategory",
            description="Kategorie der reinen Ausführungsnetzwerke über den mathematischen Schichten; Tasks, Queues, Worker und deterministisches Resultat-Gluing.",
            objects=(
                _obj("ExecutionTask", "reta_architecture.execution_network.ExecutionTask", "lokaler ausführbarer Task über einem bereits definierten Morphismus"),
                _obj("ExecutionQueue", "reta_architecture.execution_network.FifoTaskQueue", "FIFO/LIFO/Priority Scheduling-Oberfläche"),
                _obj("ExecutionRunResult", "reta_architecture.execution_network.ExecutionRunResult", "deterministisch reduzierte Task-Ergebnisse"),
                _obj("ExecutionNetworkBundle", "reta_architecture.execution_network.ExecutionNetworkBundle", "Ausführungsnetzwerk mit Ressourcenlimits"),
            ),
            morphisms=(
                _mor("enqueue_task", "ExecutionTask", "ExecutionQueue", "FifoTaskQueue.push", "Task in Warteschlange einreihen"),
                _mor("dequeue_task", "ExecutionQueue", "ExecutionTask", "FifoTaskQueue.pop", "Task aus Warteschlange entnehmen"),
                _mor("dispatch_task", "ExecutionTask", "ExecutionRunResult", "execute_tasks_deterministically", "Task seriell oder prozessbasiert ausführen"),
                _mor("deterministic_reduce", "ExecutionRunResult", "OrderedResultSection", "deterministic_reduce", "Taskresultate in stabiler Originalreihenfolge kleben"),
            ),
            implemented_by=("reta_architecture/execution_network.py",),
        ),
        CategorySpec(
            name="SchedulerCategory",
            description="Kategorie begrenzter Scheduler-Ressourcen: FIFO, LIFO, PriorityQueue und Semaphoren als reine Runtime-Mechanik.",
            objects=(
                _obj("FifoQueue", "reta_architecture.execution_network.FifoTaskQueue", "faire Arbeitswarteschlange"),
                _obj("LifoStack", "reta_architecture.execution_network.LifoTaskStack", "Stack für Tiefensuche/Nested-Kontexte"),
                _obj("PriorityQueue", "reta_architecture.execution_network.PriorityTaskQueue", "priorisierte Arbeitswarteschlange"),
                _obj("ResourceSemaphore", "reta_architecture.execution_network.ResourceSemaphore", "prozessfähige Ressourcenbegrenzung"),
            ),
            morphisms=(
                _mor("push", "ExecutionTask", "SchedulerQueue", "FifoTaskQueue.push / LifoTaskStack.push / PriorityTaskQueue.push", "Task in Scheduler einfügen"),
                _mor("pop", "SchedulerQueue", "ExecutionTask", "FifoTaskQueue.pop / LifoTaskStack.pop / PriorityTaskQueue.pop", "nächsten Task nach Disziplin entnehmen"),
                _mor("acquire_resource", "ResourceSemaphore", "ResourceLease", "ResourceSemaphore.acquire", "knappe Ressource reservieren"),
                _mor("release_resource", "ResourceLease", "ResourceSemaphore", "ResourceSemaphore.release", "knappe Ressource freigeben"),
            ),
            implemented_by=("reta_architecture/execution_network.py",),
        ),
        CategorySpec(
            name="ChannelCategory",
            description="Kategorie von Halfduplex- und Fullduplex-Kanälen für Supervisor/Worker- und Prompt-Kommunikation.",
            objects=(
                _obj("HalfDuplexChannel", "reta_architecture.execution_network.HalfDuplexChannel", "Request/Response-Kanal"),
                _obj("FullDuplexChannel", "reta_architecture.execution_network.FullDuplexChannel", "bidirektionaler Fortschritt-/Cancel-Kanal"),
                _obj("ChannelMessage", "reta_architecture.execution_network", "serialisierbare Worker-/Prompt-Nachricht"),
            ),
            morphisms=(
                _mor("send_message", "ChannelMessage", "HalfDuplexChannel", "send_request / send_response", "Nachricht senden"),
                _mor("receive_message", "HalfDuplexChannel", "ChannelMessage", "receive_request / receive_response", "Nachricht empfangen"),
                _mor("stream_progress", "FullDuplexChannel", "ChannelMessage", "send_b_to_a", "Fortschritt in Gegenrichtung senden"),
                _mor("cancel_message", "ChannelMessage", "FullDuplexChannel", "send_a_to_b", "Cancel/Abort-Nachricht senden"),
            ),
            implemented_by=("reta_architecture/execution_network.py",),
        ),
        CategorySpec(
            name="PersistenceCategory",
            description="Kategorie der SQLite-Materialisierung, Auditierung und Cache-Snapshots; speichert Instanzen ohne die mathematische Semantik zu definieren.",
            objects=(
                _obj("PersistentLocalSection", "reta_architecture.persistence.persist_section", "persistierte lokale Prägarbensektion"),
                _obj("PersistentSheafSnapshot", "reta_architecture.persistence.persist_sheaf_snapshot", "persistierter Garben-Snapshot"),
                _obj("PersistentExecutionRun", "reta_architecture.persistence.persist_execution_run", "persistierter Ausführungslauf"),
                _obj("AuditEvent", "reta_architecture.persistence.record_audit_event", "Auditereignis mit stabiler Payload-Prüfsumme"),
                _obj("CacheEntry", "reta_architecture.persistence.cache_put", "validierbarer materialisierter Cacheeintrag"),
            ),
            morphisms=(
                _mor("persist_section", "LocalSection", "PersistentLocalSection", "reta_architecture.persistence.persist_section", "lokale Sektion materialisieren"),
                _mor("load_section", "PersistentLocalSection", "LocalSection", "reta_architecture.persistence.load_section", "lokale Sektion laden"),
                _mor("persist_sheaf_snapshot", "SheafBundle", "PersistentSheafSnapshot", "reta_architecture.persistence.persist_sheaf_snapshot", "Garbensnapshot materialisieren"),
                _mor("persist_execution_run", "ExecutionRunResult", "PersistentExecutionRun", "reta_architecture.persistence.persist_execution_run", "Ausführungslauf auditierbar speichern"),
                _mor("record_audit_event", "ArchitectureValidationBundle", "AuditEvent", "reta_architecture.persistence.record_audit_event", "Validierungs-/Paritätsereignis auditieren"),
                _mor("cache_put", "GlobalResult", "CacheEntry", "reta_architecture.persistence.cache_put", "gültiges Resultat cachen"),
                _mor("cache_get", "CacheEntry", "GlobalResult", "reta_architecture.persistence.cache_get", "gültiges Cache-Resultat laden"),
                _mor("invalidate_cache", "CacheEntry", "CacheEntry", "reta_architecture.persistence.invalidate_cache", "Cacheeintrag ungültig markieren"),
            ),
            implemented_by=("reta_architecture/persistence.py",),
        ),
    )


def _stage43_functors() -> tuple[FunctorSpec, ...]:
    return (
        FunctorSpec("TableChunkExecutionFunctor", "TableSectionCategory", "ExecutionNetworkCategory", "covariant", {"Tables":"ExecutionTask", "TableStateSections":"ExecutionRunResult"}, {"prepare_output_table":"dispatch_task", "filter_original_lines":"enqueue_task"}, "reta_architecture/execution_network.py", "Hebt chunkbare Tabellenarbeit in ein reines Ausführungsnetzwerk."),
        FunctorSpec("ExecutionResultGluingFunctor", "ExecutionNetworkCategory", "TableSectionCategory", "covariant", {"ExecutionRunResult":"TableStateSections", "OrderedResultSection":"Tables"}, {"deterministic_reduce":"prepare_output_table"}, "reta_architecture/execution_network.py", "Klebt parallel/seriell ausgeführte Taskresultate deterministisch zurück in Tabellen-Sektionen."),
        FunctorSpec("SchedulerResourceFunctor", "SchedulerCategory", "ExecutionNetworkCategory", "covariant", {"FifoQueue":"ExecutionQueue", "LifoStack":"ExecutionQueue", "PriorityQueue":"ExecutionQueue", "ResourceSemaphore":"ExecutionNetworkBundle"}, {"push":"enqueue_task", "pop":"dequeue_task", "acquire_resource":"dispatch_task", "release_resource":"collect_result"}, "reta_architecture/execution_network.py", "Übersetzt Queue-/Stack-/Semaphore-Mechanik in ausführbare Task-Netzwerke."),
        FunctorSpec("ChannelPromptFunctor", "ChannelCategory", "LocalSectionCategory", "covariant", {"ChannelMessage":"LocalSection", "HalfDuplexChannel":"PromptStatePresheaf", "FullDuplexChannel":"PromptStatePresheaf"}, {"send_message":"update_prompt_state", "receive_message":"restrict"}, "reta_architecture/execution_network.py", "Transportiert Kanalnachrichten als lokale Prompt-/Input-Sektionen."),
        FunctorSpec("PresheafPersistenceFunctor", "LocalSectionCategory", "PersistenceCategory", "covariant", {"LocalSection":"PersistentLocalSection", "FilesystemPresheaf":"PersistentLocalSection", "PromptStatePresheaf":"PersistentLocalSection"}, {"add_section":"persist_section", "restrict":"load_section", "update_prompt_state":"persist_section"}, "reta_architecture/persistence.py", "Materialisiert lokale Prägarbensektionen in SQLite."),
        FunctorSpec("SheafPersistenceFunctor", "CanonicalSemanticSheafCategory", "PersistenceCategory", "covariant", {"ParameterSemanticsSheaf":"PersistentSheafSnapshot", "GeneratedColumnsSheaf":"PersistentSheafSnapshot", "TableOutputSheaf":"PersistentSheafSnapshot", "HtmlReferenceSheaf":"PersistentSheafSnapshot"}, {"canonicalize_pair":"persist_sheaf_snapshot", "sync_from_tables":"persist_sheaf_snapshot"}, "reta_architecture/persistence.py", "Materialisiert geklebte Garben-Snapshots in SQLite."),
        FunctorSpec("TableStatePersistenceFunctor", "TableSectionCategory", "PersistenceCategory", "covariant", {"Tables":"PersistentExecutionRun", "TableStateSections":"PersistentSheafSnapshot"}, {"prepare_output_table":"persist_execution_run", "render_table_output":"persist_sheaf_snapshot"}, "reta_architecture/persistence.py", "Speichert Tabellenzustände und Ausführungsläufe auditierbar."),
        FunctorSpec("AuditValidationPersistenceFunctor", "ArchitectureValidationCategory", "PersistenceCategory", "covariant", {"ArchitectureValidationBundle":"AuditEvent", "ArchitectureValidationCheckSpec":"AuditEvent"}, {"bootstrap_architecture_validation":"record_audit_event", "render_validation_diagram":"record_audit_event"}, "reta_architecture/persistence.py", "Persistiert Validierungs- und Paritätsereignisse als Auditlog."),
        FunctorSpec("CacheMaterializationFunctor", "PersistenceCategory", "TableSectionCategory", "covariant", {"CacheEntry":"Tables", "PersistentExecutionRun":"TableStateSections"}, {"cache_get":"prepare_output_table", "invalidate_cache":"filter_original_lines"}, "reta_architecture/persistence.py", "Lädt gültige materialisierte Resultate zurück in Tabellenpfade."),
        FunctorSpec("PersistenceAuditFunctor", "PersistenceCategory", "ArchitectureValidationCategory", "covariant", {"AuditEvent":"ArchitectureValidationCheckSpec", "PersistentSheafSnapshot":"ArchitectureValidationBundle"}, {"record_audit_event":"validate", "query_audit_events":"render_validation_diagram"}, "reta_architecture/persistence.py", "Reflektiert Persistenzereignisse in die Validierungs-/Audit-Schicht."),
        FunctorSpec("RowFilterProcessFunctor", "TableSectionCategory", "ExecutionNetworkCategory", "covariant", {"RowSet":"ExecutionTask", "TableStateSections":"ExecutionRunResult"}, {"filter_original_lines":"dispatch_task", "prepare_output_table":"deterministic_reduce"}, "reta_architecture/row_filtering.py + reta_architecture/parallel_execution.py", "Hebt reine Zeilenfilter-Scans wie Primfaktoren, Sonnenanteile und Vielfache in PyPy3-Prozesschunks."),
        FunctorSpec("ArithmeticBatchExecutionFunctor", "ActivatedArithmeticCategory", "ExecutionNetworkCategory", "covariant", {"ArithmeticExpression":"ExecutionTask", "FactorPairSet":"ExecutionRunResult", "PrimeFactorSection":"ExecutionRunResult"}, {"factor_pairs":"dispatch_task", "prime_factorize":"dispatch_task", "glue_divisor_range":"deterministic_reduce"}, "reta_architecture/arithmetic.py + reta_architecture/parallel_execution.py", "Hebt pure Arithmetikbatches in prozessbasierte Chunk-Ausführung."),
        FunctorSpec("PackageIntegrityExecutionFunctor", "ArchitectureValidationCategory", "ExecutionNetworkCategory", "covariant", {"ArchitectureValidationCheckSpec":"ExecutionTask", "RepositoryManifest":"ExecutionRunResult"}, {"bootstrap_architecture_validation":"dispatch_task", "validate_stage31_references":"deterministic_reduce"}, "reta_architecture/package_integrity.py", "Berechnet Manifest-Hashes und CSV-Zählungen in prozessbasierten Datei-Chunks."),
        FunctorSpec("PersistenceBatchPreparationFunctor", "PersistenceCategory", "ExecutionNetworkCategory", "covariant", {"PersistentLocalSection":"ExecutionTask", "PersistentSheafSnapshot":"ExecutionTask", "CacheEntry":"ExecutionTask"}, {"persist_section":"dispatch_task", "persist_sheaf_snapshot":"dispatch_task", "cache_put":"dispatch_task"}, "reta_architecture/persistence.py", "Bereitet JSON/Digests großer Persistenzbatches in Prozessen vor; SQLite-Schreibpfad bleibt seriell."),
        FunctorSpec("ProcessExecutionAuditFunctor", "ExecutionNetworkCategory", "PersistenceCategory", "covariant", {"ExecutionRunResult":"PersistentExecutionRun", "OrderedResultSection":"AuditEvent"}, {"deterministic_reduce":"persist_execution_run", "dispatch_task":"record_audit_event"}, "reta_architecture/execution_network.py + reta_architecture/persistence.py", "Spiegelt prozessbasierte Ausführungsläufe in Persistenz und Audit."),
    )


def _stage43_natural_transformations() -> tuple[NaturalTransformationSpec, ...]:
    return (
        NaturalTransformationSpec("ParallelExecutionNaturalityTransformation", "TableChunkExecutionFunctor", "ExecutionResultGluingFunctor", {"ChunkTask":"ExecutionTask", "ChunkResult":"OrderedResultSection", "TableStateSections":"Tables"}, "Parallel oder seriell ausgeführte Chunks kleben nach deterministischer Reduktion zur selben Tabellensektion.", "reta_architecture/execution_network.py", "Schützt die Netzwerkmechanik: Ausführungsreihenfolge darf die Ausgabe nicht verändern."),
        NaturalTransformationSpec("SchedulerExecutionNaturalityTransformation", "SchedulerResourceFunctor", "TableChunkExecutionFunctor", {"FifoQueue":"ExecutionQueue", "ResourceSemaphore":"ExecutionNetworkBundle"}, "Scheduler-Disziplin und Ressourcenbegrenzung verändern nur die Ausführungsreihenfolge, nicht das deterministisch geklebte Resultat.", "reta_architecture/execution_network.py", "Hält Queues, Stacks und Semaphoren aus der Semantik heraus."),
        NaturalTransformationSpec("ChannelPromptNaturalityTransformation", "ChannelPromptFunctor", "RawCommandPresheafFunctor", {"ChannelMessage":"PromptText", "PromptStatePresheaf":"LocalSection"}, "Kanaltransport eines Prompt-Befehls und direkte Rohkommando-Prägarbe führen zu derselben lokalen Prompt-Sektion.", "reta_architecture/execution_network.py", "Schützt Halfduplex/Fullduplex-Mechanik gegen Semantik-Verschiebung."),
        NaturalTransformationSpec("PresheafPersistenceRoundTripTransformation", "PresheafPersistenceFunctor", "LocalDataPresheafFunctor", {"LocalSection":"PersistentLocalSection", "FilesystemPresheaf":"PersistentLocalSection"}, "Persistieren und Laden einer lokalen Sektion liefert bei gleicher Prüfsumme dieselbe lokale Prägarbensektion.", "reta_architecture/persistence.py", "Datenbankmaterialisierung ersetzt nicht die Prägarbe, sondern rundet sie auditierbar ab."),
        NaturalTransformationSpec("SheafPersistenceRoundTripTransformation", "SheafPersistenceFunctor", "GluedSemanticSheafFunctor", {"SheafBundle":"PersistentSheafSnapshot", "ParameterSemanticsSheaf":"PersistentSheafSnapshot"}, "Persistieren und Laden eines Garben-Snapshots liefert bei gleicher Prüfsumme dieselbe geklebte Semantik.", "reta_architecture/persistence.py", "Datenbankmaterialisierung ersetzt nicht die Garbe, sondern speichert Snapshots."),
        NaturalTransformationSpec("TableStatePersistenceTransformation", "TableStatePersistenceFunctor", "ExplicitTableStateFunctor", {"TableStateSections":"PersistentSheafSnapshot", "Tables":"PersistentExecutionRun"}, "Expliziter Tabellenzustand und persistierter Tabellen-Snapshot kommutieren, solange Kontext- und Payload-Hashes gleich bleiben.", "reta_architecture/persistence.py", "Macht Tabellenläufe auditierbar, ohne Mutable-State-Semantik zu ändern."),
        NaturalTransformationSpec("CacheCoherenceTransformation", "CacheMaterializationFunctor", "TableGenerationGluingFunctor", {"CacheEntry":"Tables", "ParameterDictionaryDiagram":"PersistentSheafSnapshot"}, "Ein Cache-Hit darf nur denselben Tabellen-Zielpunkt liefern wie erneutes universelles Gluing mit identischen Kontext-/Sektionshashes.", "reta_architecture/persistence.py", "Schützt Execution-Cache gegen falsche Wiederverwendung."),
        NaturalTransformationSpec("AuditPersistenceValidationTransformation", "AuditValidationPersistenceFunctor", "PersistenceAuditFunctor", {"ArchitectureValidationBundle":"AuditEvent", "ArchitectureValidationCheckSpec":"AuditEvent"}, "Audit-Ereignisse aus Validierung und Persistenzabfragen beschreiben denselben prüfbaren Lauf.", "reta_architecture/persistence.py", "Verbindet Datenbank-Audit und Architekturvalidierung."),
        NaturalTransformationSpec("RowFilterProcessNaturalityTransformation", "RowFilterProcessFunctor", "TableChunkExecutionFunctor", {"RowSet":"ExecutionTask", "ChunkResult":"OrderedResultSection"}, "Zeilenfilter seriell oder in PyPy3-Prozesschunks liefern dieselbe RowSet-Sektion, bevor die Tabelle vorbereitet wird.", "reta_architecture/row_filtering.py + reta_architecture/parallel_execution.py", "Schützt die zusätzliche Prozessparallelisierung der Zeilenfilter gegen Semantikverschiebung."),
        NaturalTransformationSpec("ArithmeticBatchProcessNaturalityTransformation", "ArithmeticBatchExecutionFunctor", "ArithmeticRowRangeGluingFunctor", {"ArithmeticExpression":"ExecutionTask", "DivisorSection":"OrderedResultSection"}, "Arithmetikbatches über Prozesse und direkte RowRange-Arithmetik-Gluing-Pfade führen zu derselben Faktor-/Teilersektion.", "reta_architecture/arithmetic.py + reta_architecture/parallel_execution.py", "Schützt prozessbasierte Faktor- und Primfaktorberechnung."),
        NaturalTransformationSpec("PackageIntegrityProcessNaturalityTransformation", "PackageIntegrityExecutionFunctor", "WitnessToValidationFunctor", {"RepositoryManifest":"ArchitectureValidationCheckSpec", "FileDigest":"AuditEvent"}, "Serielle Manifestberechnung und prozessbasierte Datei-Chunk-Berechnung ergeben denselben RepositoryManifest-Digest.", "reta_architecture/package_integrity.py", "Macht Manifest-Parallelisierung auditierbar, ohne Paketvalidierung zu verändern."),
        NaturalTransformationSpec("PersistenceBatchPreparationNaturalityTransformation", "PersistenceBatchPreparationFunctor", "PresheafPersistenceFunctor", {"PersistentLocalSection":"LocalSection", "CacheEntry":"PersistentLocalSection"}, "Batchweise Digest-/JSON-Vorbereitung in Prozessen und direkte Persistenz einzelner Sektionen liefern bei gleicher Prüfsumme dieselben persistierten Datensätze.", "reta_architecture/persistence.py", "Schützt Batch-Persistenz gegen abweichende Datenbankmaterialisierung."),
        NaturalTransformationSpec("ProcessExecutionAuditNaturalityTransformation", "ProcessExecutionAuditFunctor", "TableStatePersistenceFunctor", {"ExecutionRunResult":"PersistentExecutionRun", "OrderedResultSection":"PersistentSheafSnapshot"}, "Persistierte Prozessläufe und persistierte Tabellenzustände beschreiben denselben deterministisch reduzierten Lauf, solange Kontext- und Payload-Hashes gleich sind.", "reta_architecture/persistence.py", "Verbindet neue Prozessparallelisierung mit Audit/Persistenz."),
    )


def bootstrap_category_theory() -> CategoryTheoryBundle:
    """Return the Stage-27 categorical architecture metadata."""

    return CategoryTheoryBundle(
        categories=_categories() + _stage32_categories() + _stage33_categories() + _stage34_categories() + _stage35_categories() + _stage36_categories() + _stage37_categories() + _stage38_categories() + _stage39_categories() + _stage40_categories() + _stage41_categories() + _stage43_categories(),
        functors=_functors() + _stage32_functors() + _stage33_functors() + _stage34_functors() + _stage35_functors() + _stage36_functors() + _stage37_functors() + _stage38_functors() + _stage39_functors() + _stage40_functors() + _stage41_functors() + _stage43_functors(),
        natural_transformations=_natural_transformations() + _stage32_natural_transformations() + _stage33_natural_transformations() + _stage34_natural_transformations() + _stage35_natural_transformations() + _stage36_natural_transformations() + _stage37_natural_transformations() + _stage38_natural_transformations() + _stage39_natural_transformations() + _stage40_natural_transformations() + _stage41_natural_transformations() + _stage43_natural_transformations(),
        paradigm_terms=_paradigm_terms(),
        plan=_plan(),
    )
