"""Legacy compatibility facade for Reta's nested prompt completer.

Stage 41 moves the concrete hierarchical completion state machine into
``reta_architecture.completion_nested``.  The historical import surface stays
unchanged: callers can still import ``NestedCompleter`` and ``ComplSitua`` from
this module.
"""

from reta_architecture.completion_nested import (
    ArchitectureNestedCompleter as NestedCompleter,
    ArchitectureNestedCompleter,
    ComplSitua,
    NestedCompletionMorphismBundle,
    NestedCompletionRuntimeView,
    NestedDict,
    WordCompleter,
    bootstrap_nested_completion_morphisms,
    hundert,
    hundert2,
)

NESTED_COMPLETION_MORPHISMS = bootstrap_nested_completion_morphisms()

__all__ = ["NestedCompleter", "ComplSitua"]
