from __future__ import annotations

import sys
from copy import copy
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable, Optional, Sequence

from .completion_runtime import CompletionRuntimeBundle, bootstrap_completion_runtime
from .prompt_execution import PromptExecutionBundle, bootstrap_prompt_execution
from .prompt_language import PromptModus, custom_split
from .prompt_preparation import PromptPreparationBundle, bootstrap_prompt_preparation
from .prompt_runtime import PromptRuntimeBundle, bootstrap_prompt_runtime
from .prompt_session import PromptLoopSetup, PromptSessionBundle, PromptTextState, bootstrap_prompt_session


@dataclass
class PromptInteractionBundle:
    """Architecture-owned controller for the interactive ``retaPrompt`` loop."""

    architecture: object
    i18n: object
    prompt_runtime: PromptRuntimeBundle
    completion_runtime: CompletionRuntimeBundle
    prompt_session: PromptSessionBundle
    prompt_preparation: PromptPreparationBundle
    prompt_execution: PromptExecutionBundle
    nested_completer_cls: Optional[type] = None
    compl_situa_enum: Optional[object] = None
    reta_prompt_help: Optional[Callable[[], None]] = None
    cliout: Optional[Callable[[str, bool], None]] = None
    is_zeilenangabe: Optional[Callable[[str], bool]] = None
    bereich_to_numbers: Optional[Callable[..., set]] = None
    debug_output: Optional[Callable[[], None]] = None
    prompt_mode2: PromptModus = PromptModus.normal
    text_dazu0: list = field(default_factory=list)
    sprachen_wahl: str = "deutsch"
    info_log: bool = False

    def __post_init__(self) -> None:
        self.i18nRP = self.i18n.retaPrompt
        self.reta_program = self.prompt_runtime.program
        self.befehle = set(self.completion_runtime.start_commands(include_numeric_shortcuts=True))
        self.befehle2 = set(self.completion_runtime.befehle2)
        self.befehle_beenden = set(self.i18nRP.befehleBeenden)

    def snapshot(self):
        return {
            "class": type(self).__name__,
            "prompt_mode2": self.prompt_mode2.name,
            "text_dazu0_len": len(self.text_dazu0),
            "befehle_len": len(self.befehle),
            "befehle2_len": len(self.befehle2),
            "befehle_beenden_len": len(self.befehle_beenden),
            "has_nested_completer": self.nested_completer_cls is not None,
            "has_reta_prompt_help": self.reta_prompt_help is not None,
            "session_layer": type(self.prompt_session).__name__,
            "preparation_layer": type(self.prompt_preparation).__name__,
            "execution_layer": type(self.prompt_execution).__name__,
        }

    def new_session(self, history: bool = False):
        return self.prompt_session.new_session(history)

    def store_prompt(self, chains, placeholder, text):
        result = self.prompt_session.store_prompt(
            chains,
            placeholder,
            text,
            self.prompt_mode2,
            self.befehle,
            self.prompt_preparation.prepare_grosse_ausgabe,
        )
        self.prompt_mode2 = result.prompt_mode2
        self.text_dazu0 = result.text_dazu0
        return result.chains, result.text_state

    def _require_loop_deps(self) -> None:
        missing = [
            name
            for name, value in {
                "nested_completer_cls": self.nested_completer_cls,
                "compl_situa_enum": self.compl_situa_enum,
                "reta_prompt_help": self.reta_prompt_help,
            }.items()
            if value is None
        ]
        if missing:
            raise RuntimeError("missing prompt loop dependencies: " + ", ".join(missing))

    def build_loop_setup(self, argv: Optional[Sequence[str]] = None) -> PromptLoopSetup:
        self._require_loop_deps()
        setup = self.prompt_session.build_loop_setup(
            list(sys.argv if argv is None else argv),
            self.nested_completer_cls,
            self.compl_situa_enum,
            self.reta_prompt_help,
            prompt_program=self.reta_program,
            debug_output=self.debug_output,
        )
        self.prompt_mode2 = setup.prompt_mode2
        self.text_dazu0 = list(setup.text_dazu0)
        self.befehle_beenden = set(setup.befehle_beenden)
        return setup

    def loop_setup_legacy_tuple(self, argv: Optional[Sequence[str]] = None):
        setup = self.build_loop_setup(argv)
        return (
            setup.befehle_beenden,
            setup.logging_switch,
            setup.prompt_prefixes,
            setup.prompt_mode,
            setup.start_completer,
            setup.only_one_command,
            setup.force_e_command,
        )

    def delete_before_storage_commands(self, placeholder, prompt_mode, text):
        return self.prompt_session.delete_before_storage_commands(
            placeholder,
            prompt_mode,
            text,
            self.is_zeilenangabe,
            self.bereich_to_numbers,
        )

    def apply_storage_output(self, pending_output, prompt_mode, text_state):
        return self.prompt_session.apply_storage_output(pending_output, prompt_mode, text_state)

    def prompt_storage_after_input(self, chains, prompt_mode, text_state):
        if prompt_mode == PromptModus.speichern:
            chains, text_state = self.store_prompt(chains, text_state.platzhalter, text_state.text)
        return chains, text_state

    def prompt_input(self, logging_switch, prompt_prefixes, prompt_mode, start_completer, text_state, only_one_command, force_e_command):
        setup = PromptLoopSetup(
            befehle_beenden=set(self.befehle_beenden),
            logging_switch=logging_switch,
            prompt_mode=prompt_mode,
            prompt_mode2=prompt_mode,
            prompt_prefixes=prompt_prefixes,
            start_completer=start_completer,
            only_one_command=only_one_command,
            force_e_command=force_e_command,
            text_dazu0=[],
        )
        return self.prompt_session.prompt_input(setup, text_state)

    def _storage_command(self, text_state, prompt_mode, chains, pending_output):
        save_after = {self.i18n.befehle2["S"], self.i18n.befehle2["BefehlSpeichernDanach"]}
        save_before = {self.i18n.befehle2["s"], self.i18n.befehle2["BefehlSpeichernDavor"]}
        output_saved = {self.i18n.befehle2["o"], self.i18n.befehle2["BefehlSpeicherungAusgeben"]}
        delete_saved = {self.i18n.befehle2["l"], self.i18n.befehle2["BefehlSpeicherungLöschen"]}
        abc = {self.i18n.befehle2["abc"], self.i18n.befehle2["abcd"]}
        save_all = save_after | save_before
        if text_state.hasWithoutABC(save_after) and len(text_state.liste) == 1:
            return True, PromptModus.speichern, chains, pending_output, text_state
        if text_state.hasWithoutABC(save_before) and len(text_state.liste) == 1:
            chains, text_state = self.store_prompt(chains, text_state.platzhalter, text_state.befehlDavor)
            return True, PromptModus.normal, chains, pending_output, text_state
        if len(text_state.menge - save_all) > 0 and len(text_state.menge & save_all) == 1 and not text_state.has(abc):
            storage_text = copy(text_state.liste)
            for token in save_all:
                try:
                    storage_text.remove(token)
                except ValueError:
                    pass
            chains, text_state = self.store_prompt(chains, text_state.platzhalter, " ".join(storage_text))
            text_state.liste = []
            text_state.text = ""
            text_state.befehlDavor = ""
            return True, PromptModus.normal, chains, pending_output, text_state
        if text_state.hasWithoutABC(output_saved) and len(text_state.liste) == 1:
            return True, PromptModus.speicherungAusgaben, chains, pending_output, text_state
        if text_state.hasWithoutABC(output_saved) and len(text_state.menge - output_saved) > 1:
            return True, PromptModus.speicherungAusgabenMitZusatz, chains, text_state.liste, text_state
        if text_state.hasWithoutABC(delete_saved) and len(text_state.liste) == 1:
            payload = str([{i + 1, a} for i, a in enumerate(custom_split(text_state.platzhalter))])
            if "--" + self.i18n.ausgabeParas["nocolor"] in text_state.listeE or self.cliout is None:
                print(payload)
            else:
                self.cliout(payload, True)
                return True, PromptModus.loeschenSelect, chains, pending_output, text_state
        return False, prompt_mode, chains, pending_output, text_state

    def _execute(self, text_state, prompt_mode, prompt_mode_last, chains, logging_switch, pending_output, only_one_command):
        text1, text2, text3 = self.prompt_preparation.rotate_where_reta_command(
            text_state.platzhalter,
            text_state.text,
            self.text_dazu0,
            prompt_mode,
        )
        prepared = self.prompt_preparation.prepare_grosse_ausgabe(
            text1,
            prompt_mode,
            self.prompt_mode2,
            prompt_mode_last,
            text2,
            text3,
        )
        is_reta, fractions, row_range, chains, max_num, text_state.liste, numeric_specs, is_short = prepared
        logging_switch = self.prompt_execution.run_grosse_ausgabe(
            is_reta,
            self.befehle_beenden,
            fractions,
            row_range,
            chains,
            logging_switch,
            max_num,
            False,
            numeric_specs,
            is_short,
            only_one_command,
            text_state,
        )
        return logging_switch, chains, pending_output, text_state

    def run_scope(self, argv: Optional[Sequence[str]] = None):
        setup = self.build_loop_setup(argv)
        logging_switch = setup.logging_switch
        prompt_mode = setup.prompt_mode
        text_state = self.prompt_session.new_text_state("")
        pending_output = ""
        chains = []
        while len(text_state.menge & self.befehle_beenden) == 0 or text_state.has({self.i18n.befehle2["abc"], self.i18n.befehle2["abcd"]}):
            prompt_mode_last = prompt_mode
            if prompt_mode not in (PromptModus.speicherungAusgaben, PromptModus.speicherungAusgabenMitZusatz):
                text_state = self.prompt_input(logging_switch, setup.prompt_prefixes, prompt_mode, setup.start_completer, text_state, setup.only_one_command, setup.force_e_command)
                chains, text_state = self.prompt_storage_after_input(chains, prompt_mode, text_state)
            else:
                text_state = self.apply_storage_output(pending_output, prompt_mode, text_state)
            if prompt_mode == PromptModus.loeschenSelect:
                text_state.text, prompt_mode, _ = self.delete_before_storage_commands(text_state.platzhalter, prompt_mode, text_state.text)
                text_state.platzhalter = text_state.text
                self.text_dazu0 = text_state.liste
                continue
            prompt_mode = PromptModus.normal
            handled, prompt_mode, chains, pending_output, text_state = self._storage_command(text_state, prompt_mode, chains, pending_output)
            if handled:
                continue
            logging_switch, chains, pending_output, text_state = self._execute(text_state, prompt_mode, prompt_mode_last, chains, logging_switch, pending_output, setup.only_one_command)
        return self.sprachen_wahl


