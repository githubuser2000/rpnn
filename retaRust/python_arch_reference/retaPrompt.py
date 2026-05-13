#!/usr/bin/env pypye
# -*- coding: utf-8 -*-
"""Interactive Reta prompt compatibility facade.

The prompt runtime is now owned by ``reta_architecture.prompt_interaction``.
This module intentionally remains thin so the historical launchers (``rp``,
``rpl``, ``retaPrompt``) can continue importing ``retaPrompt.start()`` while the
actual controller logic lives inside the explicit architecture layer.
"""

from __future__ import annotations

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "libs"))

from center import BereichToNumbers2, cliout, i18n, isZeilenAngabe, retaPromptHilfe, x
from LibRetaPrompt import completionRuntime, gebrochenErlaubteZahlen, promptRuntime, promptSession, wahl15, wahl16
from nestedAlx import ComplSitua, NestedCompleter
from reta_architecture import PromptModus, PromptTextState
from reta_architecture.parallel_execution import apply_parallel_environment, bootstrap_parallel_execution, extract_parallel_config_from_argv
from reta_architecture.prompt_execution import PromptGrosseAusgabe, bruchBereichsManagementAndWbefehl, retaExecuteNprint, zeiln1234create
from reta_architecture.prompt_interaction import PromptInteractionBundle, bootstrap_prompt_interaction


sys.argv, prompt_parallel_config = extract_parallel_config_from_argv(sys.argv)
apply_parallel_environment(prompt_parallel_config)

i18nRP = i18n.retaPrompt
retaProgram = promptRuntime.program
retaProgram.parallel_config = prompt_parallel_config
architecture = retaProgram.architecture
architecture.parallel_execution = bootstrap_parallel_execution(prompt_parallel_config)
promptPreparation = architecture.bootstrap_prompt_preparation(i18n=i18n)
promptExecution = architecture.bootstrap_prompt_execution(i18n=i18n)
promptInteraction = bootstrap_prompt_interaction(
    architecture=architecture,
    i18n=i18n,
    prompt_runtime=promptRuntime,
    completion_runtime=completionRuntime,
    prompt_session=promptSession,
    prompt_preparation=promptPreparation,
    prompt_execution=promptExecution,
    nested_completer_cls=NestedCompleter,
    compl_situa_enum=ComplSitua,
    reta_prompt_help=retaPromptHilfe,
    cliout=cliout,
    is_zeilenangabe=isZeilenAngabe,
    bereich_to_numbers=BereichToNumbers2,
    debug_output=lambda: x("T", i18nRP.infoDebugAktiv),
)

TXT = PromptTextState
PromptScopeController = PromptInteractionBundle
befehle = promptInteraction.befehle
befehle2 = promptInteraction.befehle2
befehleBeenden = promptInteraction.befehle_beenden
infoLog = promptInteraction.info_log
sprachenWahl = promptInteraction.sprachen_wahl
promptMode2 = promptInteraction.prompt_mode2
textDazu0 = promptInteraction.text_dazu0
verdreheWoReTaBefehl = getattr(promptPreparation, "rotate_where_reta_command")
regExReplace = getattr(promptPreparation, "regex_replace")
promptVorbereitungGrosseAusgabe = getattr(promptPreparation, "prepare_grosse_ausgabe")


def _sync_from_controller() -> None:
    global befehle, befehle2, befehleBeenden, infoLog, sprachenWahl, promptMode2, textDazu0
    befehle = promptInteraction.befehle
    befehle2 = promptInteraction.befehle2
    befehleBeenden = promptInteraction.befehle_beenden
    infoLog = promptInteraction.info_log
    sprachenWahl = promptInteraction.sprachen_wahl
    promptMode2 = promptInteraction.prompt_mode2
    textDazu0 = promptInteraction.text_dazu0


def newSession(history=False):
    return promptInteraction.new_session(history)


def speichern(ketten, platzhalter, text):
    result = promptInteraction.store_prompt(ketten, platzhalter, text)
    _sync_from_controller()
    return result


def PromptAllesVorGroesserSchleife():
    result = promptInteraction.loop_setup_legacy_tuple(sys.argv)
    _sync_from_controller()
    return result


def PromptLoescheVorSpeicherungBefehle(platzhalter, promptMode, text):
    result = promptInteraction.delete_before_storage_commands(platzhalter, promptMode, text)
    _sync_from_controller()
    return result


def promptSpeicherungB(nochAusageben, promptMode, Txt):
    return promptInteraction.apply_storage_output(nochAusageben, promptMode, Txt)


def promptSpeicherungA(ketten, promptMode, Txt):
    result = promptInteraction.prompt_storage_after_input(ketten, promptMode, Txt)
    _sync_from_controller()
    return result


def promptInput(loggingSwitch, promptDavorDict, promptMode, startpunkt1, Txt, nurEinBefehl, immerEbefehlJa):
    return promptInteraction.prompt_input(loggingSwitch, promptDavorDict, promptMode, startpunkt1, Txt, nurEinBefehl, immerEbefehlJa)


def PromptScope():
    result = promptInteraction.run_scope(sys.argv)
    _sync_from_controller()
    return result


def start(sprachenWahl1="deutsch"):
    global sprachenWahl
    promptInteraction.sprachen_wahl = sprachenWahl1
    result = PromptScope()
    sprachenWahl = promptInteraction.sprachen_wahl
    return result


if __name__ == "__main__":
    PromptScope()
