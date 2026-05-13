from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Optional

from .input_semantics import InputBundle
from .row_ranges import RowRangeMorphismBundle, bootstrap_row_range_morphisms
from .arithmetic import ArithmeticMorphismBundle, bootstrap_arithmetic_morphisms
from .console_io import ConsoleIOMorphismBundle, bootstrap_console_io_morphisms
from .completion_word import WordCompletionMorphismBundle, bootstrap_word_completion_morphisms
from .completion_nested import NestedCompletionMorphismBundle, bootstrap_nested_completion_morphisms
from .column_selection import ColumnSelectionBundle, bootstrap_column_selection
from .output_semantics import RetaOutputSemantics, bootstrap_output_semantics
from .output_syntax import OutputSyntaxBundle, bootstrap_output_syntax
from .table_generation import bootstrap_table_generation
from .generated_columns import GeneratedColumnsBundle, bootstrap_generated_columns
from .meta_columns import MetaColumnsBundle, bootstrap_meta_columns
from .concat_csv import ConcatCsvBundle, bootstrap_concat_csv
from .combi_join import KombiJoinBundle, bootstrap_combi_join
from .table_preparation import TablePreparationBundle, bootstrap_table_preparation
from .table_wrapping import TableWrappingBundle, bootstrap_table_wrapping
from .table_state import TableStateBundle, bootstrap_table_state
from .parameter_runtime import ParameterRuntimeBundle, bootstrap_parameter_runtime
from .program_workflow import ProgramWorkflowBundle, bootstrap_program_workflow
from .number_theory import NumberTheoryBundle, bootstrap_number_theory
from .morphisms import MorphismBundle
from .presheaves import PresheafBundle
from .schema import RetaContextSchema
from .sheaves import SheafBundle
from .topology import ContextSelection, RetaContextTopology
from .universal import UniversalBundle
from .category_theory import CategoryTheoryBundle, bootstrap_category_theory
from .architecture_map import ArchitectureMapBundle, bootstrap_architecture_map
from .architecture_contracts import ArchitectureContractsBundle, bootstrap_architecture_contracts
from .architecture_witnesses import ArchitectureWitnessBundle, bootstrap_architecture_witnesses
from .architecture_validation import ArchitectureValidationBundle, bootstrap_architecture_validation
from .architecture_coherence import ArchitectureCoherenceBundle, bootstrap_architecture_coherence
from .architecture_traces import ArchitectureTraceBundle, bootstrap_architecture_traces
from .architecture_boundaries import ArchitectureBoundariesBundle, bootstrap_architecture_boundaries
from .architecture_impact import ArchitectureImpactBundle, bootstrap_architecture_impact
from .architecture_migration import ArchitectureMigrationBundle, bootstrap_architecture_migration
from .architecture_rehearsal import ArchitectureRehearsalBundle, bootstrap_architecture_rehearsal
from .architecture_activation import ArchitectureActivationBundle, bootstrap_architecture_activation
from .architecture_progress import ArchitectureProgressBundle, bootstrap_architecture_progress
from .parallel_execution import ParallelExecutionBundle, bootstrap_parallel_execution
from .execution_network import ExecutionNetworkBundle, bootstrap_execution_network
from .persistence import PersistenceBundle, bootstrap_persistence


_ARCHITECTURE_BOOTSTRAP_CACHE: dict[Path, "RetaArchitecture"] = {}


