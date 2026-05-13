> Hinweis: Diese Datei dokumentiert den Stage-42-Zustand. Der aktuelle Gesamtstand steht in `ARCHITECTURE_STATUS.md`.

# Stage 42 Changes – Explicit Architecture Progress Overlay

Stage 42 does not move runtime behaviour. The repository already performed real
activations in Stages 37 to 41, but the Stage-34 migration metadata still
reported all steps as merely planned. Stage 42 resolves that visibility gap and
also closes the last obvious mixed data owner from the old remaining-work list.

## New owners

```text
Stage-34 migration plan only
    ↓
reta_architecture/architecture_progress.py
  ArchitectureProgressBundle
  LegacySurfaceProgressSpec
  MigrationExecutionSpec
  WaveExecutionSpec
  OutstandingWorkItemSpec

libs/lib4tables_Enum.py data owner
    ↓
reta_architecture/tag_schema.py
  TagSchemaBundle
```

## Implemented

- Added `reta_architecture/architecture_progress.py`.
- Added `RetaArchitecture.bootstrap_architecture_progress(...)` and included the
  new bundle in the architecture snapshot.
- Added probe commands:
  - `architecture-progress-json`
  - `architecture-progress-md`
- Added `ARCHITECTURE_STATUS_STAGE42.md` as one consolidated current-status
  document.
- Extended `reta_architecture/architecture_map.py` with a Stage-42 stage step.
- Added `reta_architecture/tag_schema.py` as the explicit owner of `ST`,
  `tableTags`, and the reverse mappings.
- Turned `libs/lib4tables_Enum.py` into a compatibility re-export facade.
- Extended `reta_architecture/package_integrity.py` so both new source modules
  are part of the required package surface.

## Behaviour

No CLI/runtime behaviour change.

Stage 42 now answers, explicitly and mechanically:

- which planned migration owners are already active architecture owners,
- which legacy files are now only compatibility facades,
- which local-section/data/doc owners are intentionally retained,
- and what still remains genuinely open.

## Observed result

The Stage-42 overlay now reports:

- M0 implemented/retained
- M1 implemented/retained
- M2 implemented/retained
- M3 implemented/retained
- M4 implemented/retained
- M5 implemented/retained
- M6 implemented/retained

There is no longer a mixed owner in `libs/lib4tables_Enum.py`; the remaining
open item is environment-specific command parity because the reference archive
is not available in this runtime.
