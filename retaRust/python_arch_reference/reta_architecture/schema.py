from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List, Mapping, Tuple


@dataclass(frozen=True)
class AliasGroup:
    canonical: str
    aliases: Tuple[str, ...]


@dataclass
class RetaContextSchema:
    """Structured extraction of reta's context/schema layer.

    The legacy repo used to keep almost the whole schema in the single
    `i18n/words.py` monolith. After the split refactor we can now read the
    schema from dedicated context/matrix/runtime modules while still supporting
    the old one-module access path.
    """

    language_aliases: Dict[str, str] = field(default_factory=dict)
    translation_domains: Dict[str, str] = field(default_factory=dict)
    parameters_main: List[Tuple[str, ...]] = field(default_factory=list)
    row_parameters: Dict[str, str] = field(default_factory=dict)
    output_parameters: Dict[str, str] = field(default_factory=dict)
    output_modes: Dict[str, str] = field(default_factory=dict)
    combination_parameters: Dict[str, str] = field(default_factory=dict)
    scopes: Dict[str, str] = field(default_factory=dict)
    para_n_data_matrix: List[tuple] = field(default_factory=list)
    kombi_para_n_data_matrix: Any = None
    kombi_para_n_data_matrix2: Any = None
    tag_names: Tuple[str, ...] = ()
    schema_modules: Dict[str, str] = field(default_factory=dict)

    @staticmethod
    def _stringified_mapping(source: Mapping[Any, Any]) -> Dict[str, str]:
        return {str(alias): str(canonical) for alias, canonical in dict(source).items()}

    @staticmethod
    def _parameter_groups(parameters_main) -> List[Tuple[str, ...]]:
        if parameters_main is None:
            return []
        return [
            tuple(str(v) for v in group if v is not None)
            for group in parameters_main
        ]

    @staticmethod
    def _tag_names(tag_enum=None, table_tags=None) -> Tuple[str, ...]:
        tag_names = set()
        if tag_enum is not None:
            for tag in tag_enum:
                tag_names.add(str(tag.name))
        if table_tags is not None:
            for tag_group in table_tags.keys():
                for tag in tag_group:
                    tag_names.add(str(tag.name))
        return tuple(sorted(tag_names))

    @staticmethod
    def _module_name(module_like) -> str:
        return str(getattr(module_like, "__name__", type(module_like).__name__))

    @classmethod
    def from_words_parts(
        cls,
        context_module,
        matrix_module,
        runtime_module=None,
        tag_enum=None,
        table_tags=None,
    ) -> "RetaContextSchema":
        runtime_module = runtime_module or context_module

        return cls(
            language_aliases=cls._stringified_mapping(
                getattr(context_module, "sprachen", {})
            ),
            translation_domains=cls._stringified_mapping(
                getattr(context_module, "sprachen2", {})
            ),
            parameters_main=cls._parameter_groups(
                getattr(context_module, "ParametersMain", ())
            ),
            row_parameters=cls._stringified_mapping(
                getattr(context_module, "zeilenParas", {})
            ),
            output_parameters=cls._stringified_mapping(
                getattr(context_module, "ausgabeParas", {})
            ),
            output_modes=cls._stringified_mapping(
                getattr(context_module, "ausgabeArt", {})
            ),
            combination_parameters=cls._stringified_mapping(
                getattr(context_module, "kombiMainParas", {})
            ),
            scopes=cls._stringified_mapping(getattr(runtime_module, "hauptForNeben", {})),
            para_n_data_matrix=list(getattr(matrix_module, "paraNdataMatrix", ())),
            kombi_para_n_data_matrix=getattr(matrix_module, "kombiParaNdataMatrix", None),
            kombi_para_n_data_matrix2=getattr(matrix_module, "kombiParaNdataMatrix2", None),
            tag_names=cls._tag_names(tag_enum=tag_enum, table_tags=table_tags),
            schema_modules={
                "context": cls._module_name(context_module),
                "matrix": cls._module_name(matrix_module),
                "runtime": cls._module_name(runtime_module),
            },
        )

    @classmethod
    def from_words_module(
        cls,
        words_module,
        tag_enum=None,
        table_tags=None,
    ) -> "RetaContextSchema":
        return cls.from_words_parts(
            context_module=words_module,
            matrix_module=words_module,
            runtime_module=words_module,
            tag_enum=tag_enum,
            table_tags=table_tags,
        )

    def main_alias_groups(self) -> List[AliasGroup]:
        groups: List[AliasGroup] = []
        for group in self.parameters_main:
            aliases = tuple(str(v) for v in group if v is not None)
            if not aliases:
                continue
            groups.append(AliasGroup(canonical=aliases[0], aliases=aliases))
        return groups

    def main_alias_map(self) -> Dict[str, str]:
        alias_map: Dict[str, str] = {}
        for group in self.main_alias_groups():
            alias_map[group.canonical] = group.canonical
            for alias in group.aliases:
                alias_map[str(alias)] = group.canonical
        return alias_map

    def sub_parameter_alias_groups(self) -> Dict[str, List[AliasGroup]]:
        main_alias_map = self.main_alias_map()
        groups: Dict[str, Dict[str, List[str]]] = {}
        for entry in self.para_n_data_matrix:
            if len(entry) < 2:
                continue
            main_aliases = tuple(str(v) for v in entry[0] if v is not None)
            parameter_aliases = tuple(str(v) for v in entry[1] if v is not None)
            if not main_aliases or not parameter_aliases:
                continue
            main_canonical = main_alias_map.get(main_aliases[0], main_aliases[0])
            parameter_canonical = parameter_aliases[0]
            groups.setdefault(main_canonical, {})
            groups[main_canonical].setdefault(parameter_canonical, [])
            for alias in parameter_aliases:
                if alias not in groups[main_canonical][parameter_canonical]:
                    groups[main_canonical][parameter_canonical].append(alias)
        return {
            main: [
                AliasGroup(canonical=canonical, aliases=tuple(sorted(aliases)))
                for canonical, aliases in sorted(parameters.items(), key=lambda item: item[0])
            ]
            for main, parameters in sorted(groups.items(), key=lambda item: item[0])
        }

    def snapshot(self) -> Dict[str, Any]:
        return {
            "languages": dict(sorted(self.language_aliases.items())),
            "translation_domains": dict(sorted(self.translation_domains.items())),
            "main_alias_groups": [
                {"canonical": group.canonical, "aliases": list(group.aliases)}
                for group in self.main_alias_groups()
            ],
            "row_parameters": dict(sorted(self.row_parameters.items())),
            "output_parameters": dict(sorted(self.output_parameters.items())),
            "output_modes": dict(sorted(self.output_modes.items())),
            "combination_parameters": dict(sorted(self.combination_parameters.items())),
            "scopes": dict(sorted(self.scopes.items())),
            "tag_names": list(self.tag_names),
            "para_n_data_matrix_size": len(self.para_n_data_matrix),
            "kombi_para_n_data_matrix_size": len(self.kombi_para_n_data_matrix or {}),
            "kombi_para_n_data_matrix2_size": len(self.kombi_para_n_data_matrix2 or {}),
            "schema_modules": dict(sorted(self.schema_modules.items())),
        }
