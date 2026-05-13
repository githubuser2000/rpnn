from __future__ import annotations

"""Stage-39 activated console/help/wrapping morphisms for Reta's center facade.

Historically ``libs/center.py`` owned a mixed cluster of small runtime helpers:
help-file rendering, terminal-width discovery, debug printing, CLI output,
chunking, ordered default dictionaries and order-preserving uniqueness.  Stage
39 moves that concrete logic into the architecture package while ``center.py``
keeps the historical function names as wrappers.

Mathematical reading:

* a help Markdown file is a local documentation section;
* terminal width and hyphenator state form an output-context open set;
* CLI printing is a morphism from rendered text sections to console effects;
* ordered uniqueness and chunking are finite-section morphisms used by table and
  generated-relation code;
* the old ``center`` API and the new architecture API are connected by a
  natural transformation: both routes must produce the same visible text/data
  sections.
"""

import os
import pprint
import re
import sys
from collections import OrderedDict
from collections.abc import Callable as _CallableABC
from itertools import filterfalse
from pathlib import Path
from typing import Callable, Iterable, Iterator, Optional, Sequence

try:
    from rich.console import Console as _RichConsole
    from rich.markdown import Markdown as _RichMarkdown
    from rich.syntax import Syntax as _RichSyntax
except (ModuleNotFoundError, ImportError):
    class _RichConsole:
        def __init__(self, *args, **kwargs):
            pass

        def print(self, *args, **kwargs):
            end = kwargs.get("end", "\n")
            text = " ".join(str(arg) for arg in args)
            __import__("builtins").print(text, end=end)

    class _RichMarkdown:
        def __init__(self, text, *args, **kwargs):
            self.text = text

        def __str__(self):
            return self.text

    class _RichSyntax:
        def __init__(self, text, *args, **kwargs):
            self.text = text

        def __str__(self):
            return self.text


Console = _RichConsole
Markdown = _RichMarkdown
Syntax = _RichSyntax


class DefaultOrderedDict(OrderedDict):
    """OrderedDict with a default factory, kept byte-compatible with center.py use."""

    # Source: http://stackoverflow.com/a/6190500/562769
    def __init__(self, default_factory=None, *a, **kw):
        if default_factory is not None and not isinstance(default_factory, _CallableABC):
            raise TypeError("first argument must be callable")
        OrderedDict.__init__(self, *a, **kw)
        self.default_factory = default_factory

    def __getitem__(self, key):
        try:
            return OrderedDict.__getitem__(self, key)
        except KeyError:
            return self.__missing__(key)

    def __missing__(self, key):
        if self.default_factory is None:
            raise KeyError(key)
        self[key] = value = self.default_factory()
        return value

    def __reduce__(self):
        if self.default_factory is None:
            args = tuple()
        else:
            args = (self.default_factory,)
        return type(self), args, None, None, self.items()

    def copy(self):
        return self.__copy__()

    def __copy__(self):
        return type(self)(self.default_factory, self)

    def __deepcopy__(self, memo):
        import copy

        return type(self)(self.default_factory, copy.deepcopy(self.items()))

    def __repr__(self):
        return "OrderedDefaultDict(%s, %s)" % (
            self.default_factory,
            OrderedDict.__repr__(self),
        )


def chunks(sequence: Sequence, size: int) -> Iterator[Sequence]:
    """Yield successive ``size``-sized chunks, preserving legacy helper semantics."""
    for index in range(0, len(sequence), size):
        yield sequence[index : index + size]


def unique_everseen(iterable: Iterable, key: Optional[Callable] = None, ordered_set_factory=None):
    """Yield unique elements while preserving order.

    This mirrors the historical ``center.unique_everseen`` implementation.  The
    ordered set factory is injectable so ``center.py`` can preserve its optional
    dependency on ``orderedset.OrderedSet`` while the architecture module stays
    usable without that package.
    """
    ordered_set_factory = ordered_set_factory or set
    seen = ordered_set_factory()
    seen_add = seen.add
    if key is None:
        for element in filterfalse(seen.__contains__, iterable):
            seen_add(element)
            yield element
    else:
        for element in iterable:
            marker = key(element)
            if marker not in seen:
                seen_add(marker)
                yield element


