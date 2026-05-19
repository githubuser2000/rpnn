# reta parallel runtime environment variables

This document describes the environment variables consumed by
`src/shared/parallel_runtime.rs`.

The central policy is intentionally conservative.  Generator parallelism stays
off in `auto` mode because it was slower for common `-spalten --alles` runs on
some systems.  Output, width calculation and prompt-batch execution stay
parallel in `auto` mode, but now share one global worker budget.

## Accepted mode values

Boolean/parallel mode variables accept these values:

| Meaning | Values |
|---|---|
| off | `0`, `false`, `off`, `no`, `nein`, `serial`, `seriell` |
| on | `1`, `true`, `on`, `yes`, `ja`, `parallel` |
| auto | `auto`, empty string |

Invalid values are ignored and fall back to the default behavior.

Numeric variables must be positive integers.  `0`, negative values and invalid
values are ignored.

Important: `*_SERIAL` variables are presence flags.  If such a variable exists,
the corresponding area is forced serial, even if its value is `0`.

## Global variables

| Variable | Default | Meaning |
|---|---:|---|
| `RETA_PARALLEL` | `auto` | Global parallelism mode. `0` disables all areas. `1` enables all areas unless an area disables itself. `auto` uses the area defaults below. |
| `RETA_JOBS` | detected CPU parallelism, or `1` if unknown | Maximum total worker/job budget for reta parallel work. Highest-precedence job-count variable. |
| `RETA_THREADS` | same as `RETA_JOBS` fallback | Alias for `RETA_JOBS`, used only if `RETA_JOBS` is unset. |
| `RETA_NUM_THREADS` | same as `RETA_THREADS` fallback | Alias for `RETA_JOBS`, used only if `RETA_JOBS` and `RETA_THREADS` are unset. |
| `RETA_PARALLEL_MIN_ITEMS` | unset | Global minimum item count before a parallel path may run. Area-specific minima override it. If unset, each call site uses its own fallback. |
| `RETA_PARALLEL_MIN` | unset | Alias for `RETA_PARALLEL_MIN_ITEMS`, used only if `RETA_PARALLEL_MIN_ITEMS` is unset. |
| `RETA_PARALLEL_ALLOW_NESTED` | off | If on, allows nested parallel work. Default is off to avoid thread storms. |

Job-count precedence:

```text
RETA_JOBS > RETA_THREADS > RETA_NUM_THREADS > std::thread::available_parallelism() > 1
```

Minimum-item precedence:

```text
area-specific min > global min > call-site fallback min
```

## Area defaults in `RETA_PARALLEL=auto`

| Area | Default in auto | Reason |
|---|---:|---|
| Generators | off | String/allocation-heavy generator work was slower for `-spalten --alles` when enabled by default. |
| Output | on | Output rendering was already parallel before the central policy and is now budgeted globally. |
| Widths | on | Cell-width calculation was already parallel before the central policy and is now budgeted globally. |
| PromptBatch | on | Batch execution was already parallel before the central policy and is now budgeted globally. |

If `RETA_PARALLEL=1`, all areas are enabled unless disabled by their own area
mode or `*_SERIAL` flag.  If `RETA_PARALLEL=0`, all areas are disabled,
including areas explicitly set to on.

## Generator variables

Area prefix: `RETA_GENERATORS`

| Variable | Default | Meaning |
|---|---:|---|
| `RETA_GENERATORS_PARALLEL` | `auto` | Generator area mode. In global `auto`, this means off. Set to `1` to enable generator parallelism. |
| `RETA_GENERATORS_PARALLEL_ENABLED` | `auto` | Alias for `RETA_GENERATORS_PARALLEL`. |
| `RETA_GENERATORS_SERIAL` | unset | Presence flag. If set, forces generator work serial. |
| `RETA_GENERATORS_PARALLEL_MIN_ITEMS` | unset | Area minimum item count. Overrides global min. |
| `RETA_GENERATORS_PARALLEL_MIN` | unset | Alias for `RETA_GENERATORS_PARALLEL_MIN_ITEMS`. |

Call-site fallback minimum: `512` items.

Typical explicit test:

```bash
RETA_GENERATORS_PARALLEL=1 RETA_JOBS=2 target/debug/rreta -zeilen --alles -spalten --alles
```

## Output variables

Area prefix: `RETA_OUTPUT`

| Variable | Default | Meaning |
|---|---:|---|
| `RETA_OUTPUT_PARALLEL` | `auto` | Output area mode. In global `auto`, this means on. |
| `RETA_OUTPUT_PARALLEL_ENABLED` | `auto` | Alias for `RETA_OUTPUT_PARALLEL`. |
| `RETA_OUTPUT_SERIAL` | unset | Presence flag. If set, forces output rendering serial. |
| `RETA_OUTPUT_PARALLEL_MIN_ITEMS` | unset | Area minimum item count. Overrides global min. |
| `RETA_OUTPUT_PARALLEL_MIN` | unset | Alias for `RETA_OUTPUT_PARALLEL_MIN_ITEMS`. |

Call-site fallback minima depend on the operation:

| Operation | Call-site grain | Effective fallback minimum |
|---|---:|---:|
| selected row preparation | `4` rows/worker | `8` rows |
| structured row rendering | call-specific, usually `16` or `32` rows/worker | `32` or `64` rows |
| shell output chunk rendering | `16` rows/worker | `32` rows |

