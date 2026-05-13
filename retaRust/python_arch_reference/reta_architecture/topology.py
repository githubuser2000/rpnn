from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, Iterable, List, Mapping, Optional, Set

from .schema import RetaContextSchema


@dataclass(frozen=True)
class ContextSelection:
    """A basis-open style context selection.

    The selection is intentionally symbolic instead of materialising a gigantic
    product space. Each dimension either contains an explicit whitelist of
    admissible values or is left unrestricted (None).
    """

    language: Optional[frozenset] = None
    main_parameters: Optional[frozenset] = None
    sub_parameters: Optional[frozenset] = None
    row_parameters: Optional[frozenset] = None
    output_modes: Optional[frozenset] = None
    tag_names: Optional[frozenset] = None
    combination_parameters: Optional[frozenset] = None
    scopes: Optional[frozenset] = None

    def restrict(self, **kwargs) -> "ContextSelection":
        data = {
            "language": self.language,
            "main_parameters": self.main_parameters,
            "sub_parameters": self.sub_parameters,
            "row_parameters": self.row_parameters,
            "output_modes": self.output_modes,
            "tag_names": self.tag_names,
            "combination_parameters": self.combination_parameters,
            "scopes": self.scopes,
        }
        for key, value in kwargs.items():
            if value is None:
                data[key] = None
            elif isinstance(value, frozenset):
                data[key] = value
            else:
                data[key] = frozenset(value)
        return ContextSelection(**data)

    def refine(self, other: "ContextSelection") -> "ContextSelection":
        def _meet(left: Optional[frozenset], right: Optional[frozenset]) -> Optional[frozenset]:
            if left is None:
                return right
            if right is None:
                return left
            return left & right

        return ContextSelection(
            language=_meet(self.language, other.language),
            main_parameters=_meet(self.main_parameters, other.main_parameters),
            sub_parameters=_meet(self.sub_parameters, other.sub_parameters),
            row_parameters=_meet(self.row_parameters, other.row_parameters),
            output_modes=_meet(self.output_modes, other.output_modes),
            tag_names=_meet(self.tag_names, other.tag_names),
            combination_parameters=_meet(self.combination_parameters, other.combination_parameters),
            scopes=_meet(self.scopes, other.scopes),
        )

    def is_empty(self) -> bool:
        for value in (
            self.language,
            self.main_parameters,
            self.sub_parameters,
            self.row_parameters,
            self.output_modes,
            self.tag_names,
            self.combination_parameters,
            self.scopes,
        ):
            if value is not None and len(value) == 0:
                return True
        return False

    def as_dict(self) -> Dict[str, Optional[List[str]]]:
        out: Dict[str, Optional[List[str]]] = {}
        for key in (
            "language",
            "main_parameters",
            "sub_parameters",
            "row_parameters",
            "output_modes",
            "tag_names",
            "combination_parameters",
            "scopes",
        ):
            value = getattr(self, key)
            out[key] = sorted(str(v) for v in value) if value is not None else None
        return out


@dataclass
class ContextDimension:
    name: str
    values: Set[str] = field(default_factory=set)
    aliases: Dict[str, str] = field(default_factory=dict)

    def canonicalize(self, value: Optional[str]) -> Optional[str]:
        if value is None:
            return None
        if value in self.values:
            return value
        return self.aliases.get(value)

    def include(self, canonical: str, *aliases: str) -> None:
        self.values.add(canonical)
        self.aliases[canonical] = canonical
        for alias in aliases:
            if alias:
                self.aliases[alias] = canonical
                self.values.add(canonical)


class RetaContextTopology:
    """The topological skeleton of reta.

    The topology is deliberately a topology of *contexts* (language, main and
    sub parameters, row-selection semantics, tags, output modes, combination
    domains) instead of a topology of raw content rows.
    """

    def __init__(self, schema: Optional[RetaContextSchema] = None) -> None:
        self.schema = schema
        self.dimensions: Dict[str, ContextDimension] = {
            "language": ContextDimension("language"),
            "main_parameters": ContextDimension("main_parameters"),
            "sub_parameters": ContextDimension("sub_parameters"),
            "row_parameters": ContextDimension("row_parameters"),
            "output_modes": ContextDimension("output_modes"),
            "tag_names": ContextDimension("tag_names"),
            "combination_parameters": ContextDimension("combination_parameters"),
            "scopes": ContextDimension("scopes"),
        }
        self._basis_cache: Optional[Dict[str, ContextSelection]] = None

    @classmethod
    def from_schema(cls, schema: RetaContextSchema) -> "RetaContextTopology":
        topology = cls(schema=schema)

        for alias, canonical in schema.language_aliases.items():
            topology.dimensions["language"].include(str(canonical), str(alias))

        for group in schema.parameters_main:
            aliases = [str(v) for v in group if v is not None]
            if not aliases:
                continue
            topology.dimensions["main_parameters"].include(aliases[0], *aliases[1:])

        seen_subs = set()
        for entry in schema.para_n_data_matrix:
            if len(entry) < 2:
                continue
            aliases = [str(v) for v in entry[1] if v is not None]
            if not aliases:
                continue
            key = aliases[0]
            if key in seen_subs:
                for alias in aliases:
                    topology.dimensions["sub_parameters"].aliases.setdefault(alias, key)
                continue
            topology.dimensions["sub_parameters"].include(key, *aliases[1:])
            seen_subs.add(key)

        for mapping, dimension_name in (
            (schema.row_parameters, "row_parameters"),
            (schema.output_modes, "output_modes"),
            (schema.combination_parameters, "combination_parameters"),
            (schema.scopes, "scopes"),
        ):
            if isinstance(mapping, Mapping):
                for canonical, alias in mapping.items():
                    topology.dimensions[dimension_name].include(str(canonical), str(alias))

        for tag_name in schema.tag_names:
            topology.dimensions["tag_names"].include(str(tag_name), str(tag_name))

        return topology

    def canonicalize(self, dimension: str, value: Optional[str]) -> Optional[str]:
        return self.dimensions[dimension].canonicalize(value)

    def open_for(self, dimension: str, values: Iterable[str]) -> ContextSelection:
        values = list(values)
        canonical = {
            self.canonicalize(dimension, str(value)) or str(value)
            for value in values
            if value is not None
        }
        data = {name: None for name in self.dimensions}
        data[dimension] = frozenset(canonical)
        return ContextSelection(**data)

    def basis_open_sets(self) -> Dict[str, ContextSelection]:
        if self._basis_cache is None:
            self._basis_cache = {
                name: self.open_for(name, dimension.values)
                for name, dimension in self.dimensions.items()
            }
        return dict(self._basis_cache)

    def cover_for_main(self, main_name: str) -> List[ContextSelection]:
        canonical_main = self.canonicalize("main_parameters", main_name) or main_name
        return [
            ContextSelection(main_parameters=frozenset({canonical_main})),
            ContextSelection(scopes=frozenset({"spalten"})),
        ]

    def refine(self, *selections: ContextSelection) -> ContextSelection:
        result = ContextSelection()
        for selection in selections:
            result = result.refine(selection)
        return result

    def snapshot(self) -> Dict[str, object]:
        return {
            "dimensions": {
                name: {
                    "values": sorted(str(v) for v in dimension.values),
                    "aliases": dict(sorted((str(k), str(v)) for k, v in dimension.aliases.items())),
                }
                for name, dimension in self.dimensions.items()
            },
            "basis": {key: value.as_dict() for key, value in self.basis_open_sets().items()},
        }
