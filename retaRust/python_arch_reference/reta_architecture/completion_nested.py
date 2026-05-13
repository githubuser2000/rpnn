from __future__ import annotations

"""Stage-41 activated nested-completion morphisms for the Reta prompt.

Historically ``libs/nestedAlx.py`` owned the hierarchical prompt completer:
state transitions between start commands, main parameters, value parameters and
comma/equals separated value sections.  Stage 41 moves that concrete
``NestedCompleter`` implementation into the architecture package while the
legacy module remains a thin import-compatible facade.

Mathematical reading:

* the current prompt document is a local input section;
* the cursor prefix selects a topological open set in the prompt command space;
* ``matchTextAlx`` and the ``para*``/``gleichKomma*`` transitions are morphisms
  between completion situations;
* completion options are local sections over those situations;
* the old ``nestedAlx.NestedCompleter`` path and the new architecture path are
  related by a natural transformation: both must expose the same options and
  yielded completions for the same prompt context.
"""

import difflib
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Any, Dict, Iterable, Mapping, Optional, Set, Union

try:
    from prompt_toolkit.completion import (
        CompleteEvent,
        Completer,
        Completion,
        FuzzyWordCompleter,
    )
    from prompt_toolkit.document import Document
except (ModuleNotFoundError, ImportError):
    class CompleteEvent:  # minimal fallback for architecture probes under -S
        pass

    class Completion:
        def __init__(self, text, start_position=0, display=None, display_meta=""):
            self.text = text
            self.start_position = start_position
            self.display = display if display is not None else text
            self.display_meta = display_meta

        def __repr__(self):
            return f"Completion({self.text!r}, start_position={self.start_position!r})"

    class Completer:
        def get_completions(self, document, complete_event):
            return ()

    class Document:
        def __init__(self, text: str = "", cursor_position: Optional[int] = None):
            self._text = text
            self.cursor_position = len(text) if cursor_position is None else cursor_position

        @property
        def text(self):
            return self._text

        @property
        def text_before_cursor(self):
            return self._text[: self.cursor_position]

    class FuzzyWordCompleter(Completer):
        def __init__(self, words):
            self.words = list(words)

        def get_completions(self, document, complete_event):
            prefix = document.text_before_cursor.split()[-1] if document.text_before_cursor.split() else document.text_before_cursor
            for word in self.words:
                if str(word).startswith(prefix) or prefix in str(word):
                    yield Completion(str(word), -len(prefix))

from .completion_word import ArchitectureWordCompleter as WordCompleter
from .input_semantics import RowRangeSyntax
from .row_ranges import RowRangeMorphismBundle, bootstrap_row_range_morphisms
from .split_i18n import build_split_i18n_proxy

hundert = list((str(n) for n in range(100)))
hundert2 = list((str(n) for n in range(10, 100)))
NestedDict = Mapping[str, Union[Any, Set[str], None, Completer]]

_DEFAULT_I18N = None
_DEFAULT_ROW_RANGE_MORPHISMS: Optional[RowRangeMorphismBundle] = None
_DEFAULT_COMPLETION_RUNTIME = None


def _default_i18n():
    global _DEFAULT_I18N
    if _DEFAULT_I18N is None:
        _DEFAULT_I18N = build_split_i18n_proxy()
    return _DEFAULT_I18N


def _default_row_range_morphisms(i18n=None) -> RowRangeMorphismBundle:
    global _DEFAULT_ROW_RANGE_MORPHISMS
    if _DEFAULT_ROW_RANGE_MORPHISMS is None:
        i18n = i18n or _default_i18n()
        _DEFAULT_ROW_RANGE_MORPHISMS = bootstrap_row_range_morphisms(RowRangeSyntax.from_i18n(i18n))
    return _DEFAULT_ROW_RANGE_MORPHISMS



def _default_prompt_language_refs():
    from .prompt_language import PromptModus, stextFromKleinKleinKleinBefehl

    return PromptModus.normal, stextFromKleinKleinKleinBefehl