## Width variables

Primary area prefix: `RETA_WIDTH`

Accepted alias prefix: `RETA_WIDTHS`

| Variable | Default | Meaning |
|---|---:|---|
| `RETA_WIDTH_PARALLEL` | `auto` | Width-calculation area mode. In global `auto`, this means on. |
| `RETA_WIDTH_PARALLEL_ENABLED` | `auto` | Alias for `RETA_WIDTH_PARALLEL`. |
| `RETA_WIDTH_SERIAL` | unset | Presence flag. If set, forces width calculation serial. |
| `RETA_WIDTH_PARALLEL_MIN_ITEMS` | unset | Area minimum item count. Overrides global min. |
| `RETA_WIDTH_PARALLEL_MIN` | unset | Alias for `RETA_WIDTH_PARALLEL_MIN_ITEMS`. |
| `RETA_WIDTHS_PARALLEL` | `auto` | Alias-prefix variant. |
| `RETA_WIDTHS_PARALLEL_ENABLED` | `auto` | Alias-prefix variant. |
| `RETA_WIDTHS_SERIAL` | unset | Alias-prefix variant. |
| `RETA_WIDTHS_PARALLEL_MIN_ITEMS` | unset | Alias-prefix variant. |
| `RETA_WIDTHS_PARALLEL_MIN` | unset | Alias-prefix variant. |

Call-site fallback: `32` rows/worker, therefore effective fallback minimum is
`64` rows.

## Prompt-batch variables

Primary area prefix: `RETA_PROMPT`

Accepted alias prefix: `RETA_PROMPT_BATCH`

| Variable | Default | Meaning |
|---|---:|---|
| `RETA_PROMPT_PARALLEL` | `auto` | Prompt-batch area mode. In global `auto`, this means on. |
| `RETA_PROMPT_PARALLEL_ENABLED` | `auto` | Alias for `RETA_PROMPT_PARALLEL`. |
| `RETA_PROMPT_SERIAL` | unset | Presence flag. If set, forces prompt batch execution serial. |
| `RETA_PROMPT_PARALLEL_MIN_ITEMS` | unset | Area minimum item count. Overrides global min. |
| `RETA_PROMPT_PARALLEL_MIN` | unset | Alias for `RETA_PROMPT_PARALLEL_MIN_ITEMS`. |
| `RETA_PROMPT_BATCH_PARALLEL` | `auto` | Alias-prefix variant. |
| `RETA_PROMPT_BATCH_PARALLEL_ENABLED` | `auto` | Alias-prefix variant. |
| `RETA_PROMPT_BATCH_SERIAL` | unset | Alias-prefix variant. |
| `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS` | unset | Alias-prefix variant. |
| `RETA_PROMPT_BATCH_PARALLEL_MIN` | unset | Alias-prefix variant. |

Call-site fallback: `1` command/worker, therefore effective fallback minimum is
`2` commands.

## Useful examples

Fully serial:

```bash
RETA_PARALLEL=0 target/debug/rreta -zeilen --alles -spalten --alles
```

Allow global parallelism but limit workers:

```bash
RETA_PARALLEL=1 RETA_JOBS=2 target/debug/rreta -zeilen --alles -spalten --alles
```

## Final-output streaming variables

The CLI launcher now prefers the streaming ABI `reta_run_argv_stream` when the
loaded `libreta.so` exports it.  This path avoids the previous final memory
spike where all rendered lines were joined into one large `String`, copied into
a C string, copied back into the launcher, and only then written to stdout.

The streaming handoff uses bounded FIFO queues (`std::sync::mpsc::sync_channel`).
Those queues act as semaphores/back-pressure: if stdout or stderr is slower than
the producer, producers block instead of growing an unbounded output buffer.
Stdout and stderr use the same callback interface, so the engine-to-launcher
handoff is duplex; stdin remains part of the request.  For larger outputs the
work may be split into several ordered producer queues through the central
`RETA_OUTPUT` worker budget.  Each queue remains FIFO, and
the consumer drains queues in visible line order.  LIFO is intentionally not used
for visible output because CSV, HTML and shell output must stay byte-stable and
ordered.

| Variable | Default | Meaning |
|---|---:|---|
| `RETA_OUTPUT_QUEUE_CAPACITY` | `64` | Maximum queued line frames per producer queue before the producer blocks. |
| `RETA_OUTPUT_CHUNK_BYTES` | `65536` | Maximum normal chunk size passed from the library to the launcher callback. Very long single lines may be emitted as line bytes plus newline. |
| `RETA_OUTPUT_STREAM_MIN_LINES` | `256` | Minimum lines per worker for the final streaming handoff. |
| `RETA_OUTPUT_STREAM_MIN_ITEMS` | same as above | Compatibility alias used only if `RETA_OUTPUT_STREAM_MIN_LINES` is unset. |

Example with explicit bounded streaming and two global jobs:

```bash
RETA_JOBS=2 RETA_OUTPUT_QUEUE_CAPACITY=16 RETA_OUTPUT_CHUNK_BYTES=32768 target/debug/rreta -zeilen --alles -spalten --alles
```
