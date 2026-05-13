from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Mapping, MutableMapping, Optional, Sequence, Tuple

from .facade import RetaArchitecture
from .input_semantics import PromptVocabulary
from .semantics_builder import ParameterSemanticsBuildResult, ParameterSemanticsBuilder


def _prime_command_predicate(num: int) -> int:
    num = int(num)
    if num <= 1:
        return 0
    if num == 2:
        return 1
    if num % 2 == 0:
        return 3
    divisor = 3
    while divisor * divisor <= num:
        if num % divisor == 0:
            return 3
        divisor += 2
    return 1


def build_main_parameter_commands(i18n) -> Dict[str, Optional[int]]:
    return {
        i18n.mainParaCmds["zeilen"]: 0,
        i18n.mainParaCmds["spalten"]: 1,
        i18n.mainParaCmds[tuple(i18n.tableHandling.parameterName.keys())[0]]: 2,
        i18n.mainParaCmds["ausgabe"]: 3,
        i18n.mainParaCmds["debug"]: None,
        i18n.mainParaCmds["h"]: None,
        i18n.mainParaCmds["help"]: None,
    }


@dataclass
class PromptTablesView:
    hoechsteZeile: Dict[int, int] = field(default_factory=lambda: {1024: 1024, 114: 163})


@dataclass
class PromptProgramView:
    architecture: RetaArchitecture
    mainParaCmds: Dict[str, Optional[int]]
    paraNdataMatrix: List[tuple]
    paraNdataMatrixAugmented: List[tuple]
    paraDict: Dict[Tuple[str, str], object]
    dataDict: List[dict]
    kombiParaNdataMatrix: object
    kombiParaNdataMatrix2: object
    kombiReverseDict: Dict[str, int]
    kombiReverseDict2: Dict[str, int]
    AllSimpleCommandSpalten: set
    tables: PromptTablesView = field(default_factory=PromptTablesView)


@dataclass
class PromptRuntimeBundle:
    program: PromptProgramView
    vocabulary: PromptVocabulary
    semantics: ParameterSemanticsBuildResult
    validation: Dict[str, object] = field(default_factory=dict)

    def snapshot(self) -> Dict[str, object]:
        return {
            "program_view": {
                "class": type(self.program).__name__,
                "mainParaCmds": list(self.program.mainParaCmds.keys()),
                "paraDict_len": len(self.program.paraDict),
                "dataDict_sizes": [len(d) for d in self.program.dataDict],
                "kombiReverseDict_len": len(self.program.kombiReverseDict),
                "kombiReverseDict2_len": len(self.program.kombiReverseDict2),
                "AllSimpleCommandSpalten_len": len(self.program.AllSimpleCommandSpalten),
                "max_rows": dict(self.program.tables.hoechsteZeile),
            },
            "vocabulary": self.vocabulary.snapshot(),
            "validation": dict(self.validation),
        }


class PromptRuntimeBuilder:
    def __init__(self, architecture: RetaArchitecture, i18n) -> None:
        self.architecture = architecture
        self.i18n = i18n

    def build_semantics(self) -> ParameterSemanticsBuildResult:
        return ParameterSemanticsBuilder(
            self.architecture.schema,
            gebrochen_spalten_maximum_plus1=int(self.i18n.gebrochenSpaltenMaximumPlus1),
            invert_alles=False,
            initial_data_dict=[{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}],
            prim_number_predicate=_prime_command_predicate,
            alles_parameter_names=self.i18n.ParametersMain.alles,
        ).build()

    def build_program_view(self, semantics: ParameterSemanticsBuildResult) -> PromptProgramView:
        return PromptProgramView(
            architecture=self.architecture,
            mainParaCmds=build_main_parameter_commands(self.i18n),
            paraNdataMatrix=list(self.architecture.schema.para_n_data_matrix),
            paraNdataMatrixAugmented=list(semantics.para_n_data_matrix),
            paraDict=dict(semantics.para_dict),
            dataDict=[dict(entry) for entry in semantics.data_dict],
            kombiParaNdataMatrix=semantics.kombi_para_n_data_matrix,
            kombiParaNdataMatrix2=semantics.kombi_para_n_data_matrix2,
            kombiReverseDict=dict(semantics.kombi_reverse_dict),
            kombiReverseDict2=dict(semantics.kombi_reverse_dict2),
            AllSimpleCommandSpalten=set(semantics.all_simple_command_columns),
        )

    def validate_wahl15(self, program: PromptProgramView) -> Dict[str, object]:
        comparison_values = {str(column) for entry in program.paraNdataMatrix for column in entry[1]}
        missing = []
        splitter = self.architecture.inputs.row_ranges.split_comma_list
        for value in self.i18n.wahl15.values():
            for alias in splitter(str(value)):
                alias = alias.strip()
                if alias and alias not in comparison_values:
                    missing.append(alias)
        return {
            "wahl15_missing_values": sorted(set(missing)),
            "wahl15_valid": len(missing) == 0,
        }

    def build(self) -> PromptRuntimeBundle:
        semantics = self.build_semantics()
        program = self.build_program_view(semantics)
        vocabulary = self.architecture.inputs.build_prompt_vocabulary(program, self.i18n)
        validation = self.validate_wahl15(program)
        return PromptRuntimeBundle(
            program=program,
            vocabulary=vocabulary,
            semantics=semantics,
            validation=validation,
        )


_PROMPT_RUNTIME_CACHE: MutableMapping[Tuple[str, str], PromptRuntimeBundle] = {}


def bootstrap_prompt_runtime(
    repo_root: Optional[Path] = None,
    architecture: Optional[RetaArchitecture] = None,
    i18n=None,
    force_rebuild: bool = False,
) -> PromptRuntimeBundle:
    architecture = architecture or RetaArchitecture.bootstrap(repo_root)
    if i18n is None:
        import i18n.words_runtime as i18n  # noqa: WPS433

    cache_key = (str(architecture.repo_root.resolve()), str(getattr(i18n, "__name__", type(i18n).__name__)))
    if force_rebuild or cache_key not in _PROMPT_RUNTIME_CACHE:
        _PROMPT_RUNTIME_CACHE[cache_key] = PromptRuntimeBuilder(architecture, i18n).build()
    return _PROMPT_RUNTIME_CACHE[cache_key]
