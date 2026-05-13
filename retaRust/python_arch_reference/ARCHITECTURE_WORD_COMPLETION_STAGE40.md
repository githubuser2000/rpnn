# Stage 40 Word-Completion Architecture

## Kapsel

```text
InputPromptCapsule
└─ WordCompletionMorphismBundle
   ├─ resolve_words
   ├─ word_before_cursor
   ├─ word_completion_matches
   ├─ iter_word_completions
   └─ create_completer
```

## Legacy surface

```text
libs/word_completerAlx.py
└─ WordCompleter = ArchitectureWordCompleter
```

## Naturalität

The Stage-40 naturality condition is:

```text
legacy route:
  word_completerAlx.WordCompleter(words).get_completions(document, event)

architecture route:
  WordCompletionMorphismBundle.completions(words, document)

must produce the same completion candidate section.
```

## Why this is the right next activation

After Stage 37-39, `libs/center.py` is much thinner.  The next small legacy file
with concrete runtime behaviour is `libs/word_completerAlx.py`.  It is narrow,
well-contained, and directly belongs to the prompt/input capsule.  That makes it
a safe Stage-40 activation before larger `nestedAlx.py` or `retaPrompt.py`
extractions.
