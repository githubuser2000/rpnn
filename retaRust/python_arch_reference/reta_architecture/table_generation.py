from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict

from .generated_columns import GeneratedColumnsBundle, bootstrap_generated_columns
from .concat_csv import ConcatCsvBundle, bootstrap_concat_csv
from .combi_join import KombiJoinBundle, bootstrap_combi_join


@dataclass
class TableGenerationResult:
    animals_professions_table: list = field(default_factory=list)
    rows_of_combi: object = None
    kombi_table_kombis: list = field(default_factory=list)
    maintable2subtable_relation: list = field(default_factory=list)
    prim_spalten: object = None
    gebr: Dict[str, object] = field(default_factory=dict)
    animals_professions_table2: list = field(default_factory=list)
    kombi_table_kombis2: list = field(default_factory=list)
    maintable2subtable_relation2: list = field(default_factory=list)

    def snapshot(self) -> dict:
        return {
            "class": "TableGenerationResult",
            "has_prim_spalten": self.prim_spalten is not None,
            "gebr_keys": sorted(self.gebr.keys()),
            "kombi_rows_len": len(self.rows_of_combi) if self.rows_of_combi is not None else 0,
            "animals_professions_table_len": len(self.animals_professions_table),
            "animals_professions_table2_len": len(self.animals_professions_table2),
        }


