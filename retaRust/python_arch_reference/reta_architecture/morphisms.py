from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Dict, Iterable, List, Optional, Tuple

from .topology import RetaContextTopology


@dataclass
class AliasMorphisms:
    topology: RetaContextTopology
    parameter_semantics: object

    def resolve_main_alias(self, main_name: str) -> Optional[str]:
        return self.parameter_semantics.resolve_main_alias(main_name)

    def resolve_parameter_alias(self, main_name: str, parameter_name: str) -> Optional[str]:
        return self.parameter_semantics.resolve_parameter_alias(main_name, parameter_name)

    def canonicalize_pair(self, main_name: str, parameter_name: str) -> Optional[Tuple[str, str]]:
        return self.parameter_semantics.canonicalize_pair(main_name, parameter_name)

    def column_numbers_for_pair(self, main_name: str, parameter_name: str) -> List[int]:
        return self.parameter_semantics.column_numbers_for_pair(main_name, parameter_name)


@dataclass
class RangeMorphisms:
    topology: RetaContextTopology

    def parse_row_range(self, text: str, parser: Callable[[str], Iterable[int]]) -> List[int]:
        return sorted(set(int(v) for v in parser(text)))


@dataclass
class PromptMorphisms:
    topology: RetaContextTopology

    def split(self, text: str, splitter: Callable[[str], List[str]]) -> List[str]:
        return splitter(text)

    def split_prompt_text(self, text: str, splitter: Callable[[str], List[str]]) -> List[str]:
        return self.split(text, splitter)

    def split_command_words(self, text: str, splitter: Callable[[str], List[str]]) -> List[str]:
        if text[:4] != "reta":
            return self.split(text, splitter)
        return [s.strip() for s in text.split() if len(s.strip()) > 0]

    def expand_shorthand(self, prompt_mode, stext, text_dazu, expander: Callable) -> object:
        return expander(prompt_mode, stext, text_dazu)


@dataclass
class RendererMorphisms:
    topology: RetaContextTopology
    output_semantics: object

    def output_mode_for_tables(self, tables) -> str:
        return self.output_semantics.mode_for_tables(tables)

    def apply_output_mode(self, tables, mode: str, zero_width_callback=None):
        return self.output_semantics.apply_mode_to_tables(
            tables,
            mode,
            zero_width_callback=zero_width_callback,
        )


@dataclass
class MorphismBundle:
    alias: AliasMorphisms
    ranges: RangeMorphisms
    prompt: PromptMorphisms
    renderers: RendererMorphisms

    @classmethod
    def from_topology_and_sheaves(cls, topology: RetaContextTopology, sheaves, output_semantics=None) -> "MorphismBundle":
        return cls(
            alias=AliasMorphisms(topology=topology, parameter_semantics=sheaves.parameter_semantics),
            ranges=RangeMorphisms(topology=topology),
            prompt=PromptMorphisms(topology=topology),
            renderers=RendererMorphisms(topology=topology, output_semantics=output_semantics),
        )

    def snapshot(self) -> Dict[str, object]:
        return {
            "available": ["alias", "ranges", "prompt", "renderers"],
        }