def bootstrap_prompt_interaction(*, architecture=None, repo_root: Optional[Path] = None, i18n=None, prompt_runtime=None, completion_runtime=None, prompt_session=None, prompt_preparation=None, prompt_execution=None, nested_completer_cls=None, compl_situa_enum=None, reta_prompt_help=None, cliout=None, is_zeilenangabe=None, bereich_to_numbers=None, debug_output=None, force_rebuild: bool = False) -> PromptInteractionBundle:
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent
    if architecture is None:
        from .facade import RetaArchitecture
        architecture = RetaArchitecture.bootstrap(repo_root)
    if i18n is None:
        import i18n.words_runtime as i18n  # noqa: WPS433
    prompt_runtime = prompt_runtime or bootstrap_prompt_runtime(repo_root=architecture.repo_root, architecture=architecture, i18n=i18n, force_rebuild=force_rebuild)
    completion_runtime = completion_runtime or bootstrap_completion_runtime(repo_root=architecture.repo_root, architecture=architecture, i18n=i18n, force_rebuild=force_rebuild)
    prompt_session = prompt_session or bootstrap_prompt_session(repo_root=architecture.repo_root, architecture=architecture, i18n=i18n, force_rebuild=force_rebuild)
    prompt_preparation = prompt_preparation or bootstrap_prompt_preparation(architecture=architecture, i18n=i18n, prompt_runtime=prompt_runtime, prompt_session=prompt_session, force_rebuild=force_rebuild)
    prompt_execution = prompt_execution or bootstrap_prompt_execution(architecture=architecture, i18n=i18n, prompt_runtime=prompt_runtime, completion_runtime=completion_runtime, force_rebuild=force_rebuild)
    return PromptInteractionBundle(
        architecture=architecture,
        i18n=i18n,
        prompt_runtime=prompt_runtime,
        completion_runtime=completion_runtime,
        prompt_session=prompt_session,
        prompt_preparation=prompt_preparation,
        prompt_execution=prompt_execution,
        nested_completer_cls=nested_completer_cls,
        compl_situa_enum=compl_situa_enum,
        reta_prompt_help=reta_prompt_help,
        cliout=cliout,
        is_zeilenangabe=is_zeilenangabe,
        bereich_to_numbers=bereich_to_numbers,
        debug_output=debug_output,
    )
