from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Dict, Iterable, List, Optional, Sequence, Set

from .schema import RetaContextSchema


COMMA_SPLIT_PATTERN = r",(?![^\[\]\{\}\(\)]*[\]\}\)])"


@dataclass
class RowRangeSyntax:
    multiple_prefix: str = "v"
    comma_split_pattern: str = COMMA_SPLIT_PATTERN

    @classmethod
    def from_schema(cls, schema: Optional[RetaContextSchema], i18n=None) -> "RowRangeSyntax":
        prefix = "v"
        if i18n is not None:
            try:
                prefix = str(i18n.befehle2["v"])
            except Exception:
                prefix = "v"
        return cls(multiple_prefix=prefix)

    @classmethod
    def from_i18n(cls, i18n) -> "RowRangeSyntax":
        return cls.from_schema(None, i18n=i18n)

    def split_comma_list(self, text: str) -> List[str]:
        return [segment for segment in re.split(self.comma_split_pattern, str(text))]

    def compact_comma_list(self, text: str) -> str:
        return ",".join(segment for segment in self.split_comma_list(text) if segment)

    def integer_range_pattern(self) -> re.Pattern:
        return re.compile(
            rf"^({re.escape(self.multiple_prefix)}?-?\d+)(-\d+)?((\+)(\d+))*$"
        )

    def fraction_range_pattern(self) -> re.Pattern:
        return re.compile(
            rf"^({re.escape(self.multiple_prefix)}?-?\d+/\d+)(-\d+/\d+)?((\+)(\d+/\d+))*$"
        )

    def is_integer_range_token(self, text: str) -> bool:
        return bool(re.fullmatch(self.integer_range_pattern(), str(text)))

    def is_fraction_range_token(self, text: str) -> bool:
        return bool(re.fullmatch(self.fraction_range_pattern(), str(text)))

    def snapshot(self) -> Dict[str, object]:
        return {
            "multiple_prefix": self.multiple_prefix,
            "comma_split_pattern": self.comma_split_pattern,
        }


@dataclass
class PromptVocabulary:
    main_parameters: List[str] = field(default_factory=list)
    spalten: List[str] = field(default_factory=list)
    eigs_n: List[str] = field(default_factory=list)
    eigs_r: List[str] = field(default_factory=list)
    spalten_dict: Dict[str, List[str]] = field(default_factory=dict)
    ausgabe_paras: List[str] = field(default_factory=list)
    kombi_main_paras: List[str] = field(default_factory=list)
    zeilen_paras: List[str] = field(default_factory=list)
    haupt_for_neben: List[str] = field(default_factory=list)
    not_parameter_values: List[str] = field(default_factory=list)
    haupt_for_neben_set: Set[str] = field(default_factory=set)
    ausgabe_art: List[str] = field(default_factory=list)
    zeilen_typen: List[str] = field(default_factory=list)
    zeilen_zeit: List[str] = field(default_factory=list)
    zeilen_typen_b: List[str] = field(default_factory=list)
    gebrochen_erlaubte_zahlen: Set[int] = field(default_factory=set)
    befehle: List[str] = field(default_factory=list)
    befehle2: Set[str] = field(default_factory=set)

    def snapshot(self) -> Dict[str, object]:
        return {
            "main_parameters_len": len(self.main_parameters),
            "spalten_len": len(self.spalten),
            "spalten_dict_keys": len(self.spalten_dict),
            "ausgabe_paras_len": len(self.ausgabe_paras),
            "kombi_main_paras_len": len(self.kombi_main_paras),
            "zeilen_paras_len": len(self.zeilen_paras),
            "haupt_for_neben_len": len(self.haupt_for_neben),
            "ausgabe_art_len": len(self.ausgabe_art),
            "befehle_len": len(self.befehle),
            "befehle2_len": len(self.befehle2),
            "gebrochen_erlaubte_zahlen_len": len(self.gebrochen_erlaubte_zahlen),
        }


