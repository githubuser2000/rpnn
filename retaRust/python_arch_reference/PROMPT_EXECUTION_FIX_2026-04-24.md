# Prompt execution fix and PyPy3 parallel default

## PyPy3 parallel default

`ParallelExecutionConfig(mode='auto')` is the default.

In `auto` mode process parallelism is enabled by runtime policy on PyPy/PyPy3, but it is only actually used when all runtime guards pass:

- the interpreter is PyPy/PyPy3,
- more than one worker is available,
- the prepared row count is at least the threshold,
- the chunk size is positive.

Current defaults:

- `mode=auto`
- `workers=os.cpu_count()` unless overridden
- `chunk_size=64`
- `threshold=128`

That means: on PyPy3 the parallel process path is active by default for sufficiently large row-generation workloads. Small outputs stay serial to avoid multiprocessing overhead.

Disable explicitly:

```bash
pypy3 reta.py --no-parallel ...
```

Force explicitly:

```bash
pypy3 reta.py --parallel=processes --parallel-workers=4 --parallel-chunk-size=128 ...
```

## Fixed prompt regression

The prompt command path for entries such as `a1` could fail with:

```text
NameError: name 'isReTaParameter' is not defined
```

After adding the missing architecture import, the next uncovered dependency was:

```text
NameError: name 'befehle' is not defined
```

Both are now fixed in `reta_architecture/prompt_execution.py`:

- `isReTaParameter` is imported from `reta_architecture.prompt_language`.
- `befehle` is explicitly initialized and filled from `CompletionRuntimeBundle` during `bootstrap_prompt_execution(...)`.
- `bootstrap_prompt_interaction(...)` now forwards its already-built `completion_runtime` into prompt execution so this dependency is not rebuilt unnecessarily.

## Validation run in this environment

Executed:

```bash
/usr/bin/python3 -m py_compile \
  reta_architecture/prompt_execution.py \
  reta_architecture/prompt_interaction.py \
  retaPrompt.py \
  reta.py \
  reta_architecture/parallel_execution.py

/usr/bin/python3 - <<'PY'
import reta_architecture.prompt_execution as p
class DummyCompletion:
    befehle = ['a', 'absicht', 'q']
p.configure_prompt_execution(completion_runtime=DummyCompletion())
assert p.befehle == ['a', 'absicht', 'q']
assert callable(p.isReTaParameter)
print('prompt_execution smoke ok')
PY
```

A full interactive `rp` run could not be completed reliably in this non-interactive container because the prompt UI waits on a TTY-style input loop. The reported traceback path has nevertheless been patched directly at the missing globals that caused the failure.
