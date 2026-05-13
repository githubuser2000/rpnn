from __future__ import annotations

import os
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable, DefaultDict, Dict, List, MutableMapping, Optional, Sequence, Tuple

try:
    from prompt_toolkit import PromptSession
    from prompt_toolkit.history import FileHistory
    from prompt_toolkit.styles import Style
except (ModuleNotFoundError, ImportError):
    class PromptSession:  # minimal non-interactive fallback for architecture probes/tests
        def __init__(self, *args, **kwargs):
            pass

        def prompt(self, *args, **kwargs):
            return ""

    class FileHistory:
        def __init__(self, *args, **kwargs):
            pass

    class Style:
        @classmethod
        def from_dict(cls, *args, **kwargs):
            return cls()

from .completion_runtime import CompletionRuntimeBundle, bootstrap_completion_runtime
from .facade import RetaArchitecture
from .prompt_language import (
    PromptLanguageBundle,
    PromptModus,
    bootstrap_prompt_language,
    custom_split,
    stextFromKleinKleinKleinBefehl,
)
from .prompt_runtime import PromptRuntimeBundle, bootstrap_prompt_runtime
from .topology import ContextSelection


_DEFAULT_TEXT_ARCHITECTURES: MutableMapping[str, RetaArchitecture] = {}


@dataclass
class PromptTextState:
    """Mutable prompt text state formerly owned by ``retaPrompt.py``."""

    architecture: RetaArchitecture
    i18n: object
    _text: str = ""
    _platzhalter: str = ""
    _stext: List[str] = field(default_factory=list)
    _stextS: List[str] = field(default_factory=list)
    _stextE: List[str] = field(default_factory=list)
    _e: List[str] = field(default_factory=list)
    _stextEmenge: set = field(default_factory=set)
    _stextSet: set = field(default_factory=set)
    _befehlDavor: str = ""

    def __init__(self, txt: str = "", architecture: Optional[RetaArchitecture] = None, i18n=None) -> None:
        if architecture is None:
            repo_root = Path(__file__).resolve().parent.parent
            cache_key = str(repo_root)
            architecture = _DEFAULT_TEXT_ARCHITECTURES.get(cache_key)
            if architecture is None:
                architecture = RetaArchitecture.bootstrap(repo_root)
                _DEFAULT_TEXT_ARCHITECTURES[cache_key] = architecture
        if i18n is None:
            import i18n.words_runtime as i18n  # noqa: WPS433
        self.architecture = architecture
        self.i18n = i18n
        self._text = ""
        self._platzhalter = ""
        self._stext = []
        self._stextS = []
        self._stextE = []
        self._e = []
        self._stextEmenge = set()
        self._stextSet = set()
        self._befehlDavor = ""
        self.text = txt.strip()

    def hasWithoutABC(self, hasSet: set) -> bool:
        return (
            len(hasSet & self._stextSet) > 0
            and len({self.i18n.befehle2["abc"], self.i18n.befehle2["abcd"]} & self._stextSet) == 0
        )

    def has(self, hasSet: set) -> bool:
        return len(hasSet & self._stextSet) > 0

    @property
    def e(self):
        return self._e

    @property
    def menge(self):
        return self._stextSet

    @property
    def listeE(self):
        return self._stextE

    @property
    def listeS(self):
        return self._stextS

    @property
    def liste(self):
        return self._stext

    @property
    def mengeE(self):
        return self._stextEmenge

    @property
    def platzhalter(self):
        return self._platzhalter

    @property
    def text(self):
        return self._text

    @platzhalter.setter
    def platzhalter(self, value):
        self._platzhalter = value.strip()

    @text.setter
    def text(self, value):
        assert type(value) is str
        value = str(value).strip()
        self._text = value
        if value[:4] != "reta":
            self._stext = self.architecture.morphisms.prompt.split_prompt_text(
                self._text,
                custom_split,
            )
            self._stextS = self.architecture.morphisms.prompt.split_command_words(
                value,
                custom_split,
            )
        else:
            self._stext = [s.strip() for s in self._text.split() if len(s.strip()) > 0]
            self._stextS = value.split()
        self._stextSet = set(self._stext)
        self._stextEmenge = self._stextSet | set(self._e)
        self._stextE = self._stext + self._e
        self.architecture.update_prompt_state(
            self._text,
            self._stextS,
            context=ContextSelection(scopes=frozenset({"prompt"})),
        )

    @liste.setter
    def liste(self, value):
        assert type(value) is list
        self._stext = [s.strip() for s in value if len(s.strip()) > 0]
        self._stextSet = set(self._stext)
        self._stextEmenge = self._stextSet | set(self._e)
        self._stextE = self._stext + self._e
        self._stextS = [
            token
            for group in [
                self.architecture.morphisms.prompt.split_prompt_text(v, custom_split)
                for v in value
            ]
            for token in group
        ]
        self.architecture.update_prompt_state(
            " ".join(self._stext),
            self._stextS,
            context=ContextSelection(scopes=frozenset({"prompt"})),
        )

    @e.setter
    def e(self, value):
        assert type(value) is list
        self._e = value
        self._stextEmenge = self._stextSet | set(self._e)
        self._stextE = self._stext + self._e

    @property
    def befehlDavor(self):
        return self._befehlDavor

    @befehlDavor.setter
    def befehlDavor(self, value):
        self._befehlDavor = value