def cli_output(text, color: bool = False, stype: str = "", output_enabled: bool = True) -> None:
    """Print text like the legacy ``center.cliout`` helper."""
    if not output_enabled:
        return
    if color and len(text) > 0:
        text = " ".join(text.split())
        console = Console(width=len(text))
        console.print(
            Syntax(text.strip(), stype, word_wrap=True, indent_guides=True), end=""
        )
    else:
        print(text)


def debug_pair(text1, text, info_log: bool, output_enabled: bool = True, pretty_printer=None) -> None:
    """Print a labelled debug value like legacy ``center.x``."""
    if not (info_log and output_enabled):
        return
    pretty_printer = pretty_printer or pprint.PrettyPrinter(indent=4)
    if type(text) is str:
        print(str(text1) + ": " + text)
    else:
        print(str(text1) + ": ", end="")
        pretty_printer.pprint(text)


def debug_value(text, info_log: bool, output_enabled: bool = True, pretty_printer=None) -> None:
    """Print an unlabelled debug value like legacy ``center.alxp``."""
    if not (info_log and output_enabled):
        return
    pretty_printer = pretty_printer or pprint.PrettyPrinter(indent=4)
    if type(text) is str:
        print(text)
    else:
        pretty_printer.pprint(text)


def _doc_path(repo_root: Path, readme_filename: str) -> Path:
    return Path(repo_root) / "doc" / os.path.basename(readme_filename)


def reta_prompt_help_text(repo_root: Path, i18n) -> str:
    """Return the Markdown body printed by legacy ``retaPromptHilfe``."""
    place = _doc_path(Path(repo_root), i18n.readMeFileNames.retaPrompt)
    markdown_text = place.read_text(encoding="utf-8")
    start = markdown_text.find("+++", 2)
    markdown_text = re.sub(r"{#.*}", "", markdown_text)
    return markdown_text[start + 3 :]


def print_reta_prompt_help(repo_root: Path, i18n, console_factory=Console) -> None:
    """Render the retaPrompt help section to the console."""
    console = console_factory()
    console.print(Markdown(reta_prompt_help_text(repo_root, i18n)))


def reta_help_text(repo_root: Path, i18n) -> str:
    """Return the full reta help Markdown printed by legacy ``retaHilfe``."""
    place = _doc_path(Path(repo_root), i18n.readMeFileNames.reta)
    return place.read_text(encoding="utf-8")


def print_reta_help(repo_root: Path, i18n) -> None:
    """Print the reta help Markdown like legacy ``retaHilfe``."""
    print(reta_help_text(repo_root, i18n))


def get_text_wrap_things(max_len=None) -> tuple:
    """Return terminal wrapping helpers like legacy ``center.getTextWrapThings``."""
    if "Brython" not in sys.version.split():
        try:
            import html2text  # noqa: F401 - imported for historical side-effect/API parity
        except (ModuleNotFoundError, ImportError):
            html2text = None

        class _FallbackHyphenator:
            def wrap(self, text, width):
                if width <= 0:
                    return [text]
                return [text[i : i + width] for i in range(0, len(text), width)] or [""]

        try:
            import pyphen

            dic = pyphen.Pyphen(lang="de_DE")
        except (ModuleNotFoundError, ImportError):
            dic = _FallbackHyphenator()
        try:
            from textwrap2 import fill
        except (ModuleNotFoundError, ImportError):
            from textwrap import fill as _stdlib_fill

            def fill(text, width=70, **kwargs):
                kwargs.pop("use_hyphenator", None)
                return _stdlib_fill(text, width=width, **kwargs)

        h_de = None
        try:
            size = os.get_terminal_size()
            columns_amount, shell_rows_amount = size.columns, size.lines
        except OSError:
            try:
                if sys.stdin is not None and hasattr(sys.stdin, "isatty") and sys.stdin.isatty():
                    columns_amount, shell_rows_amount = os.popen("stty size", "r").read().split()
                else:
                    raise OSError
            except Exception:
                columns_amount, shell_rows_amount = "80", "80"
    else:  # pragma: no cover - Brython compatibility path
        html2text = None
        pyphen = None
        Hyphenator = None
        fill = None
        dic = None
        h_de = None
        columns_amount = "80"
        shell_rows_amount = "80"

    shell_width = int(columns_amount)
    return shell_width, h_de, dic, fill


