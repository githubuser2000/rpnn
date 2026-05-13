from pathlib import Path

from center import BereichToNumbers2, Primzahlkreuz_pro_contra_strs, i18n, x
from reta_architecture import (
    RetaArchitecture,
    PromptModus,
    bootstrap_completion_runtime,
    bootstrap_prompt_language,
    bootstrap_prompt_runtime,
    bootstrap_prompt_session,
    custom_split,
    custom_split2,
    is15or16command,
    isReTaParameter,
    stextFromKleinKleinKleinBefehl,
    verifyBruchNganzZahlBetweenCommas,
    verifyBruchNganzZahlCommaList,
    verkuerze_dict,
)

REPO_ROOT = Path(__file__).resolve().parent.parent

# Build the architecture once and share it across the import-time prompt
# compatibility bundles.  This keeps the legacy facade source-compatible while
# avoiding repeated full meta-layer bootstraps after the Stage-31 validation and
# coherence layers were added.
_ARCHITECTURE = RetaArchitecture.bootstrap(REPO_ROOT)

promptRuntime = bootstrap_prompt_runtime(
    repo_root=REPO_ROOT,
    architecture=_ARCHITECTURE,
    i18n=i18n,
)
completionRuntime = bootstrap_completion_runtime(
    repo_root=REPO_ROOT,
    architecture=_ARCHITECTURE,
    i18n=i18n,
)
promptLanguage = bootstrap_prompt_language(
    repo_root=REPO_ROOT,
    architecture=_ARCHITECTURE,
    i18n=i18n,
)
promptSession = bootstrap_prompt_session(
    repo_root=REPO_ROOT,
    architecture=_ARCHITECTURE,
    i18n=i18n,
)

retaProgram = promptRuntime.program
promptVocabulary = promptRuntime.vocabulary

mainParas = list(promptVocabulary.main_parameters)
spalten = list(promptVocabulary.spalten)
eigsN = list(promptVocabulary.eigs_n)
eigsR = list(promptVocabulary.eigs_r)
spaltenDict = {key: list(value) for key, value in promptVocabulary.spalten_dict.items()}
zeilenTypen = list(promptVocabulary.zeilen_typen)
zeilenZeit = list(promptVocabulary.zeilen_zeit)
zeilenTypenB = list(promptVocabulary.zeilen_typen_b)
ausgabeParas = list(promptVocabulary.ausgabe_paras)
kombiMainParas = list(promptVocabulary.kombi_main_paras)
zeilenParas = list(promptVocabulary.zeilen_paras)
hauptForNeben = list(promptVocabulary.haupt_for_neben)
notParameterValues = list(promptLanguage.not_parameter_values)
hauptForNebenSet = set(promptVocabulary.haupt_for_neben_set)
ausgabeArt = list(promptVocabulary.ausgabe_art)
gebrochenErlaubteZahlen = set(promptLanguage.gebrochen_erlaubte_zahlen)
missingWahl15Values = list(promptRuntime.validation.get("wahl15_missing_values", []))
befehle = list(completionRuntime.befehle)
befehle2 = set(completionRuntime.befehle2)
wahl15 = dict(promptLanguage.wahl15)
wahl16 = dict(promptLanguage.wahl16)

if len(missingWahl15Values) > 0:
    for missingValue in missingWahl15Values:
        print(missingValue)
    print()
    print("assert fehlgeschlagen")
    exit()