@dataclass(frozen=True)
class TableGenerationBundle:
    """Universal table-gluing layer for reta's main Program workflow.

    It keeps the legacy `Concat`, `Prepare`, and `Combi` algorithms intact, but
    lifts their orchestration out of `reta.py`. This is the explicit place where
    CSV presheaf sections, generated-column sheaves, and Kombi joins are glued
    into the main table section.
    """

    csv_file_names: object
    generated_columns: GeneratedColumnsBundle
    concat_csv: ConcatCsvBundle
    combi_join: KombiJoinBundle

    def _concat_csv_inputs(self, program) -> tuple[object, Dict[str, object]]:
        naming = program.spaltenTypeNaming
        buckets = program.spaltenArtenKey_SpaltennummernValue
        csv_theirs_spalten: dict[int, object] = {}
        for index, input_section in enumerate(
            (
                program.puniverseprims,
                buckets[naming.gebrGal1],
                buckets[naming.gebrGal1],
                buckets[naming.gebroUni1],
                buckets[naming.gebroUni1],
                buckets[naming.gebrEmo1],
                buckets[naming.gebrEmo1],
                buckets[naming.gebrGroe1],
                buckets[naming.gebrGroe1],
            ),
            start=1,
        ):
            (
                program.relitable,
                _rows_as_numbers,
                csv_theirs_spalten[index],
            ) = self.concat_csv.read_concat_csv(
                program.tables.getConcat,
                program.relitable,
                program.rowsAsNumbers,
                input_section,
                index,
            )
        prim_spalten = csv_theirs_spalten[1]
        gebr = {
            "Gal": csv_theirs_spalten[2],
            "Gal2": csv_theirs_spalten[3],
            "Uni": csv_theirs_spalten[4],
            "Uni2": csv_theirs_spalten[5],
            "Emo": csv_theirs_spalten[6],
            "Emo2": csv_theirs_spalten[7],
            "Groe": csv_theirs_spalten[8],
            "Groe2": csv_theirs_spalten[9],
        }
        return prim_spalten, gebr

    def _set_last_line_number(self, program, param_lines, param_lines_not) -> None:
        program.architecture.bootstrap_table_preparation().capture_last_line_number(
            program.tables,
            program.relitable,
            param_lines,
            param_lines_not,
        )

    def _apply_generated_column_morphisms(self, program) -> None:
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_vervielfache_zeile(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
        )
        program.relitable, program.rowsAsNumbers = self.generated_columns.concat_modallogik(
            program.tables.getConcat,
            program.relitable,
            program.tables.generRows,
            program.rowsAsNumbers,
        )
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_prim_creativity_type(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
        )
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_gleichheit_freiheit_dominieren(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
        )
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_geist_emotion_energie_materie_topologie(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
        )
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_mond_exponzieren_logarithmus_typ(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
        )

        para_text_namen = {}
        for text in program.spaltenArtenKey_SpaltennummernValue[(0, 7)]:
            para_text_namen[text] = [program.dataDict[7][text]]

        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_prim_universe_row(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
            program.spaltenArtenKey_SpaltennummernValue[(0, 7)],
            para_text_namen,
        )
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_primzahlkreuz_pro_contra(
            program.tables.getConcat,
            program.relitable,
            program.rowsAsNumbers,
            program.spaltenArtenKey_SpaltennummernValue[(0, 7)],
            program.ParametersMain,
        )
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = self.generated_columns.concat_love_polygon(program.tables.getConcat, program.relitable, program.rowsAsNumbers)
        (
            program.relitable,
            _rows_as_numbers,
        ) = program.tables.getConcat.spalteFuerGegenInnenAussenSeitlichPrim(
            program.relitable,
            program.rowsAsNumbers,
        )

        couples_x = list(program.spaltenArtenKey_SpaltennummernValue[(0, 11)])
        (
            program.relitable,
            program.rowsAsNumbers,
        ) = program.tables.getConcat.spalteMetaKontretTheorieAbstrakt_etc_1(
            program.relitable,
            program.rowsAsNumbers,
            couples_x,
        )
        self.generated_columns.create_spalte_gestirn(program.tables, program.relitable, program.rowsAsNumbers)

    def _read_kombi_tables(self, program) -> tuple[list, list, list, list, list, list]:
        if len(program.rowsOfcombi) > 0:
            (
                animals_professions_table,
                program.relitable,
                kombi_table_kombis,
                maintable2subtable_relation,
            ) = program.tables.getCombis.readKombiCsv(
                program.relitable,
                program.rowsAsNumbers,
                program.rowsOfcombi,
                self.csv_file_names.kombi13,
            )
        else:
            animals_professions_table = []
            kombi_table_kombis = []
            maintable2subtable_relation = []

        if len(program.rowsOfcombi2) > 0:
            (
                animals_professions_table2,
                program.relitable,
                kombi_table_kombis2,
                maintable2subtable_relation2,
            ) = program.tables.getCombis.readKombiCsv(
                program.relitable,
                program.rowsAsNumbers,
                program.rowsOfcombi2,
                self.csv_file_names.kombi15,
            )
        else:
            animals_professions_table2 = []
            kombi_table_kombis2 = []
            maintable2subtable_relation2 = []

        return (
            animals_professions_table,
            kombi_table_kombis,
            maintable2subtable_relation,
            animals_professions_table2,
            kombi_table_kombis2,
            maintable2subtable_relation2,
        )

    def build_for_program(self, program, param_lines, param_lines_not) -> TableGenerationResult:
        prim_spalten, gebr = self._concat_csv_inputs(program)
        self._set_last_line_number(program, param_lines, param_lines_not)
        self._apply_generated_column_morphisms(program)
        (
            animals_professions_table,
            kombi_table_kombis,
            maintable2subtable_relation,
            animals_professions_table2,
            kombi_table_kombis2,
            maintable2subtable_relation2,
        ) = self._read_kombi_tables(program)
        program.architecture.sync_tables(program.tables)
        return TableGenerationResult(
            animals_professions_table=animals_professions_table,
            rows_of_combi=program.rowsOfcombi,
            kombi_table_kombis=kombi_table_kombis,
            maintable2subtable_relation=maintable2subtable_relation,
            prim_spalten=prim_spalten,
            gebr=gebr,
            animals_professions_table2=animals_professions_table2,
            kombi_table_kombis2=kombi_table_kombis2,
            maintable2subtable_relation2=maintable2subtable_relation2,
        )

    def snapshot(self) -> dict:
        return {
            "class": "TableGenerationBundle",
            "csv_sources": ["prim", "gebrGal", "gebroUni", "gebrEmo", "gebrGroe"],
            "generated_columns_registry": self.generated_columns.snapshot(),
            "concat_csv": self.concat_csv.snapshot(),
            "combi_join": self.combi_join.snapshot(),
            "generated_morphisms": [
                "concatVervielfacheZeile",
                "concatModallogik",
                "concatPrimCreativityType",
                "concatGleichheitFreiheitDominieren",
                "concatGeistEmotionEnergieMaterieTopologie",
                "concatMondExponzierenLogarithmusTyp",
                "concat1RowPrimUniverse2",
                "concat1PrimzahlkreuzProContra",
                "concatLovePolygon",
                "spalteFuerGegenInnenAussenSeitlichPrim",
                "spalteMetaKontretTheorieAbstrakt_etc_1",
                "createSpalteGestirn",
            ],
            "table_preparation_dependency": "capture_last_line_number",
            "kombi_csvs": [
                getattr(self.csv_file_names, "kombi13", None),
                getattr(self.csv_file_names, "kombi15", None),
            ],
        }


def bootstrap_table_generation(csv_file_names, generated_columns=None, concat_csv=None, combi_join=None) -> TableGenerationBundle:
    return TableGenerationBundle(
        csv_file_names=csv_file_names,
        generated_columns=generated_columns or bootstrap_generated_columns(),
        concat_csv=concat_csv or bootstrap_concat_csv(),
        combi_join=combi_join or bootstrap_combi_join(),
    )
