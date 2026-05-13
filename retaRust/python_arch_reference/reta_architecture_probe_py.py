#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from __future__ import annotations

import json
import sys
from pathlib import Path


def _prepare_imports() -> Path:
    here = Path(__file__).resolve()
    candidates = [here.parent, here.parent.parent, Path.cwd()]
    for base in candidates:
        if (base / "i18n" / "words.py").exists():
            sys.path.insert(0, str(base))
            sys.path.insert(0, str(base / "libs"))
            return base
    raise SystemExit("Konnte reta-Repo nicht finden.")


REPO_ROOT = _prepare_imports()

from reta_architecture import RetaArchitecture  # noqa: E402
from reta_architecture.package_integrity import RepoManifest  # noqa: E402


ARCH = RetaArchitecture.bootstrap(REPO_ROOT)


def dump(value):
    print(json.dumps(value, ensure_ascii=False, separators=(",", ":")))


def help_text(program_name: str) -> str:
    return f"""{program_name} - Architektur-Inspektion für reta

Aufruf:
  {program_name} snapshot-json
  {program_name} schema-json
  {program_name} module-split-json
  {program_name} topology-json
  {program_name} inputs-json
  {program_name} word-completion-json
  {program_name} nested-completion-json
  {program_name} row-ranges-json
  {program_name} arithmetic-json
  {program_name} console-io-json
  {program_name} column-selection-json
  {program_name} parameter-runtime-json
  {program_name} program-workflow-json
  {program_name} table-generation-json
  {program_name} table-preparation-json
  {program_name} row-filtering-json
  {program_name} table-wrapping-json
  {program_name} table-state-json
  {program_name} number-theory-json
  {program_name} table-output-json
  {program_name} table-runtime-json
  {program_name} generated-columns-json
  {program_name} meta-columns-json
  {program_name} concat-csv-json
  {program_name} combi-join-json
  {program_name} prompt-runtime-json
  {program_name} prompt-session-json
  {program_name} prompt-execution-json
  {program_name} prompt-preparation-json
  {program_name} prompt-interaction-json
  {program_name} package-integrity-json
  {program_name} completion-runtime-json
  {program_name} output-syntax-json
  {program_name} output-json
  {program_name} prompt-language-json
  {program_name} presheaves-json
  {program_name} sheaves-json
  {program_name} morphisms-json
  {program_name} universal-json
  {program_name} category-theory-json
  {program_name} architecture-map-json
  {program_name} architecture-diagram-md
  {program_name} architecture-contracts-json
  {program_name} architecture-contracts-md
  {program_name} architecture-witnesses-json
  {program_name} architecture-witnesses-md
  {program_name} architecture-validation-json
  {program_name} architecture-validation-md
  {program_name} architecture-coherence-json
  {program_name} architecture-coherence-md
  {program_name} architecture-traces-json
  {program_name} architecture-traces-md
  {program_name} architecture-boundaries-json
  {program_name} architecture-boundaries-md
  {program_name} architecture-impact-json
  {program_name} architecture-impact-md
  {program_name} architecture-migration-json
  {program_name} architecture-migration-md
  {program_name} architecture-rehearsal-json
  {program_name} architecture-rehearsal-md
  {program_name} architecture-activation-json
  {program_name} architecture-activation-md
  {program_name} architecture-progress-json
  {program_name} architecture-progress-md
"""