class ConsoleIOMorphismBundle:
    """Activated architecture owner for center-level console and utility helpers."""

    def __init__(self, repo_root: str | Path | None = None, legacy_owner: str = "libs.center", activated_stage: int = 39):
        self.repo_root = Path(repo_root).resolve() if repo_root is not None else Path(__file__).resolve().parent.parent
        self.legacy_owner = legacy_owner
        self.activated_stage = activated_stage
        self.console_cls = Console
        self.markdown_cls = Markdown
        self.syntax_cls = Syntax

    def chunks(self, sequence: Sequence, size: int):
        return chunks(sequence, size)

    def unique_everseen(self, iterable: Iterable, key: Optional[Callable] = None, ordered_set_factory=None):
        return unique_everseen(iterable, key=key, ordered_set_factory=ordered_set_factory)

    def cliout(self, text, color: bool = False, stype: str = "", output_enabled: bool = True) -> None:
        return cli_output(text, color=color, stype=stype, output_enabled=output_enabled)

    def debug_pair(self, text1, text, info_log: bool, output_enabled: bool = True, pretty_printer=None) -> None:
        return debug_pair(text1, text, info_log=info_log, output_enabled=output_enabled, pretty_printer=pretty_printer)

    def debug_value(self, text, info_log: bool, output_enabled: bool = True, pretty_printer=None) -> None:
        return debug_value(text, info_log=info_log, output_enabled=output_enabled, pretty_printer=pretty_printer)

    def reta_prompt_help_text(self, i18n) -> str:
        return reta_prompt_help_text(self.repo_root, i18n)

    def print_reta_prompt_help(self, i18n) -> None:
        return print_reta_prompt_help(self.repo_root, i18n)

    def reta_help_text(self, i18n) -> str:
        return reta_help_text(self.repo_root, i18n)

    def print_reta_help(self, i18n) -> None:
        return print_reta_help(self.repo_root, i18n)

    def text_wrap_runtime(self, max_len=None) -> tuple:
        return get_text_wrap_things(max_len=max_len)

    def default_ordered_dict_type(self):
        return DefaultOrderedDict

    def snapshot(self) -> dict:
        return {
            "class": "ConsoleIOMorphismBundle",
            "stage": self.activated_stage,
            "legacy_owner": self.legacy_owner,
            "capsule": "OutputRenderingCapsule",
            "secondary_capsule": "InputPromptCapsule",
            "category": "ActivatedConsoleIOCategory",
            "functor": "ConsoleIOActivationFunctor",
            "natural_transformation": "CenterConsoleIOToArchitectureTransformation",
            "repo_root": str(self.repo_root),
            "morphisms": [
                "reta_prompt_help_text",
                "print_reta_prompt_help",
                "reta_help_text",
                "print_reta_help",
                "get_text_wrap_things",
                "cli_output",
                "debug_pair",
                "debug_value",
                "chunks",
                "unique_everseen",
                "DefaultOrderedDict",
            ],
            "compatibility_names": [
                "retaPromptHilfe",
                "retaHilfe",
                "getTextWrapThings",
                "cliout",
                "x",
                "alxp",
                "chunks",
                "unique_everseen",
                "DefaultOrderedDict",
            ],
            "observable_invariant": "center console/help/utility wrappers and ConsoleIOMorphismBundle expose the same visible output and finite-section helper results",
        }


def bootstrap_console_io_morphisms(repo_root: str | Path | None = None) -> ConsoleIOMorphismBundle:
    return ConsoleIOMorphismBundle(repo_root=repo_root)