def _default_completion_runtime():
    """Build the historical completion runtime lazily.

    The normal prompt path passes ``completion_runtime`` explicitly from
    ``LibRetaPrompt``.  This fallback preserves old direct instantiation of
    ``nestedAlx.NestedCompleter`` without making architecture module import-time
    depend on ``LibRetaPrompt``.
    """
    global _DEFAULT_COMPLETION_RUNTIME
    if _DEFAULT_COMPLETION_RUNTIME is None:
        from .completion_runtime import bootstrap_completion_runtime
        from .facade import RetaArchitecture

        repo_root = Path(__file__).resolve().parent.parent
        architecture = RetaArchitecture.bootstrap(repo_root)
        _DEFAULT_COMPLETION_RUNTIME = bootstrap_completion_runtime(
            repo_root=repo_root,
            architecture=architecture,
            i18n=_default_i18n(),
        )
    return _DEFAULT_COMPLETION_RUNTIME


@dataclass(frozen=True)
class NestedCompletionRuntimeView:
    """Local completion sections consumed by the nested-completer morphisms."""

    program: object
    ausgabe_art: tuple[str, ...]
    ausgabe_paras: tuple[str, ...]
    befehle: tuple[str, ...]
    befehle2: frozenset[str]
    befehle2_list: tuple[str, ...]
    haupt_for_neben: tuple[str, ...]
    haupt_for_neben_set: frozenset[str]
    kombi_main_paras: tuple[str, ...]
    main_parameters: tuple[str, ...]
    spalten: tuple[str, ...]
    spalten_dict: Mapping[str, tuple[str, ...]]
    zeilen_paras: tuple[str, ...]
    zeilen_typen: tuple[str, ...]
    zeilen_typen_b: tuple[str, ...]
    zeilen_zeit: tuple[str, ...]
    kombi_value_options: Mapping[str, tuple[str, ...]]

    @classmethod
    def from_runtime(cls, completion_runtime) -> "NestedCompletionRuntimeView":
        return cls(
            program=completion_runtime.program,
            ausgabe_art=tuple(completion_runtime.ausgabe_art),
            ausgabe_paras=tuple(completion_runtime.ausgabe_paras),
            befehle=tuple(completion_runtime.befehle),
            befehle2=frozenset(completion_runtime.befehle2),
            befehle2_list=tuple(completion_runtime.befehle2_list),
            haupt_for_neben=tuple(completion_runtime.haupt_for_neben),
            haupt_for_neben_set=frozenset(completion_runtime.haupt_for_neben_set),
            kombi_main_paras=tuple(completion_runtime.kombi_main_paras),
            main_parameters=tuple(completion_runtime.main_parameters),
            spalten=tuple(completion_runtime.spalten),
            spalten_dict={key: tuple(value) for key, value in completion_runtime.spalten_dict.items()},
            zeilen_paras=tuple(completion_runtime.zeilen_paras),
            zeilen_typen=tuple(completion_runtime.zeilen_typen),
            zeilen_typen_b=tuple(completion_runtime.zeilen_typen_b),
            zeilen_zeit=tuple(completion_runtime.zeilen_zeit),
            kombi_value_options={key: tuple(value) for key, value in completion_runtime.kombi_value_options.items()},
        )

    def snapshot(self) -> dict:
        return {
            "ausgabe_art_len": len(self.ausgabe_art),
            "befehle_len": len(self.befehle),
            "befehle2_len": len(self.befehle2),
            "haupt_for_neben_len": len(self.haupt_for_neben),
            "spalten_len": len(self.spalten),
            "zeilen_paras_len": len(self.zeilen_paras),
            "kombi_option_keys": sorted(self.kombi_value_options.keys()),
        }


ifRetaAnfang = False


class ComplSitua(Enum):
    hauptPara = 0
    zeilenPara = 1
    value = 3
    neitherNor = 4
    retaAnfang = 5
    unbekannt = 6
    spaltenPara = 7
    komiPara = 8
    kombiMetaPara = 9
    ausgabePara = 10
    spaltenValPara = 11
    zeilenValPara = 12
    kombiValPara = 13
    ausgabeValPara = 14
    befehleNichtReta = 15