def main(argv):
    program_name = Path(argv[0]).name if argv else "reta_architecture_probe_py.py"
    if len(argv) <= 1 or argv[1] in {"-h", "--help", "help"}:
        print(help_text(program_name))
        return 0
    cmd = argv[1]
    if cmd == "snapshot-json":
        dump(ARCH.snapshot())
        return 0
    if cmd == "schema-json":
        dump(ARCH.snapshot()["schema"])
        return 0
    if cmd == "module-split-json":
        dump(ARCH.snapshot()["schema"].get("schema_modules", {}))
        return 0
    if cmd == "topology-json":
        dump(ARCH.snapshot()["topology"])
        return 0
    if cmd == "inputs-json":
        dump(ARCH.snapshot()["inputs"])
        return 0
    if cmd == "word-completion-json":
        dump(ARCH.bootstrap_word_completion().snapshot())
        return 0
    if cmd == "nested-completion-json":
        dump(ARCH.bootstrap_nested_completion().snapshot())
        return 0
    if cmd == "row-ranges-json":
        dump(ARCH.bootstrap_row_ranges().snapshot())
        return 0
    if cmd == "arithmetic-json":
        dump(ARCH.bootstrap_arithmetic().snapshot())
        return 0
    if cmd == "console-io-json":
        dump(ARCH.bootstrap_console_io().snapshot())
        return 0
    if cmd == "column-selection-json":
        dump(ARCH.bootstrap_column_selection().snapshot())
        return 0
    if cmd == "parameter-runtime-json":
        dump(ARCH.bootstrap_parameter_runtime().snapshot())
        return 0
    if cmd == "program-workflow-json":
        dump(ARCH.bootstrap_program_workflow().snapshot())
        return 0
    if cmd == "table-generation-json":
        dump(ARCH.bootstrap_table_generation().snapshot())
        return 0
    if cmd == "table-preparation-json":
        dump(ARCH.bootstrap_table_preparation().snapshot())
        return 0
    if cmd == "row-filtering-json":
        dump(ARCH.bootstrap_row_filtering().snapshot())
        return 0
    if cmd == "table-wrapping-json":
        dump(ARCH.bootstrap_table_wrapping().snapshot())
        return 0
    if cmd == "table-state-json":
        dump(ARCH.bootstrap_table_state().snapshot())
        return 0
    if cmd == "number-theory-json":
        dump(ARCH.bootstrap_number_theory().snapshot())
        return 0
    if cmd == "table-output-json":
        dump(ARCH.bootstrap_table_output().snapshot())
        return 0
    if cmd == "table-runtime-json":
        dump(ARCH.bootstrap_table_runtime().snapshot())
        return 0
    if cmd == "generated-columns-json":
        dump(ARCH.bootstrap_generated_columns().snapshot())
        return 0
    if cmd == "meta-columns-json":
        dump(ARCH.bootstrap_meta_columns().snapshot())
        return 0
    if cmd == "concat-csv-json":
        dump(ARCH.bootstrap_concat_csv().snapshot())
        return 0
    if cmd == "combi-join-json":
        dump(ARCH.bootstrap_combi_join().snapshot())
        return 0
    if cmd == "prompt-runtime-json":
        dump(ARCH.bootstrap_prompt_runtime().snapshot())
        return 0
    if cmd == "completion-runtime-json":
        dump(ARCH.bootstrap_completion_runtime().snapshot())
        return 0
    if cmd == "prompt-session-json":
        dump(ARCH.bootstrap_prompt_session().snapshot())
        return 0
    if cmd == "prompt-execution-json":
        dump(ARCH.bootstrap_prompt_execution().snapshot())
        return 0
    if cmd == "prompt-preparation-json":
        dump(ARCH.bootstrap_prompt_preparation().snapshot())
        return 0
    if cmd == "prompt-interaction-json":
        dump(ARCH.bootstrap_prompt_interaction().snapshot())
        return 0
    if cmd == "package-integrity-json":
        dump(RepoManifest.from_tree(REPO_ROOT).snapshot())
        return 0
    if cmd == "output-syntax-json":
        dump(ARCH.bootstrap_output_syntax().snapshot())
        return 0
    if cmd == "output-json":
        dump(ARCH.snapshot()["output_semantics"])
        return 0
    if cmd == "prompt-language-json":
        dump(ARCH.bootstrap_prompt_language().snapshot())
        return 0
    if cmd == "presheaves-json":
        dump(ARCH.snapshot()["presheaves"])
        return 0
    if cmd == "sheaves-json":
        dump(ARCH.snapshot()["sheaves"])
        return 0
    if cmd == "morphisms-json":
        dump(ARCH.snapshot()["morphisms"])
        return 0
    if cmd == "universal-json":
        dump(ARCH.snapshot()["universal"])
        return 0
    if cmd == "category-theory-json":
        dump(ARCH.bootstrap_category_theory().snapshot())
        return 0
    if cmd == "architecture-map-json":
        dump(ARCH.bootstrap_architecture_map().snapshot())
        return 0
    if cmd == "architecture-contracts-json":
        dump(ARCH.bootstrap_architecture_contracts().snapshot())
        return 0
    if cmd == "architecture-witnesses-json":
        dump(ARCH.bootstrap_architecture_witnesses().snapshot())
        return 0
    if cmd == "architecture-validation-json":
        dump(ARCH.bootstrap_architecture_validation().snapshot())
        return 0
    if cmd == "architecture-coherence-json":
        dump(ARCH.bootstrap_architecture_coherence().snapshot())
        return 0
    if cmd == "architecture-traces-json":
        dump(ARCH.bootstrap_architecture_traces().snapshot())
        return 0
    if cmd == "architecture-boundaries-json":
        dump(ARCH.bootstrap_architecture_boundaries().snapshot())
        return 0
    if cmd == "architecture-impact-json":
        dump(ARCH.bootstrap_architecture_impact().snapshot())
        return 0
    if cmd == "architecture-migration-json":
        dump(ARCH.bootstrap_architecture_migration().snapshot())
        return 0
    if cmd == "architecture-rehearsal-json":
        dump(ARCH.bootstrap_architecture_rehearsal().snapshot())
        return 0
    if cmd == "architecture-activation-json":
        dump(ARCH.bootstrap_architecture_activation().snapshot())
        return 0
    if cmd == "architecture-progress-json":
        dump(ARCH.bootstrap_architecture_progress().snapshot())
        return 0
    if cmd == "architecture-validation-md":
        validation = ARCH.bootstrap_architecture_validation()
        print("# Reta Stage-40 Architektur-Validation")
        print()
        print("## Validierungsbaum")
        print()
        print("```text")
        print(validation.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(validation.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-coherence-md":
        coherence = ARCH.bootstrap_architecture_coherence()
        print("# Reta Stage-40 Architektur-Kohärenz")
        print()
        print("## Kohärenzbaum")
        print()
        print("```text")
        print(coherence.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(coherence.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-traces-md":
        traces = ARCH.bootstrap_architecture_traces()
        print("# Reta Stage-32 Architektur-Traces")
        print()
        print("## Trace-Baum")
        print()
        print("```text")
        print(traces.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(traces.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-boundaries-md":
        boundaries = ARCH.bootstrap_architecture_boundaries()
        print("# Reta Stage-32 Architektur-Grenzen")
        print()
        print("## Boundary-Baum")
        print()
        print("```text")
        print(boundaries.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(boundaries.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-impact-md":
        impact = ARCH.bootstrap_architecture_impact()
        print("# Reta Stage-33 Architektur-Impact")
        print()
        print("## Impact-Baum")
        print()
        print("```text")
        print(impact.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(impact.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-migration-md":
        migration = ARCH.bootstrap_architecture_migration()
        print("# Reta Stage-34 Architektur-Migration")
        print()
        print("## Migration-Plan-Baum")
        print()
        print("```text")
        print(migration.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(migration.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-rehearsal-md":
        rehearsal = ARCH.bootstrap_architecture_rehearsal()
        print("# Reta Stage-35 Architektur-Rehearsal")
        print()
        print("## Rehearsal-/Readiness-Baum")
        print()
        print("```text")
        print(rehearsal.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(rehearsal.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-activation-md":
        activation = ARCH.bootstrap_architecture_activation()
        print("# Reta Stage-36 Architektur-Aktivierung")
        print()
        print("## Aktivierungs-/Commit-/Rollback-Baum")
        print()
        print("```text")
        print(activation.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(activation.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-progress-md":
        progress = ARCH.bootstrap_architecture_progress()
        print("# Reta Stage-42 Architektur-Fortschritt")
        print()
        print("## Fortschrittsbaum")
        print()
        print("```text")
        print(progress.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(progress.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-witnesses-md":
        witnesses = ARCH.bootstrap_architecture_witnesses()
        print("# Reta Stage-30 Architektur-Witnesses")
        print()
        print("## Witness-Baum")
        print()
        print("```text")
        print(witnesses.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(witnesses.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-contracts-md":
        contracts = ARCH.bootstrap_architecture_contracts()
        print("# Reta Stage-29 Architekturverträge")
        print()
        print("## Vertragsbaum")
        print()
        print("```text")
        print(contracts.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(contracts.mermaid_diagram.rstrip())
        return 0
    if cmd == "architecture-diagram-md":
        architecture_map = ARCH.bootstrap_architecture_map()
        print("# Reta Stage-42 Gesamtarchitektur")
        print()
        print("## Kapselbaum")
        print()
        print("```text")
        print(architecture_map.text_diagram.rstrip())
        print("```")
        print()
        print("## Mermaid")
        print()
        print(architecture_map.mermaid_diagram.rstrip())
        return 0
    raise SystemExit(f"Unbekannter Befehl: {cmd}")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
