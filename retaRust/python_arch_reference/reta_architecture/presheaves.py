from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Iterable, List, Mapping, MutableMapping, Optional, Sequence, Tuple

from .topology import ContextSelection


@dataclass
class LocalSection:
    context: ContextSelection
    payload: object
    source: Optional[str] = None

    def as_dict(self) -> Dict[str, object]:
        return {
            "context": self.context.as_dict(),
            "payload": self.payload,
            "source": self.source,
        }


class Presheaf:
    """Minimal symbolic presheaf used for architectural bookkeeping."""

    def __init__(self, name: str) -> None:
        self.name = name
        self._sections: List[LocalSection] = []

    def add_section(self, context: ContextSelection, payload: object, source: Optional[str] = None) -> None:
        self._sections.append(LocalSection(context=context, payload=payload, source=source))

    def restrict(self, context: ContextSelection) -> List[LocalSection]:
        result = []
        for section in self._sections:
            refined = section.context.refine(context)
            if not refined.is_empty():
                result.append(LocalSection(refined, section.payload, section.source))
        return result

    def sections(self) -> List[LocalSection]:
        return list(self._sections)

    def snapshot(self) -> List[Dict[str, object]]:
        return [section.as_dict() for section in self._sections]


class FilesystemPresheaf(Presheaf):
    def __init__(self, name: str, root: Path) -> None:
        super().__init__(name)
        self.root = root

    def discover(self, patterns: Iterable[str], context_builder) -> None:
        for pattern in patterns:
            for path in sorted(self.root.glob(pattern)):
                context = context_builder(path)
                payload = {
                    "path": str(path.relative_to(self.root)),
                    "suffix": path.suffix,
                    "name": path.name,
                }
                self.add_section(context, payload, source=str(path))


class PromptStatePresheaf(Presheaf):
    def __init__(self) -> None:
        super().__init__("prompt_state")
        self.raw_text: str = ""
        self.tokenized_text: List[str] = []

    def update(self, raw_text: str, tokens: Sequence[str], context: Optional[ContextSelection] = None) -> None:
        self.raw_text = raw_text
        self.tokenized_text = list(tokens)
        self._sections = []
        self.add_section(
            context or ContextSelection(scopes=frozenset({"prompt"})),
            {"raw_text": self.raw_text, "tokens": list(self.tokenized_text)},
            source="prompt",
        )


@dataclass
class PresheafBundle:
    csv: FilesystemPresheaf
    translations: FilesystemPresheaf
    assets: FilesystemPresheaf
    prompt_state: PromptStatePresheaf

    @classmethod
    def discover(cls, repo_root: Path) -> "PresheafBundle":
        csv = FilesystemPresheaf("csv", repo_root)
        translations = FilesystemPresheaf("translations", repo_root)
        assets = FilesystemPresheaf("assets", repo_root)
        prompt_state = PromptStatePresheaf()

        def csv_context(path: Path) -> ContextSelection:
            stem = path.stem
            lang = None
            if "-" in stem:
                maybe_lang, _rest = stem.split("-", 1)
                if len(maybe_lang) in (2, 3):
                    lang = maybe_lang
            return ContextSelection(
                language=frozenset({lang}) if lang else None,
                scopes=frozenset({"csv"}),
            )

        def translation_context(path: Path) -> ContextSelection:
            language = None
            parts = path.parts
            if "i18n" in parts:
                idx = parts.index("i18n")
                if idx + 1 < len(parts):
                    maybe = parts[idx + 1]
                    if maybe not in {"de", "en", "vn", "cn", "kr"}:
                        language = None
                    else:
                        language = maybe
            return ContextSelection(
                language=frozenset({language}) if language else None,
                scopes=frozenset({"i18n"}),
            )

        def asset_context(path: Path) -> ContextSelection:
            scope = path.suffix.lstrip(".") or "file"
            return ContextSelection(scopes=frozenset({scope}))

        csv.discover(["csv/*.csv"], csv_context)
        translations.discover(["i18n/**/*.po", "i18n/**/*.mo", "i18n/**/*.pot"], translation_context)
        assets.discover([
            "*.md",
            "*.org",
            "*.alx",
            "*.jsonl",
            "*.js",
            "*.ts",
            "doc/*.md",
            "doc/*.org",
        ], asset_context)
        return cls(csv=csv, translations=translations, assets=assets, prompt_state=prompt_state)

    def snapshot(self) -> Dict[str, object]:
        return {
            "csv": self.csv.snapshot(),
            "translations": self.translations.snapshot(),
            "assets": self.assets.snapshot(),
            "prompt_state": self.prompt_state.snapshot(),
        }
