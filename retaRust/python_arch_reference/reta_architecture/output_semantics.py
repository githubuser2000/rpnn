from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path
from typing import Callable, Dict, Optional, Tuple


@dataclass(frozen=True)
class OutputModeSpec:
    canonical_name: str
    cli_value: str
    syntax_class: type
    force_one_table: bool = False
    force_zero_width: bool = False
    marks_html_or_bbcode: bool = False
    aliases: Tuple[str, ...] = tuple()

    def snapshot(self) -> Dict[str, object]:
        return {
            "canonical_name": self.canonical_name,
            "cli_value": self.cli_value,
            "syntax_class": getattr(self.syntax_class, "__name__", str(self.syntax_class)),
            "force_one_table": self.force_one_table,
            "force_zero_width": self.force_zero_width,
            "marks_html_or_bbcode": self.marks_html_or_bbcode,
            "aliases": list(self.aliases),
        }


@dataclass(frozen=True)
class OutputModeApplication:
    canonical_name: str
    syntax_class_name: str
    force_one_table: bool
    force_zero_width: bool
    marks_html_or_bbcode: bool

    def snapshot(self) -> Dict[str, object]:
        return {
            "canonical_name": self.canonical_name,
            "syntax_class_name": self.syntax_class_name,
            "force_one_table": self.force_one_table,
            "force_zero_width": self.force_zero_width,
            "marks_html_or_bbcode": self.marks_html_or_bbcode,
        }


class RetaOutputSemantics:
    def __init__(self, repo_root: Path, mode_specs: Dict[str, OutputModeSpec]) -> None:
        self.repo_root = Path(repo_root)
        self.mode_specs = dict(mode_specs)
        self.alias_to_mode: Dict[str, str] = {}
        self.syntax_class_to_mode: Dict[type, str] = {}
        for canonical_name, spec in self.mode_specs.items():
            self.alias_to_mode[canonical_name] = canonical_name
            self.alias_to_mode[spec.cli_value] = canonical_name
            for alias in spec.aliases:
                self.alias_to_mode[str(alias)] = canonical_name
            self.syntax_class_to_mode[spec.syntax_class] = canonical_name

    def canonicalize(self, value: Optional[str]) -> Optional[str]:
        if value is None:
            return None
        return self.alias_to_mode.get(str(value))

    def spec_for(self, value: Optional[str]) -> Optional[OutputModeSpec]:
        canonical = self.canonicalize(value)
        if canonical is None:
            return None
        return self.mode_specs.get(canonical)

    def create_syntax(self, value: str):
        spec = self.spec_for(value)
        if spec is None:
            raise KeyError(value)
        return spec.syntax_class()

    def mode_for_output_syntax(self, out_type) -> Optional[str]:
        if out_type is None:
            return None
        mode_name = getattr(out_type, "mode_name", None)
        if mode_name is not None:
            canonical = self.canonicalize(str(mode_name))
            if canonical is not None:
                return canonical
        return self.syntax_class_to_mode.get(type(out_type))

    def mode_for_tables(self, tables) -> str:
        out_type = getattr(tables, "outType", None)
        mode = self.mode_for_output_syntax(out_type)
        return mode or "shell"

    def is_mode(self, tables, mode: str) -> bool:
        canonical = self.canonicalize(mode)
        return canonical is not None and self.mode_for_tables(tables) == canonical

    def apply_mode_to_tables(
        self,
        tables,
        mode: str,
        zero_width_callback: Optional[Callable[[], None]] = None,
    ) -> Optional[OutputModeApplication]:
        spec = self.spec_for(mode)
        if spec is None:
            return None
        tables.outType = spec.syntax_class()
        if spec.force_one_table:
            tables.getOut.oneTable = True
        if spec.force_zero_width and zero_width_callback is not None:
            zero_width_callback()
        return OutputModeApplication(
            canonical_name=spec.canonical_name,
            syntax_class_name=getattr(spec.syntax_class, "__name__", str(spec.syntax_class)),
            force_one_table=spec.force_one_table,
            force_zero_width=spec.force_zero_width,
            marks_html_or_bbcode=spec.marks_html_or_bbcode,
        )

    def snapshot(self) -> Dict[str, object]:
        return {
            "available_modes": sorted(self.mode_specs.keys()),
            "mode_specs": {key: self.mode_specs[key].snapshot() for key in sorted(self.mode_specs)},
        }


@lru_cache(maxsize=4)
def _bootstrap_output_semantics(repo_root_str: str) -> RetaOutputSemantics:
    repo_root = Path(repo_root_str)

    import i18n.words_context as words_context
    from .output_syntax import OUTPUT_SYNTAX_CLASSES

    mode_classes = dict(OUTPUT_SYNTAX_CLASSES)

    mode_specs: Dict[str, OutputModeSpec] = {}
    for canonical_name, cli_value in dict(words_context.ausgabeArt).items():
        syntax_class = mode_classes[canonical_name]
        aliases = tuple(dict.fromkeys((canonical_name, str(cli_value), getattr(syntax_class, "mode_name", canonical_name))))
        mode_specs[canonical_name] = OutputModeSpec(
            canonical_name=canonical_name,
            cli_value=str(cli_value),
            syntax_class=syntax_class,
            force_one_table=bool(getattr(syntax_class, "force_one_table", False)),
            force_zero_width=bool(getattr(syntax_class, "force_zero_width", False)),
            marks_html_or_bbcode=bool(getattr(syntax_class, "marks_html_or_bbcode", False)),
            aliases=aliases,
        )
    return RetaOutputSemantics(repo_root=repo_root, mode_specs=mode_specs)


def bootstrap_output_semantics(repo_root: Optional[Path] = None) -> RetaOutputSemantics:
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent
    return _bootstrap_output_semantics(str(Path(repo_root).resolve()))