@dataclass
class PromptLoopSetup:
    befehle_beenden: set
    logging_switch: bool
    prompt_mode: PromptModus
    prompt_mode2: PromptModus
    prompt_prefixes: DefaultDict[PromptModus, str]
    start_completer: object
    only_one_command: List[str]
    force_e_command: bool
    text_dazu0: List[str] = field(default_factory=list)

    def snapshot(self) -> Dict[str, object]:
        return {
            "logging_switch": bool(self.logging_switch),
            "prompt_mode": self.prompt_mode.name,
            "prompt_mode2": self.prompt_mode2.name,
            "only_one_command": list(self.only_one_command),
            "force_e_command": bool(self.force_e_command),
            "befehle_beenden_len": len(self.befehle_beenden),
            "prompt_prefixes": {key.name: value for key, value in self.prompt_prefixes.items()},
        }


@dataclass
class PromptStoreResult:
    chains: list
    text_state: PromptTextState
    prompt_mode2: PromptModus
    text_dazu0: List[str]


@dataclass
class PromptSessionBundle:
    architecture: RetaArchitecture
    i18n: object
    prompt_runtime: PromptRuntimeBundle
    completion_runtime: CompletionRuntimeBundle
    prompt_language: PromptLanguageBundle
    history_file: str = field(default_factory=lambda: os.path.expanduser("~") + os.sep + ".ReTaPromptHistory")

    def snapshot(self) -> Dict[str, object]:
        return {
            "class": type(self).__name__,
            "history_file": self.history_file,
            "prompt_runtime_class": type(self.prompt_runtime).__name__,
            "completion_runtime_class": type(self.completion_runtime).__name__,
            "prompt_language_class": type(self.prompt_language).__name__,
            "default_text_architecture": str(self.architecture.repo_root),
        }

    def new_text_state(self, text: str = "") -> PromptTextState:
        return PromptTextState(text, architecture=self.architecture, i18n=self.i18n)

    def new_session(self, history: bool = False):
        bundle = self

        class ToggleHistory(FileHistory):
            def __init__(self, file_path, *args, **kwargs):
                super().__init__(file_path, *args, **kwargs)
                self.inner_history = FileHistory(file_path)
                self.logging_enabled = True

            def append_string(self, string):
                if (
                    self.logging_enabled
                    and bundle.i18n.befehle2["nichtloggen"] not in custom_split(string)
                    and bundle.i18n.befehle2["loggen"] not in custom_split(string)
                ):
                    super().append_string(string)

            def get_strings(self):
                return self.inner_history.get_strings()

            def enable_logging(self):
                self.logging_enabled = True

            def disable_logging(self):
                self.logging_enabled = False

            def add_to_history(self, string):
                self.append_string(string)

        if history:
            return PromptSession(history=ToggleHistory(self.history_file))
        return PromptSession()

    def build_loop_setup(
        self,
        argv: Sequence[str],
        nested_completer_cls,
        compl_situa_enum,
        reta_prompt_help: Callable[[], None],
        prompt_program=None,
        debug_output: Optional[Callable[[], None]] = None,
    ) -> PromptLoopSetup:
        i18nRP = self.i18n.retaPrompt
        argv = list(argv)
        if "-" + i18nRP.retaPromptParameter["vi"] not in argv:
            reta_prompt_help()
        logging_switch = "-" + i18nRP.retaPromptParameter["log"] in argv
        if ("-" + i18nRP.retaPromptParameter["h"] in argv) or (
            "-" + i18nRP.retaPromptParameter["help"] in argv
        ):
            print(i18nRP.helptext)
            raise SystemExit(0)
        target_program = prompt_program or self.prompt_runtime.program
        if "-" + i18nRP.retaPromptParameter["debug"] in argv:
            target_program.propInfoLog = True
            if "-" + i18nRP.retaPromptParameter["e"] not in argv and debug_output is not None:
                debug_output()

        if "-" + i18nRP.retaPromptParameter["befehl"] in argv:
            start = argv.index("-" + i18nRP.retaPromptParameter["befehl"]) + 1
            only_one_command = argv[start:]
        else:
            only_one_command = []
        force_e_command = "-" + i18nRP.retaPromptParameter["e"] in argv
        start_completer = nested_completer_cls(
            {command: None for command in self.completion_runtime.start_commands(include_numeric_shortcuts=True)},
            {},
            compl_situa_enum.retaAnfang,
            "",
            {
                **{"reta": compl_situa_enum.retaAnfang},
                **{
                    command: compl_situa_enum.befehleNichtReta
                    for command in self.completion_runtime.befehle2
                },
            },
            completion_runtime=self.completion_runtime,
        )
        prompt_prefixes = defaultdict(lambda: ">")
        prompt_prefixes[PromptModus.speichern] = i18nRP.wspeichernWort
        prompt_prefixes[PromptModus.loeschenSelect] = i18nRP.wloeschenWort
        return PromptLoopSetup(
            befehle_beenden=set(self.i18n.retaPrompt.befehleBeenden),
            logging_switch=logging_switch,
            prompt_mode=PromptModus.normal,
            prompt_mode2=PromptModus.normal,
            prompt_prefixes=prompt_prefixes,
            start_completer=start_completer,
            only_one_command=only_one_command,
            force_e_command=force_e_command,
            text_dazu0=[],
        )

    def store_prompt(
        self,
        chains,
        placeholder: str,
        text: str,
        prompt_mode2: PromptModus,
        befehle,
        prepare_large_output,
    ) -> PromptStoreResult:
        i18nRP = self.i18n.retaPrompt
        has_placeholder = len(placeholder) > 0
        has_chains = len(chains) > 0
        txt_state = self.new_text_state(text)
        txt_state.platzhalter = placeholder
        if has_placeholder or has_chains:
            if has_placeholder:
                where_reta_command = []
                txt_placeholder = self.new_text_state(txt_state.platzhalter)
                if txt_state.liste[:1] == ["reta"]:
                    where_reta_command += ["bereits-dabei"]
                    txt_state.liste.pop(0)
                if txt_placeholder.liste[:1] == ["reta"]:
                    where_reta_command += ["bei-dazu"]
                    txt_placeholder.liste.pop(0)
                if len(where_reta_command) > 0:
                    txt_state.platzhalter = (
                        "reta " + " ".join(txt_placeholder.liste) + " " + " ".join(txt_state.liste)
                    )
                else:
                    combined_tokens = []
                    long_short_commands = []
                    for prompt_command in custom_split(txt_state.platzhalter) + txt_state.liste:
                        if prompt_command in befehle and len(prompt_command) > 1:
                            long_short_commands += [prompt_command.strip()]
                        else:
                            combined_tokens += [prompt_command.strip()]
                    txt2 = self.new_text_state()
                    txt2.liste = combined_tokens
                    _, txt2.liste = stextFromKleinKleinKleinBefehl(
                        PromptModus.AusgabeSelektiv,
                        txt2.liste,
                        [],
                    )
                    replacements = i18nRP.replacements
                    if len(combined_tokens) > 0 and txt2.liste[0] not in [
                        "reta",
                        self.i18n.befehle2["shell"],
                        self.i18n.befehle2["python"],
                        self.i18n.befehle2["abstand"],
                    ]:
                        normalized_tokens = []
                        for token in txt2.liste:
                            normalized_tokens += [replacements.get(token, token)]
                        txt2.liste = normalized_tokens
                    txt_state.platzhalter = " ".join(txt2.liste + long_short_commands)
        else:
            txt_state.platzhalter = "" if txt_state.text is None else str(txt_state.text)
        txt_state.text = ""
        if txt_state.platzhalter != "" or not (has_placeholder or has_chains):
            prompt_mode2 = PromptModus.AusgabeSelektiv
        else:
            prompt_mode2 = PromptModus.normal
        (
            _,
            _,
            _,
            _,
            _,
            stext,
            _,
            _,
        ) = prepare_large_output(
            txt_state.platzhalter,
            prompt_mode2,
            prompt_mode2,
            PromptModus.normal,
            txt_state.platzhalter,
            [],
        )
        return PromptStoreResult(
            chains=chains,
            text_state=txt_state,
            prompt_mode2=prompt_mode2,
            text_dazu0=stext,
        )

    def delete_before_storage_commands(
        self,
        placeholder: str,
        prompt_mode,
        text: str,
        is_zeilenangabe,
        bereich_to_numbers,
    ):
        txt_to_delete = self.new_text_state(text)
        txt_existing = self.new_text_state(placeholder)
        deletable_by_index = {index + 1: value for index, value in enumerate(txt_existing.liste)}
        deletable_by_value = {value: index + 1 for index, value in enumerate(txt_existing.liste)}
        use_token_deletion = False
        if is_zeilenangabe(txt_to_delete.text):
            if txt_to_delete.text not in deletable_by_value.keys() or not txt_to_delete.text.isdecimal():
                for token in bereich_to_numbers(txt_to_delete.text, False, 0):
                    try:
                        del deletable_by_index[token]
                    except Exception:
                        pass
                txt_existing.platzhalter = " ".join(deletable_by_index.values())
            else:
                use_token_deletion = True
        else:
            use_token_deletion = True
        if use_token_deletion:
            for token in txt_to_delete.liste:
                try:
                    txt_existing.liste = list(filter(lambda value: value != token, txt_existing.liste))
                except Exception:
                    pass
            txt_to_delete = self.new_text_state("")
            txt_existing.platzhalter = " ".join(txt_existing.liste)
        return txt_existing.platzhalter, PromptModus.normal, txt_to_delete.text

    def apply_storage_output(self, pending_output, prompt_mode, txt_state):
        if prompt_mode == PromptModus.speicherungAusgaben:
            txt_state.text = txt_state.platzhalter
        elif prompt_mode == PromptModus.speicherungAusgabenMitZusatz:
            txt_state.text = txt_state.platzhalter + " " + pending_output
        return txt_state

    def prompt_input(self, setup: PromptLoopSetup, txt_state: PromptTextState):
        i18nRP = self.i18n.retaPrompt
        if len(setup.only_one_command) == 0:
            session = self.new_session(setup.logging_switch)
            try:
                txt_state.befehlDavor = txt_state.text
                txt_state.text = session.prompt(
                    [("class:bla", setup.prompt_prefixes[setup.prompt_mode])],
                    completer=setup.start_completer if setup.prompt_mode != PromptModus.loeschenSelect else None,
                    wrap_lines=True,
                    complete_while_typing=True,
                    vi_mode="-" + i18nRP.retaPromptParameter["vi"] in sys.argv,
                    style=Style.from_dict({"bla": "#0000ff bg:#ffff00"})
                    if setup.logging_switch
                    else Style.from_dict({"bla": "#0000ff bg:#ff0000"}),
                    placeholder=txt_state.platzhalter,
                )
                if setup.force_e_command and txt_state.text[:4] != "reta":
                    txt_state.e = [
                        self.i18n.befehle2[
                            "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                        ]
                    ]
                else:
                    txt_state.e = []
            except KeyboardInterrupt:
                raise SystemExit()
        else:
            txt_state.text = " ".join(setup.only_one_command)
            txt_state.e = []
            txt_state.befehlDavor = ""
        return txt_state


_PROMPT_SESSION_CACHE: MutableMapping[Tuple[str, str], PromptSessionBundle] = {}


def bootstrap_prompt_session(
    repo_root: Optional[Path] = None,
    architecture: Optional[RetaArchitecture] = None,
    i18n=None,
    force_rebuild: bool = False,
) -> PromptSessionBundle:
    architecture = architecture or RetaArchitecture.bootstrap(repo_root)
    if i18n is None:
        import i18n.words_runtime as i18n  # noqa: WPS433

    cache_key = (
        str(architecture.repo_root.resolve()),
        str(getattr(i18n, "__name__", type(i18n).__name__)),
    )
    if force_rebuild or cache_key not in _PROMPT_SESSION_CACHE:
        prompt_runtime = bootstrap_prompt_runtime(
            repo_root=architecture.repo_root,
            architecture=architecture,
            i18n=i18n,
        )
        completion_runtime = bootstrap_completion_runtime(
            repo_root=architecture.repo_root,
            architecture=architecture,
            i18n=i18n,
        )
        prompt_language = bootstrap_prompt_language(
            repo_root=architecture.repo_root,
            architecture=architecture,
            i18n=i18n,
        )
        _PROMPT_SESSION_CACHE[cache_key] = PromptSessionBundle(
            architecture=architecture,
            i18n=i18n,
            prompt_runtime=prompt_runtime,
            completion_runtime=completion_runtime,
            prompt_language=prompt_language,
        )
    return _PROMPT_SESSION_CACHE[cache_key]
