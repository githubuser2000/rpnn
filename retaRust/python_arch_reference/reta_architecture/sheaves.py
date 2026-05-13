from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Iterable, List, Mapping, Optional, Sequence, Tuple

from .schema import RetaContextSchema


@dataclass
class ParameterSemanticsSheaf:
    main_alias_map: Dict[str, str] = field(default_factory=dict)
    main_alias_groups: List[Dict[str, object]] = field(default_factory=list)
    parameter_alias_groups: Dict[str, List[Dict[str, object]]] = field(default_factory=dict)
    pair_to_columns: Dict[Tuple[str, str], List[int]] = field(default_factory=dict)
    parameters_main: List[Tuple[str, ...]] = field(default_factory=list)
    para_n_data_matrix: List[tuple] = field(default_factory=list)
    kombi_para_n_data_matrix: object = None
    kombi_para_n_data_matrix2: object = None
    global_parameter_dict: Dict[Tuple[str, str], object] = field(default_factory=dict)
    global_data_dicts: List[dict] = field(default_factory=list)

    @classmethod
    def from_schema(cls, schema: RetaContextSchema) -> "ParameterSemanticsSheaf":
        sheaf = cls()
        sheaf.parameters_main = [tuple(str(v) for v in group) for group in schema.parameters_main]
        sheaf.para_n_data_matrix = list(schema.para_n_data_matrix)
        sheaf.kombi_para_n_data_matrix = schema.kombi_para_n_data_matrix
        sheaf.kombi_para_n_data_matrix2 = schema.kombi_para_n_data_matrix2
        sheaf._rebuild_alias_maps()
        return sheaf

    def _rebuild_alias_maps(self) -> None:
        self.main_alias_map = {}
        self.main_alias_groups = []
        for group in self.parameters_main:
            aliases = [str(v) for v in group if v is not None]
            if not aliases:
                continue
            canonical = aliases[0]
            self.main_alias_groups.append({"canonical": canonical, "aliases": aliases})
            for alias in aliases:
                self.main_alias_map[str(alias)] = canonical

        parameter_groups: Dict[str, Dict[str, List[str]]] = {}
        pair_to_columns: Dict[Tuple[str, str], set] = {}
        for entry in self.para_n_data_matrix:
            if len(entry) < 3:
                continue
            main_aliases = tuple(str(x) for x in entry[0])
            parameter_aliases = tuple(str(x) for x in entry[1])
            datas = entry[2]
            if not main_aliases or not parameter_aliases:
                continue
            main_canonical = self.main_alias_map.get(main_aliases[0], main_aliases[0])
            parameter_canonical = parameter_aliases[0]
            parameter_groups.setdefault(main_canonical, {})
            parameter_groups[main_canonical].setdefault(parameter_canonical, [])
            for alias in parameter_aliases:
                if alias not in parameter_groups[main_canonical][parameter_canonical]:
                    parameter_groups[main_canonical][parameter_canonical].append(alias)
            cols = pair_to_columns.setdefault((main_canonical, parameter_canonical), set())
            try:
                cols |= {int(x) for x in datas}
            except TypeError:
                # Some matrix entries contain generated or callable semantics in
                # later positions. Only the direct-column slot belongs here.
                pass

        self.parameter_alias_groups = {
            main: [
                {"canonical": canonical, "aliases": sorted(aliases)}
                for canonical, aliases in sorted(groups.items(), key=lambda item: item[0])
            ]
            for main, groups in parameter_groups.items()
        }
        self.pair_to_columns = {
            key: sorted(value)
            for key, value in sorted(pair_to_columns.items(), key=lambda item: item[0])
        }

    def canonical_main_alias_groups(self) -> List[Dict[str, object]]:
        return list(self.main_alias_groups)

    def resolve_main_alias(self, main_name: str) -> Optional[str]:
        return self.main_alias_map.get(main_name)

    def parameter_alias_groups_for_main(self, main_name: str) -> List[Dict[str, object]]:
        canonical_main = self.resolve_main_alias(main_name) or main_name
        return list(self.parameter_alias_groups.get(canonical_main, []))

    def resolve_parameter_alias(self, main_name: str, parameter_name: str) -> Optional[str]:
        canonical_main = self.resolve_main_alias(main_name) or main_name
        for group in self.parameter_alias_groups.get(canonical_main, []):
            aliases = [str(v) for v in group["aliases"]]
            if parameter_name in aliases:
                return str(group["canonical"])
        return None

    def canonicalize_pair(self, main_name: str, parameter_name: str) -> Optional[Tuple[str, str]]:
        canonical_main = self.resolve_main_alias(main_name)
        if canonical_main is None:
            return None
        canonical_parameter = self.resolve_parameter_alias(canonical_main, parameter_name)
        if canonical_parameter is None:
            return None
        return canonical_main, canonical_parameter

    def column_numbers_for_pair(self, main_name: str, parameter_name: str) -> List[int]:
        pair = self.canonicalize_pair(main_name, parameter_name)
        if pair is None:
            return []
        return list(self.pair_to_columns.get(pair, []))

    def reverse_map_canonical_pairs(self) -> Dict[int, List[Tuple[str, str]]]:
        out: Dict[int, List[Tuple[str, str]]] = {}
        for pair, columns in self.pair_to_columns.items():
            for col in columns:
                out.setdefault(int(col), [])
                if pair not in out[int(col)]:
                    out[int(col)].append(pair)
        for column in out:
            out[column] = sorted(out[column], key=lambda item: (item[0], item[1]))
        return dict(sorted(out.items(), key=lambda item: item[0]))

    def exact_meta_for_column(self, column_number: int) -> List[Dict[str, object]]:
        matches: List[Dict[str, object]] = []
        for entry in self.para_n_data_matrix:
            if len(entry) < 3:
                continue
            main_aliases = [str(x) for x in entry[0]]
            parameter_aliases = [str(x) for x in entry[1]]
            datas = entry[2]
            try:
                if column_number not in datas:
                    continue
            except TypeError:
                continue
            if not main_aliases or not parameter_aliases:
                continue
            matches.append(
                {
                    "column_number": int(column_number),
                    "parameter_main": main_aliases[0],
                    "parameter_main_aliases": main_aliases,
                    "parameter": parameter_aliases[0],
                    "parameter_aliases": parameter_aliases,
                }
            )
        return sorted(matches, key=lambda item: (str(item["parameter_main"]), str(item["parameter"])))

    def sync_program_semantics(self, para_dict: Mapping[Tuple[str, str], object], data_dicts: Sequence[dict]) -> None:
        self.global_parameter_dict = dict(para_dict)
        self.global_data_dicts = [dict(d) for d in data_dicts]

    def snapshot(self) -> Dict[str, object]:
        return {
            "main_alias_groups": self.canonical_main_alias_groups(),
            "parameter_alias_groups": self.parameter_alias_groups,
            "pair_to_columns": {
                f"{main}::{parameter}": cols
                for (main, parameter), cols in self.pair_to_columns.items()
            },
            "global_parameter_dict_size": len(self.global_parameter_dict),
            "global_data_dict_sizes": [len(d) for d in self.global_data_dicts],
        }