class ArchitectureNestedCompleter(Completer):
    """Architecture-owned implementation behind ``nestedAlx.NestedCompleter``."""

    def __init__(
        self,
        options: Dict[str, Optional[Completer]],
        optionsStandard: Dict[str, Optional[Completer]],
        situation: ComplSitua,
        lastString: str,
        optionsTypes: Dict[str, Optional[ComplSitua]],
        ignore_case: bool = True,
        completion_runtime=None,
        i18n=None,
        row_range_morphisms: Optional[RowRangeMorphismBundle] = None,
    ) -> None:
        self.i18n = i18n or _default_i18n()
        self.row_range_morphisms = row_range_morphisms or _default_row_range_morphisms(self.i18n)
        self.completion_runtime = completion_runtime or _default_completion_runtime()
        self.runtime_view = NestedCompletionRuntimeView.from_runtime(self.completion_runtime)
        self.options2 = optionsStandard
        self.options1 = options
        self.options = {**options, **optionsStandard}
        self.ignore_case = ignore_case
        self.ExOptions: dict = {}
        self.ifGleichheitszeichen = False
        self.optionsPark: Dict[str, Optional[Completer]] = {}
        self.situationsTyp: Optional[ComplSitua] = situation
        self.lastString: str = lastString
        self.optionsTypes: Dict[str, Optional[ComplSitua]] = optionsTypes
        self.spaltenParaWort: Optional[str] = "  "
        self.kombiParaWort: Optional[str] = "  "
        self.ausgabeParaWort: Optional[str] = "  "
        self.zeilenParaWort: Optional[str] = "  "
        self.nebenParaWort: Optional[str] = "  "
        self.lastCommands: set = set()

    def _child(self, options, options_standard, situation, last_string, options_types):
        return type(self)(
            options,
            options_standard,
            situation,
            last_string,
            options_types,
            ignore_case=self.ignore_case,
            completion_runtime=self.completion_runtime,
            i18n=self.i18n,
            row_range_morphisms=self.row_range_morphisms,
        )

    def optionsSync(self):
        self.options = {**self.options1, **self.options2}

    def __repr__(self) -> str:
        return "NestedCompleter(%r, ignore_case=%r)" % (self.options, self.ignore_case)

    completers: set = set()

    def __eq__(self, obj) -> bool:
        return self.options.keys() == obj.options.keys()

    def __hash__(self):
        return hash(str(self.options.keys()))

    def matchTextAlx(self, first_term: str, trennzeichen: str = " ") -> Optional[Completer]:
        result = None
        for i in range(len(first_term), -1, -1):
            result = self.options.get(first_term[:i])
            if result is not None:
                break
        if result is None:
            result = self._child({}, {}, self.situationsTyp, first_term, {})
            self.__setOptions(result, first_term, trennzeichen)
        return result

    def __setOptions(self, completer: "ArchitectureNestedCompleter", first_term: str, trennzeichen: str):
        global ifRetaAnfang
        i18n = self.i18n
        view = self.runtime_view
        gleich = trennzeichen == "=" and self.situationsTyp in (
            ComplSitua.spaltenPara,
            ComplSitua.zeilenPara,
            ComplSitua.komiPara,
            ComplSitua.ausgabePara,
        )
        komma = trennzeichen == "," and self.situationsTyp in (
            ComplSitua.spaltenValPara,
            ComplSitua.zeilenValPara,
            ComplSitua.kombiValPara,
            ComplSitua.ausgabeValPara,
        )
        self.lastCommands |= {first_term}
        if trennzeichen == " ":
            if self.situationsTyp == ComplSitua.retaAnfang and first_term == "reta":
                ifRetaAnfang = True
                completer.options = {key: None for key in view.haupt_for_neben}
                completer.optionsTypes = {key: ComplSitua.hauptPara for key in view.haupt_for_neben}
                completer.lastString = first_term
                completer.situationsTyp = ComplSitua.hauptPara
            elif self.situationsTyp in (ComplSitua.retaAnfang, ComplSitua.befehleNichtReta) and first_term not in view.haupt_for_neben:
                if (
                    (
                        len(self.lastCommands & view.befehle2) > 0
                        and any(self.row_range_morphisms.is_fraction_or_integer_range(txt) for txt in self.lastCommands)
                    )
                    or _default_prompt_language_refs()[1](_default_prompt_language_refs()[0], list(self.lastCommands), [])[0]
                    or not ifRetaAnfang
                ):
                    liste = list(view.befehle2_list) + list(view.haupt_for_neben)
                else:
                    liste = list(view.befehle2_list)
                    ifRetaAnfang = False
                completer.options = {key: None for key in liste}
                completer.optionsTypes = {key: ComplSitua.befehleNichtReta for key in liste}
                completer.lastString = first_term
                completer.situationsTyp = ComplSitua.befehleNichtReta
            else:
                if len({first_term, self.nebenParaWort} & set(view.haupt_for_neben_set)) > 0:
                    if "-" + i18n.hauptForNeben["zeilen"] == first_term:
                        var1, var2 = self.paraZeilen(completer)
                    elif "-" + i18n.hauptForNeben["spalten"] == first_term:
                        var1, var2 = self.paraSpalten(completer)
                    elif "-" + i18n.hauptForNeben["ausgabe"] == first_term:
                        var1, var2 = self.paraAusgabe(completer)
                    elif "-" + i18n.hauptForNeben["kombination"] == first_term:
                        var1, var2 = self.paraKombination(completer)
                    elif "-" + i18n.hauptForNeben["zeilen"] == self.nebenParaWort:
                        var1, var2 = self.paraZeilen(completer)
                    elif "-" + i18n.hauptForNeben["spalten"] == self.nebenParaWort:
                        var1, var2 = self.paraSpalten(completer)
                    elif "-" + i18n.hauptForNeben["ausgabe"] == self.nebenParaWort:
                        var1, var2 = self.paraAusgabe(completer)
                    elif "-" + i18n.hauptForNeben["kombination"] == self.nebenParaWort:
                        var1, var2 = self.paraKombination(completer)

                    if not ifRetaAnfang:
                        try:
                            var1 += list(view.befehle2_list)
                        except Exception:
                            var1 = []
                    try:
                        var1
                    except UnboundLocalError:
                        var1 = []
                    completer.options = {key: None for key in var1}
                    completer.optionsTypes = {key: var2 for key in var1}
                    completer.lastString = first_term
                    completer.nebenParaWort = first_term if first_term in view.haupt_for_neben else self.nebenParaWort
                    if first_term not in view.haupt_for_neben:
                        completer.options = {**completer.options, **{key: None for key in view.haupt_for_neben}}
                        completer.optionsTypes = {**completer.optionsTypes, **{key: ComplSitua.hauptPara for key in view.haupt_for_neben}}
        elif gleich or komma:
            if "-" + i18n.hauptForNeben["spalten"] == first_term:
                var2, var3, var4 = self.gleichKommaSpalten(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["zeilen"] == first_term:
                var2, var3, var4 = self.gleichKommaZeilen(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["kombination"] == first_term:
                var2, var3, var4 = self.gleichKommaKombi(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["ausgabe"] == first_term:
                var2, var3, var4 = self.gleichKommaAusg(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["spalten"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaSpalten(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["zeilen"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaZeilen(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["kombination"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaKombi(completer, first_term, gleich, komma)
            elif "-" + i18n.hauptForNeben["ausgabe"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaAusg(completer, first_term, gleich, komma)
            else:
                var3 = ""
                var4 = {}
            suchWort = first_term[2:] if gleich else var3[2:] if komma and var3 is not None else 0
            try:
                var1 = var4[suchWort]
            except KeyError:
                var1 = difflib.get_close_matches(suchWort, var4.keys())
            completer.options = {key: None for key in var1}
            completer.optionsTypes = {key: var2 for key in var1}
            completer.lastString = first_term
            completer.nebenParaWort = self.nebenParaWort

    def gleichKommaKombi(self, completer, first_term, gleich, komma):
        completer.kombiParaWort = first_term if gleich else self.kombiParaWort if komma else None
        var4 = {key: list(value) for key, value in self.runtime_view.kombi_value_options.items()}
        var2 = ComplSitua.kombiValPara
        var3 = self.kombiParaWort
        completer.situationsTyp = ComplSitua.kombiValPara
        return var2, var3, var4

    def gleichKommaZeilen(self, completer, first_term, gleich, komma):
        i18n = self.i18n
        view = self.runtime_view
        completer.zeilenParaWort = first_term if gleich else self.zeilenParaWort if komma else None
        var4 = {key: hundert for key in i18n.zeilenParas if "--" + key + "=" in view.zeilen_paras}
        var4.update({key: [""] for key in i18n.zeilenParas if "--" + key + "=" not in view.zeilen_paras})
        var4[i18n.zeilenParas["typ"]] = list(view.zeilen_typen) + ["-" + t for t in view.zeilen_typen]
        var4[i18n.zeilenParas["primzahlen"]] = list(view.zeilen_typen_b) + ["-" + t for t in view.zeilen_typen_b]
        var4[i18n.zeilenParas["zeit"]] = list(view.zeilen_zeit) + ["-" + t for t in view.zeilen_zeit]
        var4.update({"*": var4[i18n.zeilenParas["typ"]] + var4[i18n.zeilenParas["primzahlen"]] + var4[i18n.zeilenParas["zeit"]]})
        var2 = ComplSitua.zeilenPara
        var3 = self.zeilenParaWort
        completer.situationsTyp = ComplSitua.zeilenValPara
        return var2, var3, var4

    def gleichKommaAusg(self, completer, first_term, gleich, komma):
        i18n = self.i18n
        view = self.runtime_view
        completer.ausgabeParaWort = first_term if gleich else self.ausgabeParaWort if komma else None
        var4 = {
            "*": list(view.ausgabe_art),
            i18n.ausgabeParas["breite"]: hundert2,
            i18n.ausgabeParas["breiten"]: hundert2,
        }
        var4[i18n.ausgabeParas["art"]] = list(view.ausgabe_art)
        var2 = ComplSitua.ausgabeValPara
        var3 = self.ausgabeParaWort
        completer.situationsTyp = ComplSitua.ausgabeValPara
        return var2, var3, var4

    def gleichKommaSpalten(self, completer, first_term, gleich, komma):
        i18n = self.i18n
        view = self.runtime_view
        completer.spaltenParaWort = first_term if gleich else self.spaltenParaWort if komma else None
        var4 = {key: list(value) for key, value in view.spalten_dict.items()}
        var4.update({i18n.ausgabeParas["breite"]: hundert2, i18n.ausgabeParas["breiten"]: hundert2})
        var2 = ComplSitua.spaltenValPara
        var3 = self.spaltenParaWort
        completer.situationsTyp = ComplSitua.spaltenValPara
        return var2, var3, var4

    def paraKombination(self, completer):
        var1 = list(self.runtime_view.kombi_main_paras)
        var2 = ComplSitua.kombiValPara
        completer.situationsTyp = ComplSitua.komiPara
        return var1, var2

    def paraAusgabe(self, completer):
        var1 = list(self.runtime_view.ausgabe_paras)
        var2 = ComplSitua.ausgabeValPara
        completer.situationsTyp = ComplSitua.ausgabePara
        return var1, var2

    def paraSpalten(self, completer):
        var1 = list(self.runtime_view.spalten)
        var2 = ComplSitua.spaltenValPara
        completer.situationsTyp = ComplSitua.spaltenPara
        return var1, var2

    def paraZeilen(self, completer):
        var1 = list(self.runtime_view.zeilen_paras)
        var2 = ComplSitua.zeilenValPara
        completer.situationsTyp = ComplSitua.zeilenPara
        return var1, var2

    def get_completions(self, document: Document, complete_event: CompleteEvent) -> Iterable[Completion]:
        text = document.text_before_cursor.lstrip()
        stripped_len = len(document.text_before_cursor) - len(text)
        gleich: bool = "=" in text
        komma: bool = "," in text
        if " " in text:
            first_term = text.split()[0]
            completer = self.matchTextAlx(first_term)
            if completer is not None:
                remaining_text = text[len(first_term) :].lstrip()
                move_cursor = len(text) - len(remaining_text) + stripped_len
                new_document = Document(remaining_text, cursor_position=document.cursor_position - move_cursor)
                for c in completer.get_completions(new_document, complete_event):
                    yield c
        elif gleich or komma:
            text = str(text)
            first_term = text.split("=" if gleich else ",")[0]
            completer = self.matchTextAlx(first_term, "=" if gleich else ",")
            if completer is not None:
                remaining_text = text[len(first_term) + 1 :].lstrip()
                move_cursor = len(text) - len(remaining_text) + stripped_len
                new_document = Document(remaining_text, cursor_position=document.cursor_position - move_cursor)
                for c in completer.get_completions(new_document, complete_event):
                    yield c
        else:
            completer = FuzzyWordCompleter(list(self.options.keys()))
            document._text = document._text
            if self.ifGleichheitszeichen:
                completer.options = completer.optionsPark
            self.ifGleichheitszeichen = False
            for c in completer.get_completions(document, complete_event):
                yield c


NestedCompleter = ArchitectureNestedCompleter


@dataclass(frozen=True)
class NestedCompletionMorphismBundle:
    """Activated architecture owner for legacy nested prompt completion."""

    legacy_owner: str = "libs.nestedAlx"
    activated_stage: int = 41
    i18n: object = None
    row_range_morphisms: Optional[RowRangeMorphismBundle] = None

    def __post_init__(self):
        object.__setattr__(self, "i18n", self.i18n or _default_i18n())
        object.__setattr__(self, "row_range_morphisms", self.row_range_morphisms or _default_row_range_morphisms(self.i18n))

    def create_completer(
        self,
        options: Dict[str, Optional[Completer]],
        options_standard: Optional[Dict[str, Optional[Completer]]] = None,
        situation: ComplSitua = ComplSitua.retaAnfang,
        last_string: str = "",
        options_types: Optional[Dict[str, Optional[ComplSitua]]] = None,
        ignore_case: bool = True,
        completion_runtime=None,
    ) -> ArchitectureNestedCompleter:
        return ArchitectureNestedCompleter(
            options,
            options_standard or {},
            situation,
            last_string,
            options_types or {},
            ignore_case=ignore_case,
            completion_runtime=completion_runtime,
            i18n=self.i18n,
            row_range_morphisms=self.row_range_morphisms,
        )

    def runtime_view(self, completion_runtime=None) -> NestedCompletionRuntimeView:
        return NestedCompletionRuntimeView.from_runtime(completion_runtime or _default_completion_runtime())

    def sample_options(self, completion_runtime=None) -> list[str]:
        view = self.runtime_view(completion_runtime)
        completer = self.create_completer(
            {command: None for command in view.befehle[:5]},
            completion_runtime=completion_runtime,
        )
        return sorted(str(key) for key in completer.options.keys())[:5]

    def snapshot(self) -> dict:
        return {
            "class": "NestedCompletionMorphismBundle",
            "stage": self.activated_stage,
            "legacy_owner": self.legacy_owner,
            "capsule": "InputPromptCapsule",
            "category": "ActivatedNestedCompletionCategory",
            "functor": "NestedCompletionActivationFunctor",
            "natural_transformation": "NestedCompleterToArchitectureTransformation",
            "morphisms": [
                "create_completer",
                "matchTextAlx",
                "paraZeilen",
                "paraSpalten",
                "paraAusgabe",
                "paraKombination",
                "gleichKommaZeilen",
                "gleichKommaSpalten",
                "gleichKommaAusg",
                "gleichKommaKombi",
                "get_completions",
            ],
            "compatibility_names": ["NestedCompleter", "ComplSitua", "nestedAlx.NestedCompleter"],
            "situations": [item.name for item in ComplSitua],
        }


def bootstrap_nested_completion_morphisms(i18n=None, row_range_morphisms: Optional[RowRangeMorphismBundle] = None) -> NestedCompletionMorphismBundle:
    return NestedCompletionMorphismBundle(i18n=i18n, row_range_morphisms=row_range_morphisms)


__all__ = [
    "ArchitectureNestedCompleter",
    "NestedCompleter",
    "NestedCompletionMorphismBundle",
    "NestedCompletionRuntimeView",
    "NestedDict",
    "ComplSitua",
    "WordCompleter",
    "bootstrap_nested_completion_morphisms",
]
