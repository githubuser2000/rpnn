"""Legacy compatibility facade for Reta's prompt word completer.

Stage 40 moves the concrete completion matching logic into
``reta_architecture.completion_word``.  The historical import surface stays
unchanged: callers can still import ``WordCompleter`` from this module.
"""

from reta_architecture.completion_word import ArchitectureWordCompleter as WordCompleter

__all__ = ["WordCompleter"]
