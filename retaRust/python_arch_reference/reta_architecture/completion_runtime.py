from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, MutableMapping, Optional, Sequence, Set, Tuple

from .facade import RetaArchitecture
from .prompt_runtime import PromptRuntimeBundle


def sort_completion_key(key: str, i18n) -> Tuple[int, str]:
    key = str(key)
    if not key.startswith("1") and not any((key[:3] == a[:3] for a in i18n.EIGS_N_R)):
        if key in (
            i18n.befehle2["absicht"],
            i18n.befehle2["hilfe"],
            i18n.befehle2["kurzbefehle"],
        ) and len(key) != 1:
            return (0, key)
        elif key in (
            i18n.befehle2["universum"],
            i18n.befehle2["thomas"],
            i18n.befehle2["befehle"],
            i18n.befehle2["groesse"],
        ) and len(key) != 1:
            return (1, key)
        elif key in (
            i18n.befehle2["reta"],
            i18n.befehle2["bewusstsein"],
            i18n.befehle2["geist"],
            i18n.befehle2["emotion"],
            i18n.befehle2["impulse"],
        ) and len(key) != 1:
            return (2, key)
        elif key in (
            i18n.befehle2["loggen"],
            i18n.befehle2["nichtloggen"],
            i18n.befehle2["exit"],
            i18n.befehle2["quit"],
        ) and len(key) != 1:
            return (3, key)
        elif len({key} & set(i18n.befehle2.values())) > 0 and not (
            key[:3] not in ("15_", "16_", i18n.EIGS_N_R[1][:3]),
            i18n.EIGS_N_R[0][:3],
        ) and len(key) != 1:
            return (4, key)
        elif key in i18n.befehle and len(key) != 1:
            return (5, key)
        elif not key.startswith("1") and len(key) != 1:
            return (6, key)
        elif len(key.strip()) == 1:
            return (7, key)
        elif key[:3] == "15_" or key == "15":
            return (8, key)
        elif key[:3] == "16_" or key == "16":
            return (9, key)
        elif key.startswith("1") and len(key) != 1:
            if key.startswith("15"):
                return (10, key)
            elif key.startswith("16"):
                return (11, key)
            else:
                return (11, key)
        elif any((key[:3] == a[:3] for a in i18n.EIGS_N_R)):
            return (19, key)
        else:
            return (13, key)
    elif any((key[:3] == a[:3] for a in i18n.EIGS_N_R)):
        return (19, key)
    else:
        return (14, key)


@dataclass
class CompletionRuntimeBundle:
    prompt_runtime: PromptRuntimeBundle
    befehle: List[str]
    befehle2: Set[str]
    befehle2_list: List[str]
    haupt_for_neben: List[str]
    haupt_for_neben_set: Set[str]
    ausgabe_art: List[str]
    ausgabe_paras: List[str]
    kombi_main_paras: List[str]
    main_parameters: List[str]
    spalten: List[str]
    spalten_dict: Dict[str, List[str]]
    zeilen_paras: List[str]
    zeilen_typen: List[str]
    zeilen_zeit: List[str]
    zeilen_typen_b: List[str]
    kombi_value_options: Dict[str, List[str]] = field(default_factory=dict)

    @property
    def program(self):
        return self.prompt_runtime.program

    @property
    def vocabulary(self):
        return self.prompt_runtime.vocabulary

    def start_commands(self, include_numeric_shortcuts: bool = False) -> List[str]:
        commands = list(self.befehle)
        if include_numeric_shortcuts:
            for item in ("15_", "16_"):
                if item not in commands:
                    commands.append(item)
        return commands

    def snapshot(self) -> Dict[str, object]:
        return {
            "befehle_len": len(self.befehle),
            "befehle2_len": len(self.befehle2),
            "haupt_for_neben_len": len(self.haupt_for_neben),
            "spalten_dict_keys": len(self.spalten_dict),
            "kombi_option_keys": sorted(self.kombi_value_options.keys()),
            "start_commands_with_numeric_shortcuts": self.start_commands(
                include_numeric_shortcuts=True
            )[:10],
            "prompt_runtime": self.prompt_runtime.snapshot(),
        }


class CompletionRuntimeBuilder:
    def __init__(self, architecture: RetaArchitecture, i18n) -> None:
        self.architecture = architecture
        self.i18n = i18n

    def build(self) -> CompletionRuntimeBundle:
        prompt_runtime = self.architecture.bootstrap_prompt_runtime(i18n=self.i18n)
        vocabulary = prompt_runtime.vocabulary
        program = prompt_runtime.program
        befehle2_list = list(
            sorted(set(vocabulary.befehle2), key=lambda item: sort_completion_key(item, self.i18n))
        )
        kombi_galaxie = [
            item
            for sublist in program.kombiParaNdataMatrix.values()
            for item in sublist
        ]
        kombi_universum = [
            item
            for sublist in program.kombiParaNdataMatrix2.values()
            for item in sublist
        ]
        return CompletionRuntimeBundle(
            prompt_runtime=prompt_runtime,
            befehle=list(sorted(vocabulary.befehle, key=lambda item: sort_completion_key(item, self.i18n))),
            befehle2=set(befehle2_list),
            befehle2_list=befehle2_list,
            haupt_for_neben=list(vocabulary.haupt_for_neben),
            haupt_for_neben_set=set(vocabulary.haupt_for_neben_set),
            ausgabe_art=list(vocabulary.ausgabe_art),
            ausgabe_paras=list(vocabulary.ausgabe_paras),
            kombi_main_paras=list(vocabulary.kombi_main_paras),
            main_parameters=list(vocabulary.main_parameters),
            spalten=list(vocabulary.spalten),
            spalten_dict={key: list(value) for key, value in vocabulary.spalten_dict.items()},
            zeilen_paras=list(vocabulary.zeilen_paras),
            zeilen_typen=list(vocabulary.zeilen_typen),
            zeilen_zeit=list(vocabulary.zeilen_zeit),
            zeilen_typen_b=list(vocabulary.zeilen_typen_b),
            kombi_value_options={
                self.i18n.kombiMainParas["galaxie"]: kombi_galaxie,
                self.i18n.kombiMainParas["universum"]: kombi_universum,
                "*": kombi_galaxie + kombi_universum,
            },
        )


_COMPLETION_RUNTIME_CACHE: MutableMapping[Tuple[str, str], CompletionRuntimeBundle] = {}


def bootstrap_completion_runtime(
    repo_root: Optional[Path] = None,
    architecture: Optional[RetaArchitecture] = None,
    i18n=None,
    force_rebuild: bool = False,
) -> CompletionRuntimeBundle:
    architecture = architecture or RetaArchitecture.bootstrap(repo_root)
    if i18n is None:
        import i18n.words_runtime as i18n  # noqa: WPS433

    cache_key = (
        str(architecture.repo_root.resolve()),
        str(getattr(i18n, "__name__", type(i18n).__name__)),
    )
    if force_rebuild or cache_key not in _COMPLETION_RUNTIME_CACHE:
        _COMPLETION_RUNTIME_CACHE[cache_key] = CompletionRuntimeBuilder(
            architecture, i18n
        ).build()
    return _COMPLETION_RUNTIME_CACHE[cache_key]