class PromptVocabularyBuilder:
    def __init__(self, schema: RetaContextSchema, row_ranges: Optional[RowRangeSyntax] = None) -> None:
        self.schema = schema
        self.row_ranges = row_ranges or RowRangeSyntax.from_schema(schema)

    def build(self, program, i18n) -> PromptVocabulary:
        main_parameters = ["-" + str(a) for a in program.mainParaCmds]
        spalten = ["--" + str(a[0]) + "=" for a in program.paraDict.keys()] + ["--="]

        eigs_n: List[str] = []
        eigs_r: List[str] = []
        for pp in program.paraDict.keys():
            if pp[0] == i18n.konzeptE["konzept"]:
                eigs_n.append(pp[1])
            elif pp[0] == i18n.konzeptE["konzept2"]:
                eigs_r.append(pp[1])

        spalten_dict: Dict[str, List[str]] = {}
        for tupel in program.paraNdataMatrix:
            for haupt in tupel[0]:
                spalten_dict.setdefault(haupt, []).extend(list(tupel[1]))
                spalten_dict.setdefault("*", []).extend(list(tupel[1]))

        spalten.extend(
            [
                "--" + i18n.ausgabeParas["breite"] + "=",
                "--" + i18n.ausgabeParas["breiten"] + "=",
                "--" + i18n.ausgabeParas["keinenummerierung"],
                "--*=",
            ]
        )

        zeilen_typen = [
            i18n.zeilenParas["sonne"],
            i18n.zeilenParas["mond"],
            i18n.zeilenParas["planet"],
            i18n.zeilenParas["schwarzesonne"],
            i18n.zeilenParas["SonneMitMondanteil"],
            "*",
        ]
        zeilen_zeit = [
            i18n.zeilenParas["heute"],
            i18n.zeilenParas["gestern"],
            i18n.zeilenParas["morgen"],
            "*",
        ]
        zeilen_typen_b = [
            i18n.zeilenParas["aussenerste"],
            i18n.zeilenParas["innenerste"],
            i18n.zeilenParas["aussenalle"],
            i18n.zeilenParas["innenalle"],
            "*",
        ]

        ausgabe_paras = [
            "--" + s + ("=" if l else "")
            for s, l in zip(i18n.ausgabeParas.values(), i18n.ausgabeParasEqSign.values())
        ] + ["--*="]

        kombi_main_paras = [
            "--" + i18n.kombiMainParas["galaxie"] + "=",
            "--" + i18n.kombiMainParas["universum"] + "=",
            "--*=",
        ]

        zeilen_paras = [
            "--" + i18n.zeilenParas["zeit"] + "=",
            "--" + i18n.zeilenParas["zaehlung"] + "=",
            "--" + i18n.zeilenParas["vorhervonausschnitt"] + "=",
            "--" + i18n.zeilenParas["vorhervonausschnittteiler"],
            "--" + i18n.zeilenParas["primzahlvielfache"] + "=",
            "--" + i18n.zeilenParas["nachtraeglichneuabzaehlung"] + "=",
            "--" + i18n.zeilenParas["nachtraeglichneuabzaehlungvielfache"] + "=",
            "--" + i18n.zeilenParas["alles"],
            "--" + i18n.zeilenParas["potenzenvonzahlen"] + "=",
            "--" + i18n.zeilenParas["typ"] + "=",
            "--" + i18n.zeilenParas["vielfachevonzahlen"] + "=",
            "--" + i18n.zeilenParas["oberesmaximum"] + "=",
            "--" + i18n.zeilenParas["primzahlen"] + "=",
            "--" + i18n.zeilenParas["invertieren"],
            "--*=",
        ]

        haupt_for_neben = [
            "-" + s
            for s in set(i18n.hauptForNeben.values()) - {i18n.mainParaCmds["debug"]}
        ]
        not_parameter_values = ausgabe_paras + zeilen_paras + kombi_main_paras + spalten + main_parameters
        haupt_for_neben_set = set(haupt_for_neben)
        ausgabe_art = list(i18n.ausgabeArt.values())

        zum_vergleich: List[str] = []
        gebrochen_erlaubte_zahlen: Set[int] = set()
        for a in program.paraNdataMatrix:
            for b in a[1]:
                zum_vergleich.append(b)
                if len(set(a[0]) & i18n.gebrochenUniGalEinzeln) > 0:
                    gebrochen_erlaubte_zahlen |= {int(b)}
        if gebrochen_erlaubte_zahlen:
            gebrochen_erlaubte_zahlen -= {max(gebrochen_erlaubte_zahlen)}

        befehle = list(i18n.befehle)
        befehle += [i18n.EIGS_N_R[0] + a for a in eigs_n] + [i18n.EIGS_N_R[1] + a for a in eigs_r]
        befehle2 = set(befehle) - {"reta"}

        return PromptVocabulary(
            main_parameters=main_parameters,
            spalten=spalten,
            eigs_n=eigs_n,
            eigs_r=eigs_r,
            spalten_dict=spalten_dict,
            ausgabe_paras=ausgabe_paras,
            kombi_main_paras=kombi_main_paras,
            zeilen_paras=zeilen_paras,
            haupt_for_neben=haupt_for_neben,
            not_parameter_values=not_parameter_values,
            haupt_for_neben_set=haupt_for_neben_set,
            ausgabe_art=ausgabe_art,
            zeilen_typen=zeilen_typen,
            zeilen_zeit=zeilen_zeit,
            zeilen_typen_b=zeilen_typen_b,
            gebrochen_erlaubte_zahlen=gebrochen_erlaubte_zahlen,
            befehle=befehle,
            befehle2=befehle2,
        )


@dataclass
class InputBundle:
    schema: RetaContextSchema
    row_ranges: RowRangeSyntax
    prompt_vocabulary_builder: PromptVocabularyBuilder

    @classmethod
    def from_schema(cls, schema: RetaContextSchema, i18n=None) -> "InputBundle":
        row_ranges = RowRangeSyntax.from_schema(schema, i18n=i18n)
        return cls(
            schema=schema,
            row_ranges=row_ranges,
            prompt_vocabulary_builder=PromptVocabularyBuilder(schema, row_ranges=row_ranges),
        )

    def build_prompt_vocabulary(self, program, i18n) -> PromptVocabulary:
        return self.prompt_vocabulary_builder.build(program, i18n)

    def snapshot(self) -> Dict[str, object]:
        return {
            "row_ranges": self.row_ranges.snapshot(),
            "prompt_vocabulary_builder": {
                "available": True,
            },
        }