@dataclass
class GeneratedColumnsSheaf:
    generated_spalten_parameter: Dict[int, object] = field(default_factory=dict)
    generated_spalten_parameter_tags: Dict[int, object] = field(default_factory=dict)

    def sync_from_tables(self, tables) -> None:
        self.generated_spalten_parameter = dict(getattr(tables, "generatedSpaltenParameter", {}))
        self.generated_spalten_parameter_tags = dict(
            getattr(tables, "generatedSpaltenParameter_Tags", {})
        )

    def snapshot(self) -> Dict[str, object]:
        return {
            "generated_spalten_parameter": self.generated_spalten_parameter,
            "generated_spalten_parameter_tags": self.generated_spalten_parameter_tags,
        }


@dataclass
class TableOutputSheaf:
    sections: Dict[str, Dict[str, object]] = field(default_factory=dict)

    def sync_from_tables(
        self,
        tables,
        output_mode: str,
        finally_display_lines: Optional[Iterable[int]] = None,
        rows_range: Optional[Iterable[int]] = None,
    ) -> None:
        resulting_table = []
        get_out = getattr(tables, "getOut", None)
        if get_out is not None:
            resulting_table = list(getattr(get_out, "resultingTable", []))
        self.sections[output_mode] = {
            "resulting_table": resulting_table,
            "finally_display_lines": list(finally_display_lines) if finally_display_lines is not None else None,
            "rows_range": list(rows_range) if rows_range is not None else None,
        }

    def snapshot(self) -> Dict[str, object]:
        return dict(self.sections)


class HtmlReferenceSheaf:
    def __init__(self, reference_map: Optional[Dict[int, Dict[str, object]]] = None) -> None:
        self.reference_map = reference_map or {}

    @classmethod
    def from_jsonl(cls, path: Optional[Path]) -> "HtmlReferenceSheaf":
        reference_map: Dict[int, Dict[str, object]] = {}
        if path is None or not path.exists():
            return cls(reference_map)
        with path.open(encoding="utf-8") as handle:
            for line in handle:
                line = line.strip()
                if not line:
                    continue
                try:
                    payload = json.loads(line)
                except json.JSONDecodeError:
                    continue
                try:
                    column_number = int(payload["column_number"])
                except Exception:
                    continue
                if payload.get("row_number") != 0:
                    continue
                reference_map[column_number] = payload
        return cls(reference_map)

    def html_meta_for_column(self, column_number: int) -> Dict[str, object]:
        return dict(self.reference_map.get(int(column_number), {}))

    def snapshot(self) -> Dict[str, object]:
        return {str(key): value for key, value in sorted(self.reference_map.items(), key=lambda item: item[0])}


@dataclass
class SheafBundle:
    parameter_semantics: ParameterSemanticsSheaf
    generated_columns: GeneratedColumnsSheaf
    table_output: TableOutputSheaf
    html_reference: HtmlReferenceSheaf

    @classmethod
    def from_repo(cls, repo_root: Path, schema: RetaContextSchema) -> "SheafBundle":
        return cls(
            parameter_semantics=ParameterSemanticsSheaf.from_schema(schema),
            generated_columns=GeneratedColumnsSheaf(),
            table_output=TableOutputSheaf(),
            html_reference=HtmlReferenceSheaf.from_jsonl(repo_root / "htmlclassesPy.jsonl"),
        )

    def snapshot(self) -> Dict[str, object]:
        return {
            "parameter_semantics": self.parameter_semantics.snapshot(),
            "generated_columns": self.generated_columns.snapshot(),
            "table_output": self.table_output.snapshot(),
            "html_reference_size": len(self.html_reference.reference_map),
        }
