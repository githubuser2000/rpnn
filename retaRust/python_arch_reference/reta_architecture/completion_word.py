from __future__ import annotations

"""Stage-40 activated word-completion morphisms for Reta's prompt facade.

Historically ``libs/word_completerAlx.py`` owned a patched copy of a
``prompt_toolkit`` word completer.  Stage 40 moves that concrete matching logic
into the architecture package while the legacy module keeps the historical
``WordCompleter`` name as a compatibility facade.

Mathematical reading:

* the list/callable of completion words is a local completion section;
* the document prefix before the cursor is an input-open-set restriction;
* prefix/middle matching is a morphism from raw prompt text to completion
  candidates;
* yielded ``Completion`` objects form the local output section for the prompt;
* the old ``word_completerAlx.WordCompleter`` route and the new architecture
  route are connected by a natural transformation: both routes must yield the
  same completions for the same document context.
"""

from dataclasses import dataclass
from typing import Callable, Iterable, List, Mapping, Optional, Pattern, Sequence, Union

try:
    from prompt_toolkit.completion import CompleteEvent, Completer, Completion
    from prompt_toolkit.document import Document
    from prompt_toolkit.formatted_text import AnyFormattedText
except (ModuleNotFoundError, ImportError):
    class CompleteEvent:  # minimal fallback for -S architecture probes
        pass

    class Completer:
        def get_completions(self, document, complete_event):
            return ()

    class Completion:
        def __init__(self, text, start_position=0, display=None, display_meta=""):
            self.text = text
            self.start_position = start_position
            self.display = display if display is not None else text
            self.display_meta = display_meta

        def __repr__(self):
            return f"Completion({self.text!r}, start_position={self.start_position!r})"

    class Document:
        def __init__(self, text="", cursor_position=None):
            self.text = text
            self.cursor_position = len(text) if cursor_position is None else cursor_position

        @property
        def text_before_cursor(self):
            return self.text[: self.cursor_position]

        def get_word_before_cursor(self, WORD=False, pattern=None):
            before = self.text_before_cursor
            if pattern is not None:
                matches = list(pattern.finditer(before))
                return matches[-1].group(0) if matches else ""
            if WORD:
                parts = before.split()
                return parts[-1] if parts else ""
            index = len(before)
            while index > 0 and (before[index - 1].isalnum() or before[index - 1] == "_"):
                index -= 1
            return before[index:]

    AnyFormattedText = object


WordsSource = Union[List[str], Callable[[], List[str]]]


def resolve_words(words: WordsSource) -> List[str]:
    """Resolve a static or callable completion-word section."""
    if callable(words):
        return list(words())
    return list(words)


def word_before_cursor(document: Document, *, WORD: bool = False, sentence: bool = False, pattern: Optional[Pattern[str]] = None) -> str:
    """Return the prefix section inspected by the word-completion morphism."""
    if sentence:
        return document.text_before_cursor
    return document.get_word_before_cursor(WORD=WORD, pattern=pattern)


def word_completion_matches(word: str, prefix: str, *, ignore_case: bool = False, match_middle: bool = False) -> bool:
    """Return whether one completion word matches the current prefix.

    The implementation intentionally preserves the historical
    ``libs/word_completerAlx.py`` semantics, including the slightly unusual
    middle-match slice ``prefix[:len(word)] in word``.
    """
    candidate = word.lower() if ignore_case else word
    probe = prefix.lower() if ignore_case else prefix
    if match_middle:
        return probe[: len(candidate)] in candidate
    return candidate.startswith(probe[: len(candidate)])


def iter_word_completions(
    words: WordsSource,
    document: Document,
    *,
    ignore_case: bool = False,
    display_dict: Optional[Mapping[str, AnyFormattedText]] = None,
    meta_dict: Optional[Mapping[str, AnyFormattedText]] = None,
    WORD: bool = False,
    sentence: bool = False,
    match_middle: bool = False,
    pattern: Optional[Pattern[str]] = None,
) -> Iterable[Completion]:
    """Yield prompt-toolkit completions like the legacy ``WordCompleter``."""
    display_dict = display_dict or {}
    meta_dict = meta_dict or {}
    prefix = word_before_cursor(document, WORD=WORD, sentence=sentence, pattern=pattern)
    for word in resolve_words(words):
        if word_completion_matches(word, prefix, ignore_case=ignore_case, match_middle=match_middle):
            yield Completion(
                word,
                -len(prefix),
                display=display_dict.get(word, word),
                display_meta=meta_dict.get(word, ""),
            )