@dataclass
class RetaArchitecture:
    repo_root: Path
    schema: RetaContextSchema
    topology: RetaContextTopology
    inputs: InputBundle
    row_ranges: RowRangeMorphismBundle
    arithmetic: ArithmeticMorphismBundle
    console_io: ConsoleIOMorphismBundle
    word_completion: WordCompletionMorphismBundle
    nested_completion: NestedCompletionMorphismBundle
    output_syntax: OutputSyntaxBundle
    output_semantics: RetaOutputSemantics
    presheaves: PresheafBundle
    sheaves: SheafBundle
    morphisms: MorphismBundle
    universal: UniversalBundle
    category_theory: CategoryTheoryBundle
    architecture_map: ArchitectureMapBundle
    architecture_contracts: ArchitectureContractsBundle
    architecture_witnesses: ArchitectureWitnessBundle
    architecture_validation: ArchitectureValidationBundle
    architecture_coherence: ArchitectureCoherenceBundle
    architecture_traces: ArchitectureTraceBundle
    architecture_boundaries: ArchitectureBoundariesBundle
    architecture_impact: ArchitectureImpactBundle
    architecture_migration: ArchitectureMigrationBundle
    architecture_rehearsal: ArchitectureRehearsalBundle
    architecture_activation: ArchitectureActivationBundle
    architecture_progress: ArchitectureProgressBundle
    parallel_execution: ParallelExecutionBundle
    execution_network: ExecutionNetworkBundle
    persistence: PersistenceBundle
    column_selection: ColumnSelectionBundle
    parameter_runtime: ParameterRuntimeBundle
    program_workflow: ProgramWorkflowBundle
    table_preparation: TablePreparationBundle
    row_filtering: object
    table_wrapping: TableWrappingBundle
    table_state: TableStateBundle
    number_theory: NumberTheoryBundle
    table_output: TableOutputBundle
    table_runtime: object
    generated_columns: GeneratedColumnsBundle
    meta_columns: MetaColumnsBundle
    concat_csv: ConcatCsvBundle
    combi_join: KombiJoinBundle

    @classmethod
    def bootstrap(cls, repo_root: Optional[Path] = None, use_cache: bool = True) -> "RetaArchitecture":
        if repo_root is None:
            repo_root = Path(__file__).resolve().parent.parent
        repo_root = repo_root.resolve()
        if use_cache and repo_root in _ARCHITECTURE_BOOTSTRAP_CACHE:
            return _ARCHITECTURE_BOOTSTRAP_CACHE[repo_root]

        import i18n.words as words  # compatibility facade / module split metadata
        import i18n.words_context as words_context
        import i18n.words_matrix as words_matrix
        import i18n.words_runtime as words_runtime
        from .tag_schema import ST, tableTags

        schema = RetaContextSchema.from_words_parts(
            context_module=words_context,
            matrix_module=words_matrix,
            runtime_module=words_runtime,
            tag_enum=ST,
            table_tags=tableTags,
        )
        schema.schema_modules.setdefault("compatibility", str(getattr(words, "__name__", "i18n.words")))
        if hasattr(words, "MODULE_SPLIT"):
            schema.schema_modules.update(
                {f"compat:{key}": value for key, value in dict(words.MODULE_SPLIT).items()}
            )

        topology = RetaContextTopology.from_schema(schema)
        inputs = InputBundle.from_schema(schema, i18n=words_runtime)
        row_ranges = bootstrap_row_range_morphisms(inputs.row_ranges)
        arithmetic = bootstrap_arithmetic_morphisms(row_ranges, classify=words_runtime.classify)
        console_io = bootstrap_console_io_morphisms(repo_root)
        word_completion = bootstrap_word_completion_morphisms()
        nested_completion = bootstrap_nested_completion_morphisms(i18n=words_runtime, row_range_morphisms=row_ranges)
        output_syntax = bootstrap_output_syntax()
        output_semantics = bootstrap_output_semantics(repo_root)
        presheaves = PresheafBundle.discover(repo_root)
        sheaves = SheafBundle.from_repo(repo_root, schema)
        morphisms = MorphismBundle.from_topology_and_sheaves(topology, sheaves, output_semantics=output_semantics)
        universal = UniversalBundle(sheaves)
        category_theory = bootstrap_category_theory()
        architecture_map = bootstrap_architecture_map()
        architecture_contracts = bootstrap_architecture_contracts(category_theory=category_theory, architecture_map=architecture_map)
        architecture_witnesses = bootstrap_architecture_witnesses(
            repo_root=repo_root,
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
        )
        architecture_coherence = bootstrap_architecture_coherence(
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
            architecture_witnesses=architecture_witnesses,
        )
        architecture_traces = bootstrap_architecture_traces(
            repo_root=repo_root,
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
            architecture_witnesses=architecture_witnesses,
            architecture_coherence=architecture_coherence,
        )
        architecture_boundaries = bootstrap_architecture_boundaries(
            repo_root=repo_root,
            architecture_map=architecture_map,
            architecture_coherence=architecture_coherence,
        )
        architecture_impact = bootstrap_architecture_impact(
            repo_root=repo_root,
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
            architecture_witnesses=architecture_witnesses,
            architecture_coherence=architecture_coherence,
            architecture_traces=architecture_traces,
            architecture_boundaries=architecture_boundaries,
        )
        architecture_migration = bootstrap_architecture_migration(
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
            architecture_impact=architecture_impact,
        )
        architecture_rehearsal = bootstrap_architecture_rehearsal(
            category_theory=category_theory,
            architecture_contracts=architecture_contracts,
            architecture_impact=architecture_impact,
            architecture_migration=architecture_migration,
        )
        architecture_activation = bootstrap_architecture_activation(
            category_theory=category_theory,
            architecture_contracts=architecture_contracts,
            architecture_migration=architecture_migration,
            architecture_rehearsal=architecture_rehearsal,
        )
        architecture_progress = bootstrap_architecture_progress(
            repo_root=repo_root,
            architecture_migration=architecture_migration,
            architecture_activation=architecture_activation,
        )
        parallel_execution = bootstrap_parallel_execution()
        execution_network = bootstrap_execution_network()
        persistence = bootstrap_persistence(repo_root=repo_root)
        architecture_validation = bootstrap_architecture_validation(
            repo_root=repo_root,
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
            architecture_witnesses=architecture_witnesses,
            architecture_traces=architecture_traces,
            architecture_boundaries=architecture_boundaries,
            architecture_impact=architecture_impact,
            architecture_migration=architecture_migration,
            architecture_rehearsal=architecture_rehearsal,
            architecture_activation=architecture_activation,
            row_ranges=row_ranges,
            arithmetic=arithmetic,
            console_io=console_io,
            word_completion=word_completion,
            nested_completion=nested_completion,
        )
        column_selection = bootstrap_column_selection()
        parameter_runtime = bootstrap_parameter_runtime()
        table_preparation = bootstrap_table_preparation()
        from .row_filtering import bootstrap_row_filtering
        row_filtering = bootstrap_row_filtering()
        table_wrapping = bootstrap_table_wrapping()
        table_state = bootstrap_table_state()
        number_theory = bootstrap_number_theory()
        from .table_output import bootstrap_table_output
        table_output = bootstrap_table_output()
        from .table_runtime import bootstrap_table_runtime
        table_runtime = bootstrap_table_runtime(output_semantics=output_semantics, table_state=table_state)
        generated_columns = bootstrap_generated_columns()
        meta_columns = bootstrap_meta_columns()
        concat_csv = bootstrap_concat_csv()
        combi_join = bootstrap_combi_join()
        program_workflow = bootstrap_program_workflow(
            repo_root=repo_root,
            i18n=words_runtime,
            csv_file_names=words_runtime.csvFileNames,
            gebrochen_spalten_maximum_plus1=words_runtime.gebrochenSpaltenMaximumPlus1,
        )
        architecture = cls(
            repo_root=repo_root,
            schema=schema,
            topology=topology,
            inputs=inputs,
            row_ranges=row_ranges,
            arithmetic=arithmetic,
            console_io=console_io,
            word_completion=word_completion,
            nested_completion=nested_completion,
            output_syntax=output_syntax,
            output_semantics=output_semantics,
            presheaves=presheaves,
            sheaves=sheaves,
            morphisms=morphisms,
            universal=universal,
            category_theory=category_theory,
            architecture_map=architecture_map,
            architecture_contracts=architecture_contracts,
            architecture_witnesses=architecture_witnesses,
            architecture_validation=architecture_validation,
            architecture_coherence=architecture_coherence,
            architecture_traces=architecture_traces,
            architecture_boundaries=architecture_boundaries,
            architecture_impact=architecture_impact,
            architecture_migration=architecture_migration,
            architecture_rehearsal=architecture_rehearsal,
            architecture_activation=architecture_activation,
            architecture_progress=architecture_progress,
            parallel_execution=parallel_execution,
            execution_network=execution_network,
            persistence=persistence,
            column_selection=column_selection,
            parameter_runtime=parameter_runtime,
            program_workflow=program_workflow,
            table_preparation=table_preparation,
            row_filtering=row_filtering,
            table_wrapping=table_wrapping,
            table_state=table_state,
            number_theory=number_theory,
            table_output=table_output,
            table_runtime=table_runtime,
            generated_columns=generated_columns,
            meta_columns=meta_columns,
            concat_csv=concat_csv,
            combi_join=combi_join,
        )
        if use_cache:
            _ARCHITECTURE_BOOTSTRAP_CACHE[repo_root] = architecture
        return architecture

    def sync_program_semantics(self, para_dict, data_dicts) -> None:
        self.sheaves.parameter_semantics.sync_program_semantics(para_dict, data_dicts)

    def sync_tables(self, tables, output_mode: Optional[str] = None, finally_display_lines=None, rows_range=None) -> None:
        resolved_output_mode = output_mode or self.output_semantics.mode_for_tables(tables)
        self.universal.sync_tables(
            tables,
            output_mode=resolved_output_mode,
            finally_display_lines=finally_display_lines,
            rows_range=rows_range,
        )

    def update_prompt_state(self, raw_text: str, tokens, context: Optional[ContextSelection] = None) -> None:
        self.presheaves.prompt_state.update(raw_text, tokens, context=context)



    def bootstrap_row_ranges(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.row_ranges
        return bootstrap_row_range_morphisms(self.inputs.row_ranges)

    def bootstrap_arithmetic(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.arithmetic
        import i18n.words_runtime as words_runtime
        return bootstrap_arithmetic_morphisms(self.bootstrap_row_ranges(force_rebuild=force_rebuild), classify=words_runtime.classify)

    def bootstrap_console_io(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.console_io
        return bootstrap_console_io_morphisms(self.repo_root)

    def bootstrap_word_completion(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.word_completion
        return bootstrap_word_completion_morphisms()

    def bootstrap_nested_completion(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.nested_completion
        import i18n.words_runtime as words_runtime
        return bootstrap_nested_completion_morphisms(i18n=words_runtime, row_range_morphisms=self.bootstrap_row_ranges())

    def bootstrap_column_selection(self, ordered_set_factory=None, force_rebuild: bool = False):
        if ordered_set_factory is None and not force_rebuild:
            return self.column_selection
        return bootstrap_column_selection(ordered_set_factory=ordered_set_factory)

    def bootstrap_parameter_runtime(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.parameter_runtime
        return bootstrap_parameter_runtime()

    def bootstrap_program_workflow(self, csv_file_names=None, force_rebuild: bool = False):
        if csv_file_names is None and not force_rebuild:
            return self.program_workflow
        import i18n.words_runtime as words_runtime

        if csv_file_names is None:
            csv_file_names = words_runtime.csvFileNames
        return bootstrap_program_workflow(
            repo_root=self.repo_root,
            i18n=words_runtime,
            csv_file_names=csv_file_names,
            gebrochen_spalten_maximum_plus1=words_runtime.gebrochenSpaltenMaximumPlus1,
        )

    def bootstrap_table_preparation(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.table_preparation
        return bootstrap_table_preparation()

    def bootstrap_row_filtering(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.row_filtering
        from .row_filtering import bootstrap_row_filtering
        return bootstrap_row_filtering()

    def bootstrap_table_state(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.table_state
        return bootstrap_table_state()

    def bootstrap_number_theory(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.number_theory
        return bootstrap_number_theory()

    def bootstrap_table_wrapping(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.table_wrapping
        return bootstrap_table_wrapping(force_refresh=True)

    def bootstrap_output_syntax(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.output_syntax
        return bootstrap_output_syntax()

    def bootstrap_table_output(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.table_output
        from .table_output import bootstrap_table_output
        return bootstrap_table_output()

    def bootstrap_table_runtime(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.table_runtime
        from .table_runtime import bootstrap_table_runtime
        return bootstrap_table_runtime(output_semantics=self.output_semantics, table_state=self.bootstrap_table_state())

    def bootstrap_generated_columns(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.generated_columns
        return bootstrap_generated_columns()

    def bootstrap_meta_columns(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.meta_columns
        return bootstrap_meta_columns()

    def bootstrap_concat_csv(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.concat_csv
        return bootstrap_concat_csv()

    def bootstrap_combi_join(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.combi_join
        return bootstrap_combi_join()

    def bootstrap_category_theory(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.category_theory
        return bootstrap_category_theory()

    def bootstrap_architecture_map(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_map
        return bootstrap_architecture_map()

    def bootstrap_architecture_contracts(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_contracts
        return bootstrap_architecture_contracts(
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
        )

    def bootstrap_architecture_witnesses(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_witnesses
        return bootstrap_architecture_witnesses(
            repo_root=self.repo_root,
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
        )

    def bootstrap_architecture_coherence(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_coherence
        return bootstrap_architecture_coherence(
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_witnesses=self.bootstrap_architecture_witnesses(),
        )

    def bootstrap_architecture_traces(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_traces
        return bootstrap_architecture_traces(
            repo_root=self.repo_root,
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_witnesses=self.bootstrap_architecture_witnesses(),
            architecture_coherence=self.bootstrap_architecture_coherence(),
        )

    def bootstrap_architecture_boundaries(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_boundaries
        return bootstrap_architecture_boundaries(
            repo_root=self.repo_root,
            architecture_map=self.bootstrap_architecture_map(),
            architecture_coherence=self.bootstrap_architecture_coherence(),
        )

    def bootstrap_architecture_impact(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_impact
        return bootstrap_architecture_impact(
            repo_root=self.repo_root,
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_witnesses=self.bootstrap_architecture_witnesses(),
            architecture_coherence=self.bootstrap_architecture_coherence(),
            architecture_traces=self.bootstrap_architecture_traces(),
            architecture_boundaries=self.bootstrap_architecture_boundaries(),
        )

    def bootstrap_architecture_migration(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_migration
        return bootstrap_architecture_migration(
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_impact=self.bootstrap_architecture_impact(),
        )

    def bootstrap_architecture_rehearsal(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_rehearsal
        return bootstrap_architecture_rehearsal(
            category_theory=self.bootstrap_category_theory(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_impact=self.bootstrap_architecture_impact(),
            architecture_migration=self.bootstrap_architecture_migration(),
        )


    def bootstrap_architecture_activation(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_activation
        return bootstrap_architecture_activation(
            category_theory=self.bootstrap_category_theory(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_migration=self.bootstrap_architecture_migration(),
            architecture_rehearsal=self.bootstrap_architecture_rehearsal(),
        )

    def bootstrap_architecture_progress(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_progress
        return bootstrap_architecture_progress(
            repo_root=self.repo_root,
            architecture_migration=self.bootstrap_architecture_migration(),
            architecture_activation=self.bootstrap_architecture_activation(),
        )

    def bootstrap_parallel_execution(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.parallel_execution
        return bootstrap_parallel_execution()

    def bootstrap_execution_network(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.execution_network
        return bootstrap_execution_network()

    def bootstrap_persistence(self, force_rebuild: bool = False, db_path: str | None = None):
        if not force_rebuild and db_path is None:
            return self.persistence
        return bootstrap_persistence(repo_root=self.repo_root, db_path=db_path)

    def bootstrap_architecture_validation(self, force_rebuild: bool = False):
        if not force_rebuild:
            return self.architecture_validation
        return bootstrap_architecture_validation(
            repo_root=self.repo_root,
            category_theory=self.bootstrap_category_theory(),
            architecture_map=self.bootstrap_architecture_map(),
            architecture_contracts=self.bootstrap_architecture_contracts(),
            architecture_witnesses=self.bootstrap_architecture_witnesses(),
            architecture_traces=self.bootstrap_architecture_traces(),
            architecture_boundaries=self.bootstrap_architecture_boundaries(),
            architecture_impact=self.bootstrap_architecture_impact(),
            architecture_migration=self.bootstrap_architecture_migration(),
            architecture_rehearsal=self.bootstrap_architecture_rehearsal(),
            architecture_activation=self.bootstrap_architecture_activation(),
            row_ranges=self.bootstrap_row_ranges(),
            arithmetic=self.bootstrap_arithmetic(),
            console_io=self.bootstrap_console_io(),
            word_completion=self.bootstrap_word_completion(),
            nested_completion=self.bootstrap_nested_completion(),
        )

    def bootstrap_table_generation(self, csv_file_names=None, force_rebuild: bool = False):
        if csv_file_names is None:
            import i18n.words_runtime as words_runtime

            csv_file_names = words_runtime.csvFileNames
        return bootstrap_table_generation(
            csv_file_names=csv_file_names,
            generated_columns=self.bootstrap_generated_columns(),
            concat_csv=self.bootstrap_concat_csv(),
            combi_join=self.bootstrap_combi_join(),
        )

    def bootstrap_prompt_runtime(self, i18n=None, force_rebuild: bool = False):
        from .prompt_runtime import bootstrap_prompt_runtime

        return bootstrap_prompt_runtime(
            repo_root=self.repo_root,
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
        )


    def bootstrap_completion_runtime(self, i18n=None, force_rebuild: bool = False):
        from .completion_runtime import bootstrap_completion_runtime

        return bootstrap_completion_runtime(
            repo_root=self.repo_root,
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
        )

    def bootstrap_prompt_language(self, i18n=None, force_rebuild: bool = False):
        from .prompt_language import bootstrap_prompt_language

        return bootstrap_prompt_language(
            repo_root=self.repo_root,
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
        )

    def bootstrap_prompt_session(self, i18n=None, force_rebuild: bool = False):
        from .prompt_session import bootstrap_prompt_session

        return bootstrap_prompt_session(
            repo_root=self.repo_root,
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
        )

    def bootstrap_prompt_execution(self, i18n=None, force_rebuild: bool = False):
        from .prompt_execution import bootstrap_prompt_execution

        return bootstrap_prompt_execution(
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
        )

    def bootstrap_prompt_preparation(self, i18n=None, force_rebuild: bool = False):
        from .prompt_preparation import bootstrap_prompt_preparation

        return bootstrap_prompt_preparation(
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
        )

    def bootstrap_prompt_interaction(self, i18n=None, force_rebuild: bool = False, **kwargs):
        from .prompt_interaction import bootstrap_prompt_interaction

        return bootstrap_prompt_interaction(
            architecture=self,
            i18n=i18n,
            force_rebuild=force_rebuild,
            **kwargs,
        )

    def snapshot(self):
        return {
            "schema": self.schema.snapshot(),
            "topology": self.topology.snapshot(),
            "inputs": self.inputs.snapshot(),
            "row_ranges": self.bootstrap_row_ranges().snapshot(),
            "arithmetic": self.bootstrap_arithmetic().snapshot(),
            "console_io": self.bootstrap_console_io().snapshot(),
            "word_completion": self.bootstrap_word_completion().snapshot(),
            "nested_completion": self.bootstrap_nested_completion().snapshot(),
            "output_syntax": self.bootstrap_output_syntax().snapshot(),
            "column_selection": self.bootstrap_column_selection().snapshot(),
            "parameter_runtime": self.bootstrap_parameter_runtime().snapshot(),
            "program_workflow": self.bootstrap_program_workflow().snapshot(),
            "table_generation": self.bootstrap_table_generation().snapshot(),
            "table_preparation": self.bootstrap_table_preparation().snapshot(),
            "row_filtering": self.bootstrap_row_filtering().snapshot(),
            "table_wrapping": self.bootstrap_table_wrapping().snapshot(),
            "table_state": self.bootstrap_table_state().snapshot(),
            "number_theory": self.bootstrap_number_theory().snapshot(),
            "table_output": self.bootstrap_table_output().snapshot(),
            "table_runtime": self.bootstrap_table_runtime().snapshot(),
            "generated_columns": self.bootstrap_generated_columns().snapshot(),
            "meta_columns": self.bootstrap_meta_columns().snapshot(),
            "concat_csv": self.bootstrap_concat_csv().snapshot(),
            "combi_join": self.bootstrap_combi_join().snapshot(),
            "output_semantics": self.output_semantics.snapshot(),
            "prompt_language": self.bootstrap_prompt_language().snapshot(),
            "prompt_session": self.bootstrap_prompt_session().snapshot(),
            "prompt_execution": self.bootstrap_prompt_execution().snapshot(),
            "prompt_preparation": self.bootstrap_prompt_preparation().snapshot(),
            "prompt_interaction": self.bootstrap_prompt_interaction().snapshot(),
            "presheaves": self.presheaves.snapshot(),
            "sheaves": self.sheaves.snapshot(),
            "morphisms": self.morphisms.snapshot(),
            "universal": self.universal.snapshot(),
            "category_theory": self.bootstrap_category_theory().snapshot(),
            "architecture_map": self.bootstrap_architecture_map().snapshot(),
            "architecture_contracts": self.bootstrap_architecture_contracts().snapshot(),
            "architecture_witnesses": self.bootstrap_architecture_witnesses().snapshot(),
            "architecture_validation": self.bootstrap_architecture_validation().snapshot(),
            "architecture_coherence": self.bootstrap_architecture_coherence().snapshot(),
            "architecture_traces": self.bootstrap_architecture_traces().snapshot(),
            "architecture_boundaries": self.bootstrap_architecture_boundaries().snapshot(),
            "architecture_impact": self.bootstrap_architecture_impact().snapshot(),
            "architecture_migration": self.bootstrap_architecture_migration().snapshot(),
            "architecture_rehearsal": self.bootstrap_architecture_rehearsal().snapshot(),
            "architecture_activation": self.bootstrap_architecture_activation().snapshot(),
            "architecture_progress": self.bootstrap_architecture_progress().snapshot(),
            "parallel_execution": self.bootstrap_parallel_execution().snapshot(),
        }