class ArchitectureWordCompleter(Completer):
    """Architecture-owned implementation behind ``word_completerAlx.WordCompleter``."""

    def __init__(
        self,
        words: WordsSource,
        ignore_case: bool = False,
        display_dict: Optional[Mapping[str, AnyFormattedText]] = None,
        meta_dict: Optional[Mapping[str, AnyFormattedText]] = None,
        WORD: bool = False,
        sentence: bool = False,
        match_middle: bool = False,
        pattern: Optional[Pattern[str]] = None,
    ) -> None:
        assert not (WORD and sentence)
        self.words = words
        self.ignore_case = ignore_case
        self.display_dict = display_dict or {}
        self.meta_dict = meta_dict or {}
        self.WORD = WORD
        self.sentence = sentence
        self.match_middle = match_middle
        self.pattern = pattern

    def __eq__(self, obj) -> bool:
        return self.words == obj.words

    def get_completions(self, document: Document, complete_event: CompleteEvent) -> Iterable[Completion]:
        yield from iter_word_completions(
            self.words,
            document,
            ignore_case=self.ignore_case,
            display_dict=self.display_dict,
            meta_dict=self.meta_dict,
            WORD=self.WORD,
            sentence=self.sentence,
            match_middle=self.match_middle,
            pattern=self.pattern,
        )


@dataclass(frozen=True)
class WordCompletionMorphismBundle:
    """Activated architecture owner for legacy word-completion matching."""

    legacy_owner: str = "libs.word_completerAlx"
    activated_stage: int = 40

    def create_completer(
        self,
        words: WordsSource,
        ignore_case: bool = False,
        display_dict: Optional[Mapping[str, AnyFormattedText]] = None,
        meta_dict: Optional[Mapping[str, AnyFormattedText]] = None,
        WORD: bool = False,
        sentence: bool = False,
        match_middle: bool = False,
        pattern: Optional[Pattern[str]] = None,
    ) -> ArchitectureWordCompleter:
        return ArchitectureWordCompleter(
            words,
            ignore_case=ignore_case,
            display_dict=display_dict,
            meta_dict=meta_dict,
            WORD=WORD,
            sentence=sentence,
            match_middle=match_middle,
            pattern=pattern,
        )

    def resolve_words(self, words: WordsSource) -> List[str]:
        return resolve_words(words)

    def word_before_cursor(self, document: Document, *, WORD: bool = False, sentence: bool = False, pattern: Optional[Pattern[str]] = None) -> str:
        return word_before_cursor(document, WORD=WORD, sentence=sentence, pattern=pattern)

    def matches(self, word: str, prefix: str, *, ignore_case: bool = False, match_middle: bool = False) -> bool:
        return word_completion_matches(word, prefix, ignore_case=ignore_case, match_middle=match_middle)

    def completions(
        self,
        words: WordsSource,
        document: Document,
        *,
        ignore_case: bool = False,
        display_dict: Optional[Mapping[str, AnyFormattedText]] = None,
        meta_dict: Optional[Mapping[str, AnyFormattedText]] = None,
        WORD: bool = False,
        sentence: bool = False,
        match_middle: bool = False,
        pattern: Optional[Pattern[str]] = None,
    ) -> Iterable[Completion]:
        return iter_word_completions(
            words,
            document,
            ignore_case=ignore_case,
            display_dict=display_dict,
            meta_dict=meta_dict,
            WORD=WORD,
            sentence=sentence,
            match_middle=match_middle,
            pattern=pattern,
        )

    def sample_completions(self, prefix: str = "re") -> list[str]:
        return [item.text for item in self.completions(["reta", "religion", "alpha"], Document(prefix))]

    def snapshot(self) -> dict:
        return {
            "class": "WordCompletionMorphismBundle",
            "stage": self.activated_stage,
            "legacy_owner": self.legacy_owner,
            "capsule": "InputPromptCapsule",
            "category": "ActivatedWordCompletionCategory",
            "functor": "WordCompletionActivationFunctor",
            "natural_transformation": "WordCompleterToArchitectureTransformation",
            "morphisms": [
                "resolve_words",
                "word_before_cursor",
                "word_completion_matches",
                "iter_word_completions",
                "create_completer",
            ],
            "compatibility_names": [
                "WordCompleter",
                "word_completerAlx.WordCompleter",
            ],
            "sample": {
                "prefix": "re",
                "texts": self.sample_completions("re"),
            },
        }


def bootstrap_word_completion_morphisms() -> WordCompletionMorphismBundle:
    """Return the Stage-40 activated word-completion bundle."""
    return WordCompletionMorphismBundle()
