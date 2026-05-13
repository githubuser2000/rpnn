from __future__ import annotations

import ast
import gc
import io
import multiprocessing
import sys
import unittest
from collections import OrderedDict
from contextlib import redirect_stdout
from pathlib import Path
from types import SimpleNamespace


REPO_ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO_ROOT))
sys.path.insert(0, str(REPO_ROOT / "libs"))

import center  # noqa: E402
import LibRetaPrompt  # noqa: E402
import word_completerAlx  # noqa: E402
import nestedAlx  # noqa: E402
import i18n.words as words  # noqa: E402
import i18n.words_context as words_context  # noqa: E402
import i18n.words_matrix as words_matrix  # noqa: E402
import i18n.words_runtime as words_runtime  # noqa: E402
import reta  # noqa: E402
import lib4tables  # noqa: E402
from reta_architecture import (  # noqa: E402
    ParameterSemanticsBuilder,
    PromptModus,
    RetaArchitecture,
    bootstrap_prompt_language,
    bootstrap_prompt_session,
    custom_split,
    custom_split2,
    is15or16command,
    isReTaParameter,
    stextFromKleinKleinKleinBefehl,
    verifyBruchNganzZahlBetweenCommas,
    verkuerze_dict,
)
from reta_architecture.prompt_execution import PromptExecutionBundle  # noqa: E402
from reta_architecture.prompt_preparation import PromptPreparationBundle  # noqa: E402
from reta_architecture.prompt_interaction import PromptInteractionBundle  # noqa: E402
from reta_architecture.parameter_runtime import ParameterRuntimeBundle  # noqa: E402
from reta_architecture.program_workflow import ProgramWorkflowBundle  # noqa: E402
from reta_architecture.table_preparation import TablePreparationBundle  # noqa: E402
from reta_architecture.table_wrapping import TableWrappingBundle  # noqa: E402
from reta_architecture.table_state import TableStateBundle  # noqa: E402
from reta_architecture.category_theory import CategoryTheoryBundle  # noqa: E402
from reta_architecture.architecture_map import ArchitectureMapBundle  # noqa: E402
from reta_architecture.architecture_contracts import ArchitectureContractsBundle  # noqa: E402
from reta_architecture.architecture_witnesses import ArchitectureWitnessBundle  # noqa: E402
from reta_architecture.architecture_validation import ArchitectureValidationBundle  # noqa: E402
from reta_architecture.architecture_coherence import ArchitectureCoherenceBundle  # noqa: E402
from reta_architecture.architecture_traces import ArchitectureTraceBundle  # noqa: E402
from reta_architecture.architecture_boundaries import ArchitectureBoundariesBundle  # noqa: E402
from reta_architecture.architecture_impact import ArchitectureImpactBundle  # noqa: E402
from reta_architecture.architecture_migration import ArchitectureMigrationBundle  # noqa: E402
from reta_architecture.architecture_rehearsal import ArchitectureRehearsalBundle  # noqa: E402
from reta_architecture.architecture_activation import ArchitectureActivationBundle  # noqa: E402
from reta_architecture.architecture_progress import ArchitectureProgressBundle  # noqa: E402
from reta_architecture.parallel_execution import (  # noqa: E402
    ParallelExecutionBundle,
    ParallelExecutionConfig,
    RETA_PARALLEL_PROCESSOR_CORES,
    RETA_PROCESSOR_CORES,
    decode_kombi_rows_in_processes,
    decode_religion_rows_in_processes,
    extract_parallel_config_from_argv,
    max_cell_text_len_in_processes,
    prepare_kombi_join_tables_in_processes,
    select_columns_in_processes,
)
from reta_architecture.row_ranges import RowRangeMorphismBundle  # noqa: E402
from reta_architecture.arithmetic import ArithmeticMorphismBundle  # noqa: E402
from reta_architecture.console_io import ConsoleIOMorphismBundle  # noqa: E402
from reta_architecture.completion_word import WordCompletionMorphismBundle, ArchitectureWordCompleter  # noqa: E402
from reta_architecture.completion_nested import NestedCompletionMorphismBundle, ArchitectureNestedCompleter, ComplSitua as ArchitectureComplSitua  # noqa: E402
from reta_architecture.table_output import TableOutputBundle  # noqa: E402
from reta_architecture.generated_columns import GeneratedColumnsBundle  # noqa: E402
from reta_architecture.meta_columns import MetaColumnsBundle  # noqa: E402
from reta_architecture.concat_csv import ConcatCsvBundle  # noqa: E402
from reta_architecture.combi_join import KombiJoinBundle  # noqa: E402
from reta_architecture.row_filtering import RowFilteringBundle  # noqa: E402
from reta_architecture.output_syntax import OutputSyntaxBundle  # noqa: E402
from reta_architecture.number_theory import NumberTheoryBundle, primCreativity as arch_prim_creativity, moonNumber as arch_moon_number  # noqa: E402
from reta_architecture.package_integrity import RepoManifest  # noqa: E402
from reta_architecture.execution_network import (  # noqa: E402
    ExecutionNetworkBundle,
    ExecutionNetworkConfig,
    ExecutionTask,
    FifoTaskQueue,
    FullDuplexChannel,
    HalfDuplexChannel,
    LifoTaskStack,
    PriorityTaskQueue,
    execute_tasks_deterministically,
)
from reta_architecture.persistence import (  # noqa: E402
    PersistenceBundle,
    PersistenceConfig,
    bootstrap_persistence,
    cache_get,
    cache_put,
    invalidate_cache,
    load_section,
    load_sheaf_snapshot,
    persist_execution_run,
    persist_section,
    persist_sheaf_snapshot,
    query_audit_events,
    record_audit_event,
)
from reta_architecture.tag_schema import TagSchemaBundle, bootstrap_tag_schema  # noqa: E402


def _architecture_refactor_double_payload(value):
    return value * 2


class ArchitectureRefactorRegressionTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.architecture = RetaArchitecture.bootstrap(REPO_ROOT)

    def tearDown(self):
        gc.collect()

    def _build_program(self):
        return reta.Program(["reta.py"], runAlles=False)

    def _semantic_program(self):
        return self.architecture.bootstrap_prompt_runtime(i18n=center.i18n).program

    def test_schema_is_explicit(self):
        schema = self.architecture.schema
        self.assertGreater(len(schema.language_aliases), 0)
        self.assertGreater(len(schema.parameters_main), 0)
        self.assertGreater(len(schema.para_n_data_matrix), 0)

    def test_words_split_modules_are_visible(self):
        schema = self.architecture.schema
        self.assertEqual(schema.schema_modules["context"], "i18n.words_context")
        self.assertEqual(schema.schema_modules["matrix"], "i18n.words_matrix")
        self.assertEqual(schema.schema_modules["runtime"], "i18n.words_runtime")
        self.assertEqual(words.MODULE_SPLIT["context"], "i18n.words_context")
        self.assertEqual(len(words.paraNdataMatrix), len(words_matrix.paraNdataMatrix))
        self.assertEqual(words.ParametersMain.alles, words_context.ParametersMain.alles)
        self.assertEqual(words.csvFileNames.religion, words_runtime.csvFileNames.religion)

    def test_input_layer_is_explicit(self):
        snapshot = self.architecture.inputs.snapshot()
        self.assertEqual(snapshot["row_ranges"]["multiple_prefix"], center.i18n.befehle2["v"])
        self.assertTrue(snapshot["prompt_vocabulary_builder"]["available"])

    def test_prompt_runtime_layer_is_explicit(self):
        prompt_runtime = self.architecture.bootstrap_prompt_runtime(i18n=center.i18n, force_rebuild=True)
        snapshot = prompt_runtime.snapshot()
        self.assertEqual(snapshot["program_view"]["class"], "PromptProgramView")
        self.assertEqual(snapshot["program_view"]["paraDict_len"], 4155)
        self.assertEqual(snapshot["program_view"]["dataDict_sizes"], [555, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0])
        self.assertTrue(snapshot["validation"]["wahl15_valid"])
        self.assertEqual(prompt_runtime.program.mainParaCmds, LibRetaPrompt.retaProgram.mainParaCmds)
        self.assertEqual(prompt_runtime.vocabulary.snapshot(), LibRetaPrompt.promptVocabulary.snapshot())
        program = self._semantic_program()
        self.assertEqual(prompt_runtime.program.dataDict[0], program.dataDict[0])
        self.assertEqual(prompt_runtime.program.kombiReverseDict, program.kombiReverseDict)
        self.assertEqual(prompt_runtime.program.kombiReverseDict2, program.kombiReverseDict2)
        self.assertEqual(prompt_runtime.program.AllSimpleCommandSpalten, program.AllSimpleCommandSpalten)


    def test_prompt_session_layer_is_explicit(self):
        prompt_session = self.architecture.bootstrap_prompt_session(i18n=center.i18n, force_rebuild=True)
        snapshot = prompt_session.snapshot()
        self.assertEqual(snapshot["class"], "PromptSessionBundle")
        self.assertEqual(snapshot["prompt_runtime_class"], "PromptRuntimeBundle")
        self.assertEqual(snapshot["completion_runtime_class"], "CompletionRuntimeBundle")
        self.assertEqual(snapshot["prompt_language_class"], "PromptLanguageBundle")
        text_state = prompt_session.new_text_state("p5")
        self.assertEqual(type(text_state).__name__, "PromptTextState")
        self.assertEqual(text_state.liste, ["p5"])

    def test_prompt_execution_layer_is_explicit(self):
        prompt_execution = self.architecture.bootstrap_prompt_execution(i18n=center.i18n, force_rebuild=True)
        snapshot = prompt_execution.snapshot()
        self.assertIsInstance(prompt_execution, PromptExecutionBundle)
        self.assertEqual(snapshot["class"], "PromptExecutionBundle")
        self.assertEqual(snapshot["command_runner"], "PromptGrosseAusgabe")
        self.assertEqual(snapshot["fraction_manager"], "bruchBereichsManagementAndWbefehl")
        self.assertEqual(snapshot["reta_executor"], "retaExecuteNprint")

    def test_prompt_preparation_layer_is_explicit(self):
        prompt_preparation = self.architecture.bootstrap_prompt_preparation(i18n=center.i18n, force_rebuild=True)
        snapshot = prompt_preparation.snapshot()
        self.assertIsInstance(prompt_preparation, PromptPreparationBundle)
        self.assertEqual(snapshot["class"], "PromptPreparationBundle")
        self.assertEqual(snapshot["command_rotator"], "verdreheWoReTaBefehl")
        self.assertEqual(snapshot["regex_rewriter"], "regExReplace")
        self.assertEqual(snapshot["output_preparer"], "promptVorbereitungGrosseAusgabe")
        self.assertGreaterEqual(snapshot["beenden_commands_len"], 1)
        text_state = self.architecture.bootstrap_prompt_session(i18n=center.i18n).new_text_state("reta -spalten --religionen=sternpolygon")
        self.assertEqual(prompt_preparation.regex_replace(text_state), text_state.liste)

    def test_prompt_interaction_layer_is_explicit(self):
        prompt_interaction = self.architecture.bootstrap_prompt_interaction(i18n=center.i18n, force_rebuild=True)
        snapshot = prompt_interaction.snapshot()
        self.assertIsInstance(prompt_interaction, PromptInteractionBundle)
        self.assertEqual(snapshot["class"], "PromptInteractionBundle")
        self.assertEqual(snapshot["session_layer"], "PromptSessionBundle")
        self.assertEqual(snapshot["preparation_layer"], "PromptPreparationBundle")
        self.assertEqual(snapshot["execution_layer"], "PromptExecutionBundle")
        self.assertFalse(snapshot["has_nested_completer"])
        self.assertGreater(snapshot["befehle_len"], 100)

    def test_completion_runtime_layer_is_explicit(self):
        completion_runtime = self.architecture.bootstrap_completion_runtime(i18n=center.i18n, force_rebuild=True)
        snapshot = completion_runtime.snapshot()
        self.assertEqual(snapshot["befehle_len"], len(LibRetaPrompt.completionRuntime.befehle))
        self.assertEqual(
            sorted(snapshot["kombi_option_keys"]),
            ["*", center.i18n.kombiMainParas["galaxie"], center.i18n.kombiMainParas["universum"]],
        )
        self.assertIn("15_", completion_runtime.start_commands(include_numeric_shortcuts=True))
        self.assertEqual(completion_runtime.program.paraDict, LibRetaPrompt.completionRuntime.program.paraDict)

    def test_nested_completion_morphism_layer_is_activated(self):
        nested_completion = self.architecture.bootstrap_nested_completion(force_rebuild=True)
        snapshot = nested_completion.snapshot()
        self.assertIsInstance(nested_completion, NestedCompletionMorphismBundle)
        self.assertEqual(snapshot["stage"], 41)
        self.assertEqual(snapshot["category"], "ActivatedNestedCompletionCategory")
        self.assertEqual(snapshot["natural_transformation"], "NestedCompleterToArchitectureTransformation")
        self.assertIn("matchTextAlx", snapshot["morphisms"])
        self.assertIn("gleichKommaSpalten", snapshot["morphisms"])
        self.assertIs(nestedAlx.NestedCompleter, ArchitectureNestedCompleter)
        self.assertIs(nestedAlx.ComplSitua, ArchitectureComplSitua)
        completer = nested_completion.create_completer(
            {"reta": None},
            {},
            ArchitectureComplSitua.retaAnfang,
            "",
            {"reta": ArchitectureComplSitua.retaAnfang},
            completion_runtime=LibRetaPrompt.completionRuntime,
        )
        self.assertIsInstance(completer, ArchitectureNestedCompleter)
        nested_source = (REPO_ROOT / "libs" / "nestedAlx.py").read_text(encoding="utf-8")
        arch_source = (REPO_ROOT / "reta_architecture" / "completion_nested.py").read_text(encoding="utf-8")
        self.assertIn("NESTED_COMPLETION_MORPHISMS = bootstrap_nested_completion_morphisms", nested_source)
        self.assertIn("ArchitectureNestedCompleter as NestedCompleter", nested_source)
        self.assertIn("class ArchitectureNestedCompleter", arch_source)
        self.assertIn("class NestedCompletionMorphismBundle", arch_source)
        self.assertNotIn("class NestedCompleter(Completer)", nested_source)

    def test_prompt_language_layer_is_explicit(self):
        prompt_language = self.architecture.bootstrap_prompt_language(i18n=center.i18n, force_rebuild=True)
        snapshot = prompt_language.snapshot()
        self.assertEqual(snapshot["class"], "PromptLanguageBundle")
        self.assertEqual(snapshot["not_parameter_values_len"], len(LibRetaPrompt.notParameterValues))
        self.assertEqual(snapshot["gebrochen_erlaubte_zahlen_len"], len(LibRetaPrompt.gebrochenErlaubteZahlen))
        self.assertEqual(prompt_language.wahl15, LibRetaPrompt.wahl15)
        self.assertTrue(isReTaParameter("--religionen=sternpolygon"))
        self.assertTrue(is15or16command("15_"))
        self.assertEqual(custom_split("alpha (beta gamma) delta"), ["alpha", "(beta gamma)", "delta"])
        self.assertEqual(custom_split2("{1,2},3", ","), ["{1,2}", "3"])
        self.assertEqual(verkuerze_dict({"a": 1, "b": 1, "c": 2}), {"a": 1, "c": 2})
        self.assertEqual(
            verifyBruchNganzZahlBetweenCommas([], "", [], [], [], "5", []),
            ([True], [], [], ["5"], True),
        )
        if_kurz, expanded = stextFromKleinKleinKleinBefehl(PromptModus.normal, ["p5"], [])
        self.assertTrue(if_kurz)
        self.assertEqual(expanded, [center.i18n.befehle2["mulpri"], "5"])

    def test_center_uses_split_i18n_proxy(self):
        self.assertEqual(
            center.i18n.__source_modules__,
            ("i18n.words_context", "i18n.words_matrix", "i18n.words_runtime"),
        )
        self.assertEqual(center.ROW_RANGE_SYNTAX.multiple_prefix, center.i18n.befehle2["v"])
        self.assertEqual(center.BereichToNumbers2("1-3"), {1, 2, 3})
        self.assertEqual(center.BereichToNumbers2(f"{center.i18n.befehle2['v']}2-2", False, 7), {2, 4, 6})

    def test_row_range_morphism_layer_is_activated(self):
        row_ranges = self.architecture.bootstrap_row_ranges(force_rebuild=True)
        snapshot = row_ranges.snapshot()
        self.assertIsInstance(row_ranges, RowRangeMorphismBundle)
        self.assertEqual(snapshot["class"], "RowRangeMorphismBundle")
        self.assertEqual(snapshot["stage"], 37)
        self.assertEqual(snapshot["capsule"], "InputPromptCapsule")
        self.assertEqual(snapshot["category"], "ActivatedRowRangeCategory")
        self.assertEqual(snapshot["functor"], "RowRangeActivationFunctor")
        self.assertEqual(snapshot["natural_transformation"], "CenterRowRangeToArchitectureTransformation")
        self.assertIn("range_to_numbers", snapshot["morphisms"])
        self.assertIn("BereichToNumbers2", snapshot["compatibility_names"])
        self.assertEqual(row_ranges.syntax.multiple_prefix, center.i18n.befehle2["v"])
        samples = ["1", "1-3", "1-3+1", "-2,1-4", "{1,2,5}", "(1,2,5)", f"{center.i18n.befehle2['v']}2-2"]
        for sample in samples:
            with self.subTest(sample=sample):
                self.assertEqual(center.BereichToNumbers2(sample, False, 20), row_ranges.range_to_numbers(sample, False, 20))
                self.assertEqual(center.isZeilenAngabe(sample), row_ranges.is_row_range(sample))
        self.assertEqual(center.strAsGeneratorToListOfNumStrs("{1,2,5}"), {1, 2, 5})
        self.assertEqual(row_ranges.str_as_generator("(1,2,5)"), {1, 2, 5})
        center_source = (REPO_ROOT / "libs" / "center.py").read_text(encoding="utf-8")
        row_source = (REPO_ROOT / "reta_architecture" / "row_ranges.py").read_text(encoding="utf-8")
        self.assertIn("ROW_RANGE_MORPHISMS = bootstrap_row_range_morphisms", center_source)
        self.assertIn("return ROW_RANGE_MORPHISMS.range_to_numbers", center_source)
        self.assertIn("return ROW_RANGE_MORPHISMS.is_row_range", center_source)
        self.assertIn("def range_to_numbers", row_source)
        self.assertIn("class RowRangeMorphismBundle", row_source)



    def test_arithmetic_morphism_layer_is_activated(self):
        arithmetic = self.architecture.bootstrap_arithmetic(force_rebuild=True)
        snapshot = arithmetic.snapshot()
        self.assertIsInstance(arithmetic, ArithmeticMorphismBundle)
        self.assertEqual(snapshot["class"], "ArithmeticMorphismBundle")
        self.assertEqual(snapshot["stage"], 38)
        self.assertEqual(snapshot["capsule"], "InputPromptCapsule")
        self.assertEqual(snapshot["category"], "ActivatedArithmeticCategory")
        self.assertEqual(snapshot["functor"], "ArithmeticActivationFunctor")
        self.assertEqual(snapshot["natural_transformation"], "CenterArithmeticToArchitectureTransformation")
        self.assertEqual(snapshot["depends_on"]["row_range_stage"], 37)
        self.assertIn("factor_pairs", snapshot["morphisms"])
        self.assertIn("teiler", snapshot["compatibility_names"])
        self.assertEqual(set(center.multiples(12)), set(arithmetic.multiples(12)))
        self.assertEqual(set(center.multiples(12)), {(6, 2), (4, 3), (12, 1)})
        self.assertEqual(center.primfaktoren(24), arithmetic.prime_factors(24))
        self.assertEqual(center.primfaktoren(25, True), arithmetic.prime_factors(25, True))
        self.assertEqual(center.teiler("6"), arithmetic.divisors_for_range("6"))
        values_center = [2, 2, 2, 3]
        values_arch = [2, 2, 2, 3]
        self.assertEqual(center.primRepeat(values_center), arithmetic.prime_repeat(values_arch))
        self.assertEqual(values_center, values_arch)
        values_center = [2, 2, 2, 3]
        values_arch = [2, 2, 2, 3]
        self.assertEqual(center.primRepeat2(values_center), arithmetic.prime_repeat_pairs(values_arch))
        self.assertEqual(values_center, values_arch)
        self.assertEqual(center.textHatZiffer("abc3"), arithmetic.has_digit("abc3"))
        self.assertEqual(center.textHatZiffer("abc"), arithmetic.has_digit("abc"))
        self.assertEqual(center.invert_dict_B({"a": ["1", "2"], "b": ["2"]}), arithmetic.invert_dict({"a": ["1", "2"], "b": ["2"]}))
        center_source = (REPO_ROOT / "libs" / "center.py").read_text(encoding="utf-8")
        arithmetic_source = (REPO_ROOT / "reta_architecture" / "arithmetic.py").read_text(encoding="utf-8")
        self.assertIn("ARITHMETIC_MORPHISMS = bootstrap_arithmetic_morphisms", center_source)
        self.assertIn("return ARITHMETIC_MORPHISMS.multiples", center_source)
        self.assertIn("return ARITHMETIC_MORPHISMS.divisors_for_range", center_source)
        self.assertIn("class ArithmeticMorphismBundle", arithmetic_source)
        self.assertIn("def factor_pairs", arithmetic_source)

    def test_console_io_morphism_layer_is_activated(self):
        console_io = self.architecture.bootstrap_console_io(force_rebuild=True)
        snapshot = console_io.snapshot()
        self.assertIsInstance(console_io, ConsoleIOMorphismBundle)
        self.assertEqual(snapshot["class"], "ConsoleIOMorphismBundle")
        self.assertEqual(snapshot["stage"], 39)
        self.assertEqual(snapshot["capsule"], "OutputRenderingCapsule")
        self.assertEqual(snapshot["category"], "ActivatedConsoleIOCategory")
        self.assertEqual(snapshot["functor"], "ConsoleIOActivationFunctor")
        self.assertEqual(snapshot["natural_transformation"], "CenterConsoleIOToArchitectureTransformation")
        self.assertIn("cli_output", snapshot["morphisms"])
        self.assertIn("getTextWrapThings", snapshot["compatibility_names"])
        self.assertEqual(list(center.chunks([1, 2, 3, 4, 5], 2)), [[1, 2], [3, 4], [5]])
        self.assertEqual(list(center.unique_everseen("ABCA")), list(console_io.unique_everseen("ABCA")))
        self.assertEqual(list(center.unique_everseen("ABCA")), ["A", "B", "C"])
        legacy_width = center.getTextWrapThings()[0]
        arch_width = console_io.text_wrap_runtime()[0]
        self.assertIsInstance(legacy_width, int)
        self.assertIsInstance(arch_width, int)
        self.assertGreaterEqual(arch_width, 20)
        sample = {"a": [1, 2], "b": [2]}
        dod = center.DefaultOrderedDict(list)
        dod["x"].append(1)
        self.assertEqual(dod["x"], [1])
        center_source = (REPO_ROOT / "libs" / "center.py").read_text(encoding="utf-8")
        console_source = (REPO_ROOT / "reta_architecture" / "console_io.py").read_text(encoding="utf-8")
        self.assertIn("CONSOLE_IO_MORPHISMS = bootstrap_console_io_morphisms", center_source)
        self.assertIn("return CONSOLE_IO_MORPHISMS.chunks", center_source)
        self.assertIn("return CONSOLE_IO_MORPHISMS.unique_everseen", center_source)
        self.assertIn("class ConsoleIOMorphismBundle", console_source)
        self.assertIn("def cli_output", console_source)


    def test_word_completion_morphism_layer_is_activated(self):
        from reta_architecture.completion_word import Document

        word_completion = self.architecture.bootstrap_word_completion(force_rebuild=True)
        snapshot = word_completion.snapshot()
        self.assertIsInstance(word_completion, WordCompletionMorphismBundle)
        self.assertEqual(snapshot["class"], "WordCompletionMorphismBundle")
        self.assertEqual(snapshot["stage"], 40)
        self.assertEqual(snapshot["capsule"], "InputPromptCapsule")
        self.assertEqual(snapshot["category"], "ActivatedWordCompletionCategory")
        self.assertEqual(snapshot["functor"], "WordCompletionActivationFunctor")
        self.assertEqual(snapshot["natural_transformation"], "WordCompleterToArchitectureTransformation")
        self.assertIn("iter_word_completions", snapshot["morphisms"])
        self.assertIn("WordCompleter", snapshot["compatibility_names"])

        words = ["reta", "religion", "alpha"]
        legacy = word_completerAlx.WordCompleter(words)
        arch = word_completion.create_completer(words)
        self.assertIsInstance(legacy, ArchitectureWordCompleter)
        self.assertEqual(type(legacy), type(arch))
        legacy_texts = [item.text for item in legacy.get_completions(Document("re"), None)]
        arch_texts = [item.text for item in word_completion.completions(words, Document("re"))]
        self.assertEqual(legacy_texts, arch_texts)
        self.assertEqual(legacy_texts, ["reta", "religion"])

        middle_legacy = word_completerAlx.WordCompleter(["alpha", "beta", "theta"], match_middle=True)
        middle_texts = [item.text for item in middle_legacy.get_completions(Document("et"), None)]
        self.assertEqual(middle_texts, ["beta", "theta"])

        facade_source = (REPO_ROOT / "libs" / "word_completerAlx.py").read_text(encoding="utf-8")
        completion_source = (REPO_ROOT / "reta_architecture" / "completion_word.py").read_text(encoding="utf-8")
        self.assertIn("ArchitectureWordCompleter as WordCompleter", facade_source)
        self.assertNotIn("def get_completions", facade_source)
        self.assertIn("class ArchitectureWordCompleter", completion_source)
        self.assertIn("class WordCompletionMorphismBundle", completion_source)

    def test_prompt_vocabulary_matches_exported_globals(self):
        vocabulary = LibRetaPrompt.promptVocabulary
        self.assertEqual(LibRetaPrompt.mainParas, list(vocabulary.main_parameters))
        self.assertEqual(LibRetaPrompt.ausgabeParas, list(vocabulary.ausgabe_paras))
        self.assertEqual(LibRetaPrompt.kombiMainParas, list(vocabulary.kombi_main_paras))
        self.assertEqual(LibRetaPrompt.zeilenParas, list(vocabulary.zeilen_paras))
        self.assertEqual(LibRetaPrompt.hauptForNebenSet, set(vocabulary.haupt_for_neben_set))
        self.assertEqual(LibRetaPrompt.gebrochenErlaubteZahlen, set(vocabulary.gebrochen_erlaubte_zahlen))
        self.assertEqual(LibRetaPrompt.befehle2, set(vocabulary.befehle2))

    def test_libretaprompt_is_thin_compatibility_facade(self):
        source = (REPO_ROOT / "libs" / "LibRetaPrompt.py").read_text(encoding="utf-8")
        self.assertIn("bootstrap_prompt_runtime", source)
        self.assertIn("bootstrap_prompt_language", source)
        self.assertIn("bootstrap_prompt_session", source)
        self.assertNotIn("reta.Program([sys.argv[0]", source)
        self.assertNotIn("class PromptModus", source)
        self.assertNotIn("def custom_split", source)
        self.assertNotIn("def stextFromKleinKleinKleinBefehl", source)
        self.assertEqual(type(LibRetaPrompt.retaProgram).__name__, "PromptProgramView")
        self.assertEqual(type(LibRetaPrompt.promptSession).__name__, "PromptSessionBundle")
        self.assertNotIsInstance(LibRetaPrompt.retaProgram, reta.Program)


    def test_completion_stack_sources_use_explicit_completion_runtime(self):
        nested_source = (REPO_ROOT / "libs" / "nestedAlx.py").read_text(encoding="utf-8")
        reta_prompt_source = (REPO_ROOT / "retaPrompt.py").read_text(encoding="utf-8")
        nested_tree = ast.parse(nested_source)
        reta_prompt_tree = ast.parse(reta_prompt_source)

        def collect_imports(tree):
            imports = {}
            for node in tree.body:
                if isinstance(node, ast.ImportFrom):
                    imports.setdefault(node.module, set()).update(alias.name for alias in node.names)
            return imports

        nested_imports = collect_imports(nested_tree)
        reta_prompt_imports = collect_imports(reta_prompt_tree)

        self.assertEqual(set(nested_imports.get("LibRetaPrompt", set())), set())
        self.assertIn("reta_architecture.completion_nested", nested_imports)
        self.assertIn("ArchitectureNestedCompleter", set(nested_imports.get("reta_architecture.completion_nested", set())))
        self.assertEqual(set(nested_imports.get("reta_architecture", set())), set())
        self.assertEqual(
            set(reta_prompt_imports.get("nestedAlx", set())),
            {"ComplSitua", "NestedCompleter"},
        )
        self.assertEqual(
            set(reta_prompt_imports.get("reta_architecture", set())),
            {"PromptModus", "PromptTextState"},
        )
        self.assertEqual(
            set(reta_prompt_imports.get("reta_architecture.prompt_interaction", set())),
            {"PromptInteractionBundle", "bootstrap_prompt_interaction"},
        )
        self.assertIn("promptInteraction = bootstrap_prompt_interaction", reta_prompt_source)
        self.assertIn("promptInteraction.run_scope", reta_prompt_source)
        self.assertIn("promptInteraction.prompt_input", reta_prompt_source)
        self.assertIn("promptInteraction.store_prompt", reta_prompt_source)
        self.assertIn("promptInteraction.delete_before_storage_commands", reta_prompt_source)
        self.assertNotIn("promptSession.build_loop_setup", reta_prompt_source)
        self.assertNotIn("promptExecution.run_grosse_ausgabe", reta_prompt_source)
        self.assertNotIn("promptPreparation.prepare_grosse_ausgabe", reta_prompt_source)
        self.assertNotIn("def PromptGrosseAusgabe", reta_prompt_source)
        self.assertNotIn("def bruchBereichsManagementAndWbefehl", reta_prompt_source)
        self.assertNotIn("def regExReplace", reta_prompt_source)
        self.assertNotIn("def promptVorbereitungGrosseAusgabe", reta_prompt_source)
        self.assertNotIn("class TXT", reta_prompt_source)


    def test_column_selection_layer_is_explicit(self):
        column_selection = self.architecture.bootstrap_column_selection()
        snapshot = column_selection.snapshot()
        self.assertEqual(snapshot["class"], "ColumnSelectionBundle")
        self.assertEqual(snapshot["positive_bucket_count"], 12)
        self.assertEqual(snapshot["negative_bucket_count"], 12)
        self.assertEqual(column_selection.type_naming.ordinary, (0, 0))
        self.assertEqual(column_selection.type_naming.metakonkretNot, (1, 11))
        bucket_map = column_selection.new_bucket_map()
        self.assertEqual(sorted(bucket_map.keys()), [(neg, bucket) for neg in (0, 1) for bucket in range(12)])

    def test_generated_columns_layer_is_explicit(self):
        generated_columns = self.architecture.bootstrap_generated_columns(force_rebuild=True)
        snapshot = generated_columns.snapshot()
        self.assertIsInstance(generated_columns, GeneratedColumnsBundle)
        self.assertEqual(snapshot["class"], "GeneratedColumnsBundle")
        self.assertGreaterEqual(snapshot["count"], 9)
        names = [item["method_name"] for item in snapshot["morphisms"]]
        self.assertIn("concatLovePolygon", names)
        self.assertIn("concatVervielfacheZeile", names)
        self.assertIn("concatModallogik", names)
        self.assertIn("concat1RowPrimUniverse2", names)
        self.assertIn("concat1PrimzahlkreuzProContra", names)
        self.assertIn("concatPrimCreativityType", names)
        self.assertIn("concatGleichheitFreiheitDominieren", names)
        self.assertIn("concatGeistEmotionEnergieMaterieTopologie", names)
        self.assertIn("concatMondExponzierenLogarithmusTyp", names)
        self.assertTrue(hasattr(self.architecture, "generated_columns"))
        self.assertIn("generated_columns_registry", self.architecture.bootstrap_table_generation().snapshot())

    def test_table_output_layer_is_explicit(self):
        table_output = self.architecture.bootstrap_table_output(force_rebuild=True)
        snapshot = table_output.snapshot()
        self.assertIsInstance(table_output, TableOutputBundle)
        self.assertEqual(snapshot["class"], "TableOutputBundle")
        self.assertEqual(snapshot["output_class"], "TableOutput")
        tables = __import__("tableHandling").Tables(None, [])
        renderer = table_output.create(tables, [])
        self.assertEqual(type(renderer).__name__, "TableOutput")
        self.assertIs(renderer.tables, tables)


    def test_row_filtering_layer_is_explicit(self):
        row_filtering = self.architecture.bootstrap_row_filtering()
        snapshot = row_filtering.snapshot()
        self.assertIsInstance(row_filtering, RowFilteringBundle)
        self.assertEqual(snapshot["class"], "RowFilteringBundle")
        self.assertEqual(snapshot["legacy_owner"], "libs.lib4tables_prepare.Prepare")
        self.assertIn("moon_sun_planet", snapshot["condition_families"])
        self.assertIn("z_y_position_filters", snapshot["condition_families"])
        self.assertTrue(hasattr(self.architecture, "row_filtering"))

    def test_table_wrapping_layer_is_explicit(self):
        table_wrapping = self.architecture.bootstrap_table_wrapping(force_rebuild=True)
        snapshot = table_wrapping.snapshot()
        self.assertIsInstance(table_wrapping, TableWrappingBundle)
        self.assertEqual(snapshot["class"], "TableWrappingBundle")
        self.assertEqual(snapshot["legacy_owner"], "libs.lib4tables_prepare.Prepare")
        self.assertIn("wrap_cell_text", snapshot["morphisms"])
        self.assertIn("width_for_row", snapshot["morphisms"])
        self.assertTrue(hasattr(self.architecture, "table_wrapping"))

    def test_number_theory_layer_is_explicit(self):
        number_theory = self.architecture.bootstrap_number_theory()
        snapshot = number_theory.snapshot()
        self.assertIsInstance(number_theory, NumberTheoryBundle)
        self.assertEqual(snapshot["class"], "NumberTheoryBundle")
        self.assertEqual(snapshot["dependency_profile"], "math-only")
        self.assertIn("primCreativity", snapshot["morphisms"])
        self.assertIn("moonNumber", snapshot["morphisms"])
        self.assertEqual(arch_prim_creativity(1), lib4tables.primCreativity(1))
        self.assertEqual(arch_prim_creativity(12), lib4tables.primCreativity(12))
        self.assertEqual(arch_moon_number(16), lib4tables.moonNumber(16))
        self.assertTrue(hasattr(self.architecture, "number_theory"))

    def test_prepare_stack_delegates_to_row_filtering_layer(self):
        prepare_source = (REPO_ROOT / "libs" / "lib4tables_prepare.py").read_text(encoding="utf-8")
        row_filtering_source = (REPO_ROOT / "reta_architecture" / "row_filtering.py").read_text(encoding="utf-8")
        self.assertIn("architecture_filter_original_lines", prepare_source)
        self.assertIn("architecture_parameters_cmd_with_some_bereich", prepare_source)
        self.assertIn("architecture_set_zaehlungen", prepare_source)
        self.assertIn("def filter_original_lines", row_filtering_source)
        self.assertIn("def parameters_cmd_with_some_bereich", row_filtering_source)
        self.assertIn("def set_zaehlungen", row_filtering_source)
        self.assertIn("from .number_theory import isPrimMultiple, moonNumber, primFak", row_filtering_source)
        self.assertNotIn("from lib4tables import isPrimMultiple", row_filtering_source)
        self.assertLess(prepare_source.count('if "mond" in condition'), row_filtering_source.count('if "mond" in condition'))

    def test_table_preparation_layer_is_explicit(self):
        table_preparation = self.architecture.bootstrap_table_preparation()
        snapshot = table_preparation.snapshot()
        self.assertIsInstance(table_preparation, TablePreparationBundle)
        self.assertEqual(snapshot["class"], "TablePreparationBundle")
        self.assertEqual(snapshot["display_line_morphism"], "select_display_lines")
        self.assertEqual(snapshot["tag_gluing_morphism"], "tag_output_column")
        self.assertIn("prepare_main_output", snapshot["universal_operations"])
        self.assertIn("prepare_kombi_output", snapshot["universal_operations"])
        self.assertIn("process_parallel_row_chunks", snapshot["universal_operations"])
        self.assertEqual(snapshot["parallel_row_morphism"], "prepare_rows_in_processes")
        self.assertTrue(hasattr(self.architecture, "table_preparation"))

    def test_parallel_execution_layer_is_explicit_and_argv_driven(self):
        parallel_execution = self.architecture.bootstrap_parallel_execution()
        snapshot = parallel_execution.snapshot()
        self.assertIsInstance(parallel_execution, ParallelExecutionBundle)
        self.assertEqual(snapshot["class"], "ParallelExecutionBundle")
        self.assertEqual(snapshot["strategy"], "process_chunked_table_work")
        self.assertIn("processor_cores", snapshot)
        self.assertEqual(snapshot["default_workers"], RETA_PARALLEL_PROCESSOR_CORES)
        self.assertGreaterEqual(RETA_PROCESSOR_CORES.virtual, RETA_PROCESSOR_CORES.physical)
        self.assertIn("select_columns_in_processes", snapshot["morphisms"])
        self.assertIn("max_cell_text_len_in_processes", snapshot["morphisms"])
        self.assertIn("decode_religion_rows_in_processes", snapshot["morphisms"])
        argv, config = extract_parallel_config_from_argv([
            "reta.py",
            "--parallel=processes",
            "--parallel-workers=2",
            "--parallel-chunk-size=1",
        ])
        self.assertEqual(argv, ["reta.py"])
        self.assertIsInstance(config, ParallelExecutionConfig)
        self.assertEqual(config.mode, "processes")
        self.assertEqual(config.workers, 2)
        self.assertEqual(config.chunk_size, 1)

    def test_parallel_row_preparation_matches_serial_result(self):
        if "fork" not in multiprocessing.get_all_start_methods():
            self.skipTest("process-row parity check requires fork for this lightweight fixture")
        from reta_architecture.table_preparation import prepare_output_table

        class FakePrepare:
            def __init__(self, parallel_config):
                self.tables = SimpleNamespace(
                    generatedSpaltenParameter={},
                    generatedSpaltenParameter_Tags={},
                    dataDict=[{}],
                )
                self.hoechsteZeile = {1024: 2}
                self.originalLinesRange = range(3)
                self.shellRowsAmount = 0
                self.rowsAsNumbers = {0, 1}
                self.breiten = []
                self.textwidth = 0
                self.religionNumbers = []
                self.ifZeilenSetted = False
                self.parallel_config = parallel_config

            def FilterOriginalLines(self, numRange, paramLines):
                return set(numRange)

            def setWidth(self, rowToDisplay, combiRows1=0):
                return 0

            def wrapping(self, text, length):
                return None

        content_table = [
            ["A", "B", "C"],
            ["alpha", "beta", "skip"],
            ["gamma", "delta", "skip"],
        ]
        serial_prepare = FakePrepare(ParallelExecutionConfig(mode="off"))
        parallel_prepare = FakePrepare(
            ParallelExecutionConfig(
                mode="processes",
                workers=2,
                chunk_size=1,
                threshold=1,
                start_method="fork",
            )
        )
        serial_result = prepare_output_table(serial_prepare, set(), set(), content_table, {0, 1}, {})
        parallel_result = prepare_output_table(parallel_prepare, set(), set(), content_table, {0, 1}, {})
        self.assertEqual(parallel_result, serial_result)
        self.assertEqual(parallel_prepare.religionNumbers, serial_prepare.religionNumbers)
        self.assertEqual(parallel_prepare.parallel_last_result["class"], "ParallelRowsResult")

    def test_more_parallel_table_helpers_match_serial_results(self):
        if "fork" not in multiprocessing.get_all_start_methods():
            self.skipTest("process helper parity check requires fork for this lightweight fixture")
        config = ParallelExecutionConfig(
            mode="processes",
            workers=2,
            chunk_size=1,
            threshold=1,
            start_method="fork",
        )

        religion_rows = [
            (0, ["a<b", "|{\"\":\"plain\",\"html\":\"<b>html</b>\",\"bbcode\":\"[b]bb[/b]\"}|"]),
            (1, ["plain", "cell"]),
        ]
        religion_result = decode_religion_rows_in_processes(religion_rows, "html", config=config)
        self.assertEqual(religion_result.values[0], ["a&lt;b", "<b>html</b>"])

        kombi_rows = [(0, ["Header", "Name"]), (1, ["1|2/3", "Hund"])]
        kombi_result = decode_kombi_rows_in_processes(kombi_rows, config=config)
        self.assertEqual(kombi_result.values[1][1], ["1|2/3", "(1|2/3) Hund (1|2/3)"])
        self.assertEqual(kombi_result.values[1][2], [1, 2, 3])

        table = [
            [["a"], ["b"], ["c"]],
            [["d"], ["e"], ["f"]],
        ]
        selected = select_columns_in_processes(table, [1, 3], config=config)
        self.assertEqual(selected.values, [[["a"], ["c"]], [["d"], ["f"]]])

        new_table = [
            [["A", ""], ["long"]],
            [["abc"], ["x", "yyyy"]],
        ]
        widths = max_cell_text_len_in_processes(new_table, range(2), config=config)
        self.assertEqual(dict(widths.values), {0: 3, 1: 4})

        chosen = OrderedDict(((2, [1]), (3, [0, 1])))
        kombi_join = prepare_kombi_join_tables_in_processes(chosen, [["H"], ["A"]], config=config)
        self.assertEqual(kombi_join.values[0], OrderedDict(((2, [["A"]]),)))
        self.assertEqual(kombi_join.values[1], OrderedDict(((3, [["H"], ["A"]]),)))

    def test_table_generation_layer_is_explicit(self):
        table_generation = self.architecture.bootstrap_table_generation(csv_file_names=center.i18n.csvFileNames)
        snapshot = table_generation.snapshot()
        self.assertEqual(snapshot["class"], "TableGenerationBundle")
        self.assertIn("concatModallogik", snapshot["generated_morphisms"])
        self.assertIn("concat1RowPrimUniverse2", snapshot["generated_morphisms"])
        self.assertIn("concat1PrimzahlkreuzProContra", snapshot["generated_morphisms"])
        self.assertIn("createSpalteGestirn", snapshot["generated_morphisms"])
        self.assertEqual(len(snapshot["kombi_csvs"]), 2)


    def test_parameter_runtime_layer_is_explicit(self):
        parameter_runtime = self.architecture.bootstrap_parameter_runtime()
        snapshot = parameter_runtime.snapshot()
        self.assertIsInstance(parameter_runtime, ParameterRuntimeBundle)
        self.assertEqual(snapshot["class"], "ParameterRuntimeBundle")
        self.assertEqual(snapshot["column_function"], "produce_all_spalten_numbers")
        self.assertEqual(snapshot["parse_function"], "parameters_to_commands_and_numbers")
        self.assertEqual(snapshot["upper_limit_apply_function"], "apply_upper_limit_argument")
        self.assertTrue(hasattr(self.architecture, "parameter_runtime"))

    def test_program_workflow_layer_is_explicit(self):
        program_workflow = self.architecture.bootstrap_program_workflow()
        snapshot = program_workflow.snapshot()
        self.assertIsInstance(program_workflow, ProgramWorkflowBundle)
        self.assertEqual(snapshot["class"], "ProgramWorkflowBundle")
        self.assertIn("load_religion_table", snapshot["orchestration_steps"])
        self.assertIn("join_kombi_tables", snapshot["orchestration_steps"])
        self.assertEqual(len(snapshot["kombi_csvs"]), 2)
        self.assertTrue(hasattr(self.architecture, "program_workflow"))

    def test_prepare_stack_delegates_to_table_preparation_layer(self):
        prepare_source = (REPO_ROOT / "libs" / "lib4tables_prepare.py").read_text(encoding="utf-8")
        program_workflow_source = (REPO_ROOT / "reta_architecture" / "program_workflow.py").read_text(encoding="utf-8")
        table_generation_source = (REPO_ROOT / "reta_architecture" / "table_generation.py").read_text(encoding="utf-8")
        table_preparation_source = (REPO_ROOT / "reta_architecture" / "table_preparation.py").read_text(encoding="utf-8")
        self.assertIn("architecture_prepare_output_table", prepare_source)
        self.assertIn("architecture_select_display_lines", prepare_source)
        self.assertIn("architecture_tag_output_column", prepare_source)
        self.assertIn("prepare_main_output", program_workflow_source)
        self.assertIn("prepare_kombi_output", program_workflow_source)
        self.assertIn("capture_last_line_number", table_generation_source)
        self.assertIn("def prepare_output_table", table_preparation_source)
        self.assertIn("prepare_rows_in_processes", table_preparation_source)
        self.assertIn("def tag_output_column", table_preparation_source)
        self.assertLess(prepare_source.count("lib4tables_Enum.tableTags2"), table_preparation_source.count("lib4tables_Enum.tableTags2"))

    def test_reta_program_delegates_to_program_workflow_layer(self):
        reta_source = (REPO_ROOT / "reta.py").read_text(encoding="utf-8")
        workflow_source = (REPO_ROOT / "reta_architecture" / "program_workflow.py").read_text(encoding="utf-8")
        self.assertIn("bootstrap_program_workflow", reta_source)
        self.assertIn("workflow_everything", reta_source)
        self.assertIn("combi_table_workflow", reta_source)
        self.assertIn("bootstrap_column_selection", workflow_source)
        self.assertIn("bind_program_sections", workflow_source)
        self.assertIn("bootstrap_table_generation", workflow_source)
        self.assertIn("build_for_program", workflow_source)
        self.assertIn("parameters_to_commands_and_numbers", reta_source)
        self.assertIn("produce_all_spalten_numbers", reta_source)
        self.assertNotIn("def resultingSpaltenFromTuple", reta_source)
        self.assertNotIn("def parameters_to_commands_and_numbers", reta_source)
        self.assertNotIn("CsvTheirsSpalten: dict = {}", reta_source)
        self.assertNotIn("namedtuple(\n            \"SpaltenTyp\"", reta_source)


    def test_output_syntax_layer_is_explicit(self):
        output_syntax = self.architecture.bootstrap_output_syntax(force_rebuild=True)
        snapshot = output_syntax.snapshot()
        self.assertIsInstance(output_syntax, OutputSyntaxBundle)
        self.assertEqual(snapshot["class"], "OutputSyntaxBundle")
        self.assertEqual(snapshot["architecture_owner"], "reta_architecture.output_syntax")
        self.assertIn("html", snapshot["modes"])
        self.assertTrue(snapshot["modes"]["markdown"]["force_zero_width"])
        self.assertIs(output_syntax.class_for("markdown"), lib4tables.markdownSyntax)
        self.assertTrue(hasattr(self.architecture, "output_syntax"))

    def test_lib4tables_is_thin_number_and_output_facade(self):
        lib4tables_source = (REPO_ROOT / "libs" / "lib4tables.py").read_text(encoding="utf-8")
        output_syntax_source = (REPO_ROOT / "reta_architecture" / "output_syntax.py").read_text(encoding="utf-8")
        self.assertIn("reta_architecture.output_syntax", lib4tables_source)
        self.assertIn("reta_architecture.number_theory", lib4tables_source)
        self.assertNotIn("class htmlSyntax", lib4tables_source)
        self.assertNotIn("def primFak", lib4tables_source)
        self.assertIn("class htmlSyntax", output_syntax_source)
        self.assertIn("class OutputSyntaxBundle", output_syntax_source)

    def test_output_semantics_is_explicit(self):
        snapshot = self.architecture.output_semantics.snapshot()
        self.assertEqual(snapshot["available_modes"], ["bbcode", "csv", "emacs", "html", "markdown", "nichts", "shell"])
        self.assertTrue(snapshot["mode_specs"]["csv"]["force_one_table"])
        self.assertTrue(snapshot["mode_specs"]["markdown"]["force_zero_width"])

    def test_output_mode_registry_matches_table_runtime(self):
        program = self._build_program()
        self.assertEqual(program.architecture.output_semantics.mode_for_tables(program.tables), "shell")
        application = program.apply_output_mode("markdown")
        self.assertTrue(application)
        self.assertEqual(program.tables.outputModeName, "markdown")
        self.assertTrue(program.tables.markdownOutputYes)
        self.assertIsInstance(program.tables.outType, lib4tables.markdownSyntax)
        program.apply_output_mode("html")
        self.assertEqual(program.tables.outputModeName, "html")
        self.assertTrue(program.tables.htmlOutputYes)

    def test_semantic_builder_does_not_mutate_schema_sets(self):
        source = (REPO_ROOT / "reta_architecture" / "semantics_builder.py").read_text(encoding="utf-8")
        self.assertIn("next(iter(spalten_nummer_oder_etc))", source)
        self.assertNotIn("spalten_nummer_oder_etc.pop()", source)

    def test_empty_modal_concat_short_circuits(self):
        source = (REPO_ROOT / "reta_architecture" / "generated_columns.py").read_text(encoding="utf-8")
        concat_source = (REPO_ROOT / "libs" / "lib4tables_concat.py").read_text(encoding="utf-8")
        self.assertIn("def concat_modallogik", source)
        self.assertIn("if len(conceptsRowsSetOfTuple) == 0:", source)
        self.assertIn("return concat.relitable, rowsAsNumbers", source)
        self.assertIn("generated_column_morphisms.concat_modallogik", concat_source)
        self.assertIn("generated_column_morphisms.concat_vervielfache_zeile", concat_source)
        self.assertIn("generated_column_morphisms.concat_prim_universe_row", concat_source)
        self.assertIn("generated_column_morphisms.concat_primzahlkreuz_pro_contra", concat_source)

    def test_universal_merge_avoids_repeated_deepcopy(self):
        source = (REPO_ROOT / "reta_architecture" / "universal.py").read_text(encoding="utf-8")
        self.assertIn("merged_data_dicts = [dict(d) for d in data_dicts1]", source)
        self.assertIn("deepcopy every newly merged value", source)
        self.assertNotIn("deepcopy(dict(d))", source)

    def test_output_stack_sources_use_explicit_output_architecture(self):
        table_source = (REPO_ROOT / "libs" / "tableHandling.py").read_text(encoding="utf-8")
        table_runtime_source = (REPO_ROOT / "reta_architecture" / "table_runtime.py").read_text(encoding="utf-8")
        reta_source = (REPO_ROOT / "reta.py").read_text(encoding="utf-8")
        self.assertIn("from reta_architecture.table_runtime import", table_source)
        self.assertIn("OUTPUT_SEMANTICS = bootstrap_output_semantics", table_runtime_source)
        self.assertIn("from .table_output import TableOutput", table_runtime_source)
        self.assertNotIn("class Output:", table_runtime_source)
        self.assertIn("def apply_output_mode(self, outputtype: str)", reta_source)
        self.assertNotIn('if outputtype == i18n.ausgabeArt["shell"]', reta_source)

    def test_remaining_executable_scripts_use_split_i18n_proxy(self):
        html_source = (REPO_ROOT / "grundStrukHtml.py").read_text(encoding="utf-8")
        readme_source = (REPO_ROOT / "libs" / "generate4readme.py").read_text(encoding="utf-8")
        self.assertIn("build_split_i18n_proxy", html_source)
        self.assertIn("build_split_i18n_proxy", readme_source)
        self.assertNotIn("import i18n.words as i18n", html_source)
        self.assertNotIn("import i18n.words as i18n", readme_source)


    def test_execution_network_layer_is_explicit_and_deterministic(self):
        execution_network = self.architecture.bootstrap_execution_network()
        snapshot = execution_network.snapshot()
        self.assertIsInstance(execution_network, ExecutionNetworkBundle)
        self.assertEqual(snapshot["category"], "ExecutionNetworkCategory")
        self.assertIn("FifoTaskQueue", snapshot["queues"])
        self.assertIn("HalfDuplexChannel", snapshot["channels"])
        self.assertIn("deterministic_reduce", snapshot["morphisms"])

        tasks = [ExecutionTask(2, 2), ExecutionTask(0, 0), ExecutionTask(1, 1)]
        result = execute_tasks_deterministically(
            tasks,
            handler=_architecture_refactor_double_payload,
            config=ExecutionNetworkConfig(queue_discipline="fifo"),
        )
        self.assertEqual(result.values, [0, 2, 4])
        self.assertEqual(result.mode, "serial")

        fifo = FifoTaskQueue(tasks)
        self.assertEqual([fifo.pop().index, fifo.pop().index, fifo.pop().index], [2, 0, 1])
        lifo = LifoTaskStack(tasks)
        self.assertEqual([lifo.pop().index, lifo.pop().index, lifo.pop().index], [1, 0, 2])
        priority = PriorityTaskQueue([ExecutionTask(0, "slow", priority=10), ExecutionTask(1, "fast", priority=1)])
        self.assertEqual(priority.pop().payload, "fast")

        half = HalfDuplexChannel()
        half.send_request({"cmd": "run"})
        self.assertEqual(half.receive_request(timeout=0.1), {"cmd": "run"})
        half.send_response({"ok": True})
        self.assertEqual(half.receive_response(timeout=0.1), {"ok": True})
        full = FullDuplexChannel()
        full.send_a_to_b("cancel")
        full.send_b_to_a("progress")
        self.assertEqual(full.receive_a_to_b(timeout=0.1), "cancel")
        self.assertEqual(full.receive_b_to_a(timeout=0.1), "progress")

    def test_persistence_layer_is_explicit_and_roundtrips_sections(self):
        persistence = self.architecture.bootstrap_persistence()
        snapshot = persistence.snapshot()
        self.assertIsInstance(persistence, PersistenceBundle)
        self.assertEqual(snapshot["category"], "PersistenceCategory")
        self.assertIn("local_sections", snapshot["tables"])
        self.assertIn("persist_section", snapshot["morphisms"])

        file_backed = bootstrap_persistence(config=PersistenceConfig(db_path=":memory:"))
        with file_backed.connect() as connection:
            section = persist_section(
                connection,
                kind="unit",
                name="prompt",
                payload={"text": "a1"},
                context={"scope": "prompt"},
            )
            loaded = load_section(connection, section.digest)
            self.assertEqual(loaded["payload"], {"text": "a1"})

            sheaf = persist_sheaf_snapshot(
                connection,
                sheaf_name="ParameterSemanticsSheaf",
                payload={"canonical": "Religionen"},
            )
            self.assertEqual(load_sheaf_snapshot(connection, sheaf.digest)["payload"], {"canonical": "Religionen"})

            execution = persist_execution_run(
                connection,
                operation="chunk-test",
                task_count=2,
                payload={"values": [1, 2]},
            )
            self.assertEqual(execution.table, "execution_runs")

            audit = record_audit_event(connection, event_type="validation", subject="unit", payload={"status": "passed"})
            events = query_audit_events(connection, event_type="validation", subject="unit")
            self.assertEqual(events[0]["payload"], {"status": "passed"})
            self.assertEqual(events[0]["payload_hash"], audit.digest)

            cache_put(connection, "unit-cache", {"value": 3})
            self.assertEqual(cache_get(connection, "unit-cache"), {"value": 3})
            self.assertEqual(invalidate_cache(connection, "unit-cache"), 1)
            self.assertIsNone(cache_get(connection, "unit-cache"))

    def test_package_integrity_manifest_is_explicit(self):
        manifest = RepoManifest.from_tree(REPO_ROOT)
        self.assertEqual(manifest.missing_required, ())
        self.assertGreaterEqual(manifest.file_count, 240)
        self.assertEqual(manifest.csv_line_counts["csv/religion.csv"], 1043)
        self.assertEqual(manifest.csv_line_counts["csv/vn-religion.csv"], 1043)
        self.assertEqual(manifest.suspicious_csvs, ())
        self.assertIn("reta_architecture/prompt_preparation.py", manifest.files)
        self.assertIn("reta_architecture/parameter_runtime.py", manifest.files)
        self.assertIn("reta_architecture/prompt_interaction.py", manifest.files)
        self.assertIn("reta_architecture/column_selection.py", manifest.files)
        self.assertIn("reta_architecture/table_generation.py", manifest.files)
        self.assertIn("reta_architecture/table_preparation.py", manifest.files)
        self.assertIn("reta_architecture/table_wrapping.py", manifest.files)
        self.assertIn("reta_architecture/table_state.py", manifest.files)
        self.assertIn("reta_architecture/table_output.py", manifest.files)
        self.assertIn("reta_architecture/table_runtime.py", manifest.files)
        self.assertIn("reta_architecture/output_syntax.py", manifest.files)
        self.assertIn("reta_architecture/program_workflow.py", manifest.files)
        self.assertIn("reta_architecture/concat_csv.py", manifest.files)
        self.assertIn("reta_architecture/combi_join.py", manifest.files)
        self.assertIn("reta_architecture/category_theory.py", manifest.files)
        self.assertIn("reta_architecture/architecture_map.py", manifest.files)
        self.assertIn("reta_architecture/architecture_contracts.py", manifest.files)
        self.assertIn("reta_architecture/architecture_witnesses.py", manifest.files)
        self.assertIn("reta_architecture/architecture_validation.py", manifest.files)
        self.assertIn("reta_architecture/architecture_coherence.py", manifest.files)
        self.assertIn("reta_architecture/architecture_traces.py", manifest.files)
        self.assertIn("reta_architecture/architecture_boundaries.py", manifest.files)
        self.assertIn("reta_architecture/architecture_impact.py", manifest.files)
        self.assertIn("reta_architecture/architecture_migration.py", manifest.files)
        self.assertIn("reta_architecture/architecture_rehearsal.py", manifest.files)
        self.assertIn("reta_architecture/architecture_activation.py", manifest.files)
        self.assertIn("reta_architecture/row_ranges.py", manifest.files)
        self.assertIn("reta_architecture/arithmetic.py", manifest.files)
        self.assertIn("reta_architecture/console_io.py", manifest.files)
        self.assertIn("reta_architecture/completion_word.py", manifest.files)
        self.assertIn("reta_architecture/completion_nested.py", manifest.files)

    def test_parameter_semantics_regression_counts(self):
        program = self._semantic_program()
        self.assertEqual(len(program.paraDict), 4155)
        self.assertEqual(
            [len(d) if hasattr(d, "__len__") else None for d in program.dataDict],
            [555, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0],
        )
        self.assertEqual(len(program.kombiReverseDict), 46)
        self.assertEqual(len(program.kombiReverseDict2), 51)
        self.assertEqual(len(program.AllSimpleCommandSpalten), 555)

    def test_builder_standalone_matches_program_semantics(self):
        program = self._semantic_program()
        builder = ParameterSemanticsBuilder(
            self.architecture.schema,
            gebrochen_spalten_maximum_plus1=reta.gebrochenSpaltenMaximumPlus1,
            invert_alles=False,
            initial_data_dict=[{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}],
            prim_number_predicate=reta.primCreativity,
            alles_parameter_names=reta.i18n.ParametersMain.alles,
        )
        built = builder.build()
        self.assertEqual(len(built.para_dict), len(program.paraDict))
        self.assertEqual([len(d) for d in built.data_dict], [len(d) for d in program.dataDict])
        self.assertEqual(len(built.kombi_reverse_dict), len(program.kombiReverseDict))
        self.assertEqual(len(built.kombi_reverse_dict2), len(program.kombiReverseDict2))
        self.assertEqual(len(built.all_simple_command_columns), len(program.AllSimpleCommandSpalten))


    def test_meta_columns_layer_is_explicit(self):
        meta_columns = self.architecture.bootstrap_meta_columns(force_rebuild=True)
        snapshot = meta_columns.snapshot()
        self.assertIsInstance(meta_columns, MetaColumnsBundle)
        self.assertEqual(snapshot["class"], "MetaColumnsBundle")
        self.assertGreaterEqual(snapshot["count"], 3)
        names = {item["method_name"] for item in snapshot["morphisms"]}
        self.assertIn("spalteMetaKontretTheorieAbstrakt_etc_1", names)
        self.assertIn("spalteFuerGegenInnenAussenSeitlichPrim", names)
        self.assertIn("readOneCSVAndReturn", names)


    def test_concat_csv_layer_is_explicit(self):
        concat_csv = self.architecture.bootstrap_concat_csv(force_rebuild=True)
        snapshot = concat_csv.snapshot()
        self.assertIsInstance(concat_csv, ConcatCsvBundle)
        self.assertEqual(snapshot["class"], "ConcatCsvBundle")
        names = {item["method_name"] for item in snapshot["morphisms"]}
        self.assertIn("readConcatCsv", names)
        self.assertIn("readConcatCsv_SetHtmlParamaters", names)
        self.assertIn("convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction", names)
        self.assertTrue(hasattr(self.architecture, "concat_csv"))
        self.assertIn("prim", snapshot["csv_sources"])

    def test_concat_stack_delegates_to_concat_csv_layer(self):
        concat_source = (REPO_ROOT / "libs" / "lib4tables_concat.py").read_text(encoding="utf-8")
        concat_csv_source = (REPO_ROOT / "reta_architecture" / "concat_csv.py").read_text(encoding="utf-8")
        table_generation_source = (REPO_ROOT / "reta_architecture" / "table_generation.py").read_text(encoding="utf-8")
        self.assertIn("concat_csv_morphisms.readConcatCsv", concat_source)
        self.assertIn("concat_csv_morphisms.convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction", concat_source)
        self.assertIn("def readConcatCsv", concat_csv_source)
        self.assertIn("def readConcatCsv_SetHtmlParamaters", concat_csv_source)
        self.assertIn("self.concat_csv.read_concat_csv", table_generation_source)
        self.assertNotIn("self.relitable, tableToAdd = self.tables.fillBoth", concat_source)
        self.assertIn("concat.relitable, tableToAdd = concat.tables.fillBoth", concat_csv_source)


    def test_combi_join_layer_is_explicit(self):
        combi_join = self.architecture.bootstrap_combi_join(force_rebuild=True)
        snapshot = combi_join.snapshot()
        self.assertIsInstance(combi_join, KombiJoinBundle)
        self.assertEqual(snapshot["class"], "KombiJoinBundle")
        self.assertIn("readKombiCsv", snapshot["morphisms"])
        self.assertIn("tableJoin", snapshot["morphisms"])
        self.assertTrue(hasattr(self.architecture, "combi_join"))


    def test_table_state_layer_is_explicit(self):
        table_state = self.architecture.bootstrap_table_state(force_rebuild=True)
        snapshot = table_state.snapshot()
        self.assertIsInstance(table_state, TableStateBundle)
        self.assertEqual(snapshot["class"], "TableStateBundle")
        self.assertIn("generated_columns", snapshot["sections"])
        self.assertTrue(hasattr(self.architecture, "table_state"))
        tables = self.architecture.bootstrap_table_runtime().create_tables(None, None)
        self.assertEqual(tables.tableStateSnapshot["highest_rows"], {1024: 1024, 114: 163})
        self.assertIs(tables.generatedSpaltenParameter, tables._state_sections.generated_columns.parameters)
        self.assertIs(tables.generatedSpaltenParameter_Tags, tables._state_sections.generated_columns.tags)
        tables.keineUeberschriften = True
        tables.keineleereninhalte = True
        tables.spaltegGestirn = True
        self.assertTrue(tables.tableStateSnapshot["display"]["keine_ueberschriften"])
        self.assertTrue(tables.tableStateSnapshot["display"]["keine_leeren_inhalte"])
        self.assertTrue(tables.tableStateSnapshot["display"]["spalte_gestirn"])

    def test_table_runtime_layer_owns_tables(self):
        table_source = (REPO_ROOT / "libs" / "tableHandling.py").read_text(encoding="utf-8")
        table_runtime_source = (REPO_ROOT / "reta_architecture" / "table_runtime.py").read_text(encoding="utf-8")
        reta_source = (REPO_ROOT / "reta.py").read_text(encoding="utf-8")
        generated_source = (REPO_ROOT / "reta_architecture" / "generated_columns.py").read_text(encoding="utf-8")
        combi_source = (REPO_ROOT / "reta_architecture" / "combi_join.py").read_text(encoding="utf-8")
        self.assertIn("from reta_architecture.table_runtime import", table_source)
        self.assertNotIn("class Tables:", table_source)
        self.assertIn("class Tables:", table_runtime_source)
        self.assertIn("from .table_state import TableStateBundle", table_runtime_source)
        self.assertNotIn("from reta_architecture.table_runtime import Tables", reta_source)
        self.assertIn("bootstrap_table_runtime().create_tables", reta_source)
        self.assertIn("state_sections", table_runtime_source)
        self.assertIn("from .combi_join import KombiJoin", table_runtime_source)
        self.assertIn("Combi = KombiJoin", table_runtime_source)
        self.assertNotIn("class Combi:", table_runtime_source)
        self.assertTrue(hasattr(self.architecture, "table_runtime"))
        self.assertIn("def create_spalte_gestirn", generated_source)
        self.assertIn("createSpalteGestirn", generated_source)
        self.assertIn("class KombiJoin", combi_source)
        self.assertIn("def readKombiCsv", combi_source)
        self.assertIn("def tableJoin", combi_source)


    def test_category_theory_layer_is_explicit(self):
        category_theory = self.architecture.bootstrap_category_theory()
        snapshot = category_theory.snapshot()
        self.assertIsInstance(category_theory, CategoryTheoryBundle)
        self.assertEqual(snapshot["class"], "CategoryTheoryBundle")
        self.assertTrue(hasattr(self.architecture, "category_theory"))
        self.assertIn("natural_transformation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["categories"], 19)
        self.assertGreaterEqual(snapshot["counts"]["functors"], 50)
        self.assertGreaterEqual(snapshot["counts"]["natural_transformations"], 23)
        category_names = {item["name"] for item in snapshot["categories"]}
        self.assertIn("OpenRetaContextCategory", category_names)
        self.assertIn("TableSectionCategory", category_names)
        self.assertIn("LegacyFacadeCategory", category_names)
        self.assertIn("ArchitectureValidationCategory", category_names)
        self.assertIn("ArchitectureTraceCategory", category_names)
        self.assertIn("ArchitectureBoundaryCategory", category_names)
        self.assertIn("ArchitectureImpactCategory", category_names)
        self.assertIn("ArchitectureMigrationCategory", category_names)
        self.assertIn("ArchitectureRehearsalCategory", category_names)
        self.assertIn("ArchitectureActivationCategory", category_names)
        self.assertIn("ActivatedRowRangeCategory", category_names)
        self.assertIn("ActivatedArithmeticCategory", category_names)
        self.assertIn("ActivatedConsoleIOCategory", category_names)
        self.assertIn("ActivatedWordCompletionCategory", category_names)
        functor_names = {item["name"] for item in snapshot["functors"]}
        self.assertIn("RawCommandPresheafFunctor", functor_names)
        self.assertIn("GeneratedColumnEndofunctorFamily", functor_names)
        self.assertIn("OutputRenderingFunctorFamily", functor_names)
        self.assertIn("ContractToValidationFunctor", functor_names)
        self.assertIn("WitnessToValidationFunctor", functor_names)
        self.assertIn("CoherenceToTraceFunctor", functor_names)
        self.assertIn("CoherenceToBoundaryFunctor", functor_names)
        self.assertIn("TraceBoundaryImpactFunctor", functor_names)
        self.assertIn("BoundaryImpactFunctor", functor_names)
        self.assertIn("ImpactGateValidationFunctor", functor_names)
        self.assertIn("MigrationCandidateFunctor", functor_names)
        self.assertIn("ImpactToMigrationPlanFunctor", functor_names)
        self.assertIn("ImpactGateBindingFunctor", functor_names)
        self.assertIn("MigrationWaveOrderingFunctor", functor_names)
        self.assertIn("MigrationGateCoherenceFunctor", functor_names)
        self.assertIn("MigrationStepRehearsalFunctor", functor_names)
        self.assertIn("MigrationGateRehearsalFunctor", functor_names)
        self.assertIn("RehearsalCoverFunctor", functor_names)
        self.assertIn("RehearsalGateValidationFunctor", functor_names)
        self.assertIn("RehearsalReadinessCoherenceFunctor", functor_names)
        self.assertIn("RehearsalActivationFunctor", functor_names)
        self.assertIn("GateActivationFunctor", functor_names)
        self.assertIn("ActivationTransactionFunctor", functor_names)
        self.assertIn("ActivationRollbackFunctor", functor_names)
        self.assertIn("ActivationValidationFunctor", functor_names)
        self.assertIn("ActivationCoherenceFunctor", functor_names)
        self.assertIn("RowRangeActivationFunctor", functor_names)
        self.assertIn("CenterRowRangeCompatibilityFunctor", functor_names)
        self.assertIn("RowRangeInputFunctor", functor_names)
        self.assertIn("RowRangeValidationFunctor", functor_names)
        self.assertIn("ArithmeticActivationFunctor", functor_names)
        self.assertIn("CenterArithmeticCompatibilityFunctor", functor_names)
        self.assertIn("ArithmeticRowRangeGluingFunctor", functor_names)
        self.assertIn("ArithmeticValidationFunctor", functor_names)
        self.assertIn("ConsoleIOActivationFunctor", functor_names)
        self.assertIn("CenterConsoleIOCompatibilityFunctor", functor_names)
        self.assertIn("ConsoleIOOutputRenderingFunctor", functor_names)
        self.assertIn("ConsoleIOValidationFunctor", functor_names)
        self.assertIn("WordCompletionActivationFunctor", functor_names)
        self.assertIn("LegacyWordCompleterCompatibilityFunctor", functor_names)
        self.assertIn("WordCompletionPromptFunctor", functor_names)
        self.assertIn("WordCompletionValidationFunctor", functor_names)
        transformation_names = {item["name"] for item in snapshot["natural_transformations"]}
        self.assertIn("RawToCanonicalParameterTransformation", transformation_names)
        self.assertIn("PresheafToSheafGluingTransformation", transformation_names)
        self.assertIn("LegacyToArchitectureTransformation", transformation_names)
        self.assertIn("TableRuntimeToStateSectionsTransformation", transformation_names)
        self.assertIn("ContractWitnessValidationTransformation", transformation_names)
        self.assertIn("CoherenceToTraceTransformation", transformation_names)
        self.assertIn("CoherenceBoundaryValidationTransformation", transformation_names)
        self.assertIn("TraceBoundaryImpactTransformation", transformation_names)
        self.assertIn("ImpactGateValidationTransformation", transformation_names)
        self.assertIn("ImpactGateMigrationTransformation", transformation_names)
        self.assertIn("MigrationPlanCoherenceTransformation", transformation_names)
        self.assertIn("MigrationRehearsalNaturalityTransformation", transformation_names)
        self.assertIn("RehearsalReadinessValidationTransformation", transformation_names)
        self.assertIn("RehearsalActivationNaturalityTransformation", transformation_names)
        self.assertIn("ActivationRollbackValidationTransformation", transformation_names)
        self.assertIn("CenterRowRangeToArchitectureTransformation", transformation_names)
        self.assertIn("RowRangeValidationTransformation", transformation_names)
        self.assertIn("CenterArithmeticToArchitectureTransformation", transformation_names)
        self.assertIn("ArithmeticRowRangeGluingTransformation", transformation_names)
        self.assertIn("CenterConsoleIOToArchitectureTransformation", transformation_names)
        self.assertIn("ConsoleIOOutputValidationTransformation", transformation_names)
        self.assertIn("WordCompleterToArchitectureTransformation", transformation_names)
        self.assertIn("WordCompletionValidationTransformation", transformation_names)
        self.assertEqual(snapshot["plan"]["behaviour_change"].split(";")[0], "keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung")
        source = (REPO_ROOT / "reta_architecture" / "category_theory.py").read_text(encoding="utf-8")
        self.assertIn("class FunctorSpec", source)
        self.assertIn("class NaturalTransformationSpec", source)
        self.assertIn("bootstrap_category_theory", source)


    def test_architecture_map_layer_is_explicit(self):
        architecture_map = self.architecture.bootstrap_architecture_map()
        snapshot = architecture_map.snapshot()
        self.assertIsInstance(architecture_map, ArchitectureMapBundle)
        self.assertEqual(snapshot["class"], "ArchitectureMapBundle")
        self.assertEqual(snapshot["stage"], 42)
        self.assertTrue(hasattr(self.architecture, "architecture_map"))
        self.assertGreaterEqual(snapshot["counts"]["capsules"], 10)
        self.assertGreaterEqual(snapshot["counts"]["flows"], 10)
        self.assertGreaterEqual(snapshot["counts"]["legacy_mappings"], 10)
        self.assertEqual(snapshot["counts"]["stage_steps"], 42)
        capsule_names = {item["name"] for item in snapshot["capsules"]}
        self.assertIn("SchemaTopologyCapsule", capsule_names)
        self.assertIn("TableCoreCapsule", capsule_names)
        self.assertIn("CategoricalMetaCapsule", capsule_names)
        legacy_owners = {item["legacy_owner"] for item in snapshot["legacy_mappings"]}
        self.assertIn("reta.py", legacy_owners)
        self.assertIn("retaPrompt.py", legacy_owners)
        self.assertIn("libs/tableHandling.py", legacy_owners)
        self.assertIn("libs/lib4tables_concat.py", legacy_owners)
        self.assertIn("reta_architecture/architecture_rehearsal.py", legacy_owners)
        self.assertIn("reta_architecture/architecture_activation.py", legacy_owners)
        self.assertIn("libs/center.py", legacy_owners)
        self.assertIn("reta_architecture/row_ranges.py", legacy_owners)
        self.assertIn("reta_architecture/console_io.py", legacy_owners)
        self.assertIn("libs/word_completerAlx.py", legacy_owners)
        self.assertIn("reta_architecture/completion_word.py", legacy_owners)
        self.assertIn("libs/nestedAlx.py", legacy_owners)
        self.assertIn("reta_architecture/completion_nested.py", legacy_owners)
        self.assertIn("flowchart TD", snapshot["diagrams"]["mermaid"])
        self.assertIn("RetaArchitectureRoot", snapshot["diagrams"]["text"])
        self.assertEqual(snapshot["markdown_audit"]["markdown_files_in_stage27_package"], 58)
        self.assertEqual(snapshot["markdown_audit"]["uploaded_tar_markdown_files"], 0)
        source = (REPO_ROOT / "reta_architecture" / "architecture_map.py").read_text(encoding="utf-8")
        self.assertIn("ArchitectureWitnessBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureValidationBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureTraceBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureBoundariesBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureImpactBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureMigrationBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureRehearsalBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureActivationBundle", snapshot["diagrams"]["text"])
        self.assertIn("RowRangeMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArithmeticMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("ConsoleIOMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("WordCompletionMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("NestedCompletionMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("class ArchitectureMapBundle", source)
        self.assertIn("bootstrap_architecture_map", source)


    def test_architecture_contract_layer_is_explicit(self):
        contracts = self.architecture.bootstrap_architecture_contracts()
        snapshot = contracts.snapshot()
        self.assertIsInstance(contracts, ArchitectureContractsBundle)
        self.assertEqual(snapshot["class"], "ArchitectureContractsBundle")
        self.assertEqual(snapshot["stage"], 41)
        self.assertTrue(hasattr(self.architecture, "architecture_contracts"))
        self.assertIn("commutative_diagram", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["commutative_diagrams"], 23)
        self.assertGreaterEqual(snapshot["counts"]["capsule_contracts"], 10)
        self.assertGreaterEqual(snapshot["counts"]["laws"], 18)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        diagram_names = {item["name"] for item in snapshot["commutative_diagrams"]}
        self.assertIn("RawCommandNaturalitySquare", diagram_names)
        self.assertIn("PresheafSheafGluingSquare", diagram_names)
        self.assertIn("RenderedOutputParitySquare", diagram_names)
        self.assertIn("ValidationWitnessCommutationSquare", diagram_names)
        self.assertIn("CoherenceTraceNavigationSquare", diagram_names)
        self.assertIn("BoundaryImportGraphCommutationSquare", diagram_names)
        self.assertIn("TraceBoundaryImpactSquare", diagram_names)
        self.assertIn("ImpactGateValidationSquare", diagram_names)
        self.assertIn("ImpactMigrationPlanningSquare", diagram_names)
        self.assertIn("MigrationGateCoherenceSquare", diagram_names)
        self.assertIn("MigrationRehearsalSquare", diagram_names)
        self.assertIn("RehearsalReadinessValidationSquare", diagram_names)
        self.assertIn("RehearsalActivationSquare", diagram_names)
        self.assertIn("ActivationRollbackValidationSquare", diagram_names)
        self.assertIn("CenterRowRangeCompatibilitySquare", diagram_names)
        self.assertIn("RowRangeValidationSquare", diagram_names)
        self.assertIn("CenterArithmeticCompatibilitySquare", diagram_names)
        self.assertIn("ArithmeticRowRangeGluingSquare", diagram_names)
        self.assertIn("CenterConsoleIOCompatibilitySquare", diagram_names)
        self.assertIn("ConsoleIOOutputValidationSquare", diagram_names)
        self.assertIn("WordCompleterCompatibilitySquare", diagram_names)
        self.assertIn("WordCompletionValidationSquare", diagram_names)
        contract_names = {item["capsule"] for item in snapshot["capsule_contracts"]}
        self.assertIn("TableCoreCapsule", contract_names)
        self.assertIn("CategoricalMetaCapsule", contract_names)
        law_names = {item["name"] for item in snapshot["laws"]}
        self.assertIn("LegacyCompatibilityNaturalityLaw", law_names)
        self.assertIn("SheafGluingUniquenessLaw", law_names)
        self.assertIn("ArchitectureValidationCompletenessLaw", law_names)
        self.assertIn("ArchitectureTraceNavigationLaw", law_names)
        self.assertIn("ArchitectureBoundaryImportLaw", law_names)
        self.assertIn("ArchitectureImpactGateLaw", law_names)
        self.assertIn("ArchitectureMigrationOrderingLaw", law_names)
        self.assertIn("ArchitectureRehearsalReadinessLaw", law_names)
        self.assertIn("ArchitectureActivationCommitLaw", law_names)
        self.assertIn("ActivatedRowRangeLaw", law_names)
        self.assertIn("ActivatedArithmeticLaw", law_names)
        self.assertIn("ActivatedConsoleIOLaw", law_names)
        self.assertIn("ActivatedWordCompletionLaw", law_names)
        self.assertIn("architecture_contracts.py", snapshot["plan"]["implemented_in_stage_29"][0])
        self.assertIn("ArchitectureContractsBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_contracts.py").read_text(encoding="utf-8")
        self.assertIn("class CommutativeDiagramSpec", source)
        self.assertIn("class CapsuleContractSpec", source)
        self.assertIn("class RefactorLawSpec", source)
        self.assertIn("bootstrap_architecture_contracts", source)


    def test_architecture_witness_layer_is_explicit(self):
        witnesses = self.architecture.bootstrap_architecture_witnesses()
        snapshot = witnesses.snapshot()
        self.assertIsInstance(witnesses, ArchitectureWitnessBundle)
        self.assertEqual(snapshot["class"], "ArchitectureWitnessBundle")
        self.assertEqual(snapshot["stage"], 30)
        self.assertTrue(hasattr(self.architecture, "architecture_witnesses"))
        self.assertIn("witness", snapshot["paradigm"])
        self.assertIn("proof_obligation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["capsule_slices"], 10)
        self.assertGreaterEqual(snapshot["counts"]["diagram_witnesses"], 9)
        self.assertGreaterEqual(snapshot["counts"]["naturality_witnesses"], 9)
        self.assertGreaterEqual(snapshot["counts"]["obligations"], 19)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["missing_file_like_anchors"], [])
        slice_names = {item["capsule"] for item in snapshot["capsule_slices"]}
        self.assertIn("TableCoreCapsule", slice_names)
        self.assertIn("CategoricalMetaCapsule", slice_names)
        diagram_names = {item["diagram"] for item in snapshot["diagram_witnesses"]}
        self.assertIn("RawCommandNaturalitySquare", diagram_names)
        self.assertIn("LegacyArchitectureCompatibilitySquare", diagram_names)
        transformation_names = {item["transformation"] for item in snapshot["naturality_witnesses"]}
        self.assertIn("RawToCanonicalParameterTransformation", transformation_names)
        self.assertIn("ContractedNaturalityTransformation", transformation_names)
        self.assertIn("ContractWitnessValidationTransformation", transformation_names)
        self.assertIn("architecture_witnesses.py", snapshot["plan"]["implemented_in_stage_30"][0])
        self.assertIn("ArchitectureWitnessBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_witnesses.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureWitnessBundle", source)
        self.assertIn("class AnchorWitnessSpec", source)
        self.assertIn("class CapsuleSliceSpec", source)
        self.assertIn("bootstrap_architecture_witnesses", source)


    def test_architecture_validation_layer_is_explicit(self):
        validation = self.architecture.bootstrap_architecture_validation()
        snapshot = validation.snapshot()
        self.assertIsInstance(validation, ArchitectureValidationBundle)
        self.assertEqual(snapshot["class"], "ArchitectureValidationBundle")
        self.assertEqual(snapshot["stage"], 41)
        self.assertTrue(hasattr(self.architecture, "architecture_validation"))
        self.assertIn("validation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["checks"], 39)
        self.assertGreaterEqual(snapshot["counts"]["layers"], 14)
        self.assertEqual(snapshot["summary"]["status"], "passed")
        self.assertEqual(snapshot["summary"]["failed_items"], [])
        check_names = {item["name"] for item in snapshot["checks"]}
        self.assertIn("CategoryFunctorReferenceCheck", check_names)
        self.assertIn("NaturalTransformationWitnessCoverageCheck", check_names)
        self.assertIn("PackageIntegrityValidationCheck", check_names)
        self.assertIn("MarkdownStageHistoryCheck", check_names)
        self.assertIn("ArchitectureTraceValidationCheck", check_names)
        self.assertIn("BoundaryValidationStatusCheck", check_names)
        self.assertIn("ImpactValidationStatusCheck", check_names)
        self.assertIn("MigrationGateCoverageCheck", check_names)
        self.assertIn("MigrationValidationStatusCheck", check_names)
        self.assertIn("RehearsalValidationStatusCheck", check_names)
        self.assertIn("RehearsalGateCommandCoverageCheck", check_names)
        self.assertIn("ActivationValidationStatusCheck", check_names)
        self.assertIn("ActivationGateCoverageCheck", check_names)
        self.assertIn("ActivationRollbackCoverageCheck", check_names)
        self.assertIn("ActivationTransactionCoverageCheck", check_names)
        self.assertIn("MigrationGateBindingCoverageCheck", check_names)
        self.assertIn("RowRangeActivationStageCheck", check_names)
        self.assertIn("RowRangeCenterDelegationCheck", check_names)
        self.assertIn("RowRangeMorphismCoverageCheck", check_names)
        self.assertIn("RowRangeSampleExpansionCheck", check_names)
        self.assertIn("ArithmeticActivationStageCheck", check_names)
        self.assertIn("ArithmeticCenterDelegationCheck", check_names)
        self.assertIn("ArithmeticMorphismCoverageCheck", check_names)
        self.assertIn("ArithmeticSampleExpansionCheck", check_names)
        self.assertIn("ConsoleIOActivationStageCheck", check_names)
        self.assertIn("ConsoleIOCenterDelegationCheck", check_names)
        self.assertIn("ConsoleIOMorphismCoverageCheck", check_names)
        self.assertIn("ConsoleIOSampleExpansionCheck", check_names)
        self.assertIn("WordCompletionActivationStageCheck", check_names)
        self.assertIn("WordCompleterFacadeDelegationCheck", check_names)
        self.assertIn("WordCompletionMorphismCoverageCheck", check_names)
        self.assertIn("WordCompletionSampleExpansionCheck", check_names)
        self.assertIn("architecture_validation.py", snapshot["plan"]["implemented_in_stage_31"][0])
        self.assertIn("ArchitectureValidationBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureTraceBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureBoundariesBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureImpactBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureMigrationBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureRehearsalBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArchitectureActivationBundle", snapshot["diagrams"]["text"])
        self.assertIn("RowRangeMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("ArithmeticMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("ConsoleIOMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("WordCompletionMorphismBundle", snapshot["diagrams"]["text"])
        self.assertIn("WordCompletionMorphismBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_validation.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureValidationBundle", source)
        self.assertIn("class ArchitectureValidationCheckSpec", source)
        self.assertIn("bootstrap_architecture_validation", source)


    def test_architecture_coherence_layer_is_explicit(self):
        coherence = self.architecture.bootstrap_architecture_coherence()
        snapshot = coherence.snapshot()
        self.assertIsInstance(coherence, ArchitectureCoherenceBundle)
        self.assertEqual(snapshot["class"], "ArchitectureCoherenceBundle")
        self.assertEqual(snapshot["stage"], 41)
        self.assertTrue(hasattr(self.architecture, "architecture_coherence"))
        self.assertIn("coherence", snapshot["paradigm"])
        self.assertIn("natural_transformation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["capsule_coherence"], 11)
        self.assertGreaterEqual(snapshot["counts"]["functorial_routes"], 45)
        self.assertGreaterEqual(snapshot["counts"]["naturality_coherence"], 27)
        self.assertGreaterEqual(snapshot["counts"]["law_coherence"], 20)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["unresolved_functors"], [])
        capsule_names = {item["capsule"] for item in snapshot["capsule_coherence"]}
        self.assertIn("TableCoreCapsule", capsule_names)
        self.assertIn("CategoricalMetaCapsule", capsule_names)
        route_names = {item["functor_or_transformation"] for item in snapshot["functorial_routes"]}
        self.assertIn("RawToCanonicalParameterTransformation", route_names)
        self.assertIn("ContractWitnessValidationTransformation", route_names)
        self.assertIn("CoherenceToTraceFunctor", route_names)
        self.assertIn("CoherenceToBoundaryFunctor", route_names)
        self.assertIn("TraceBoundaryImpactFunctor", route_names)
        self.assertIn("ImpactGateValidationFunctor", route_names)
        self.assertIn("ImpactToMigrationPlanFunctor", route_names)
        self.assertIn("MigrationGateCoherenceFunctor", route_names)
        self.assertIn("MigrationStepRehearsalFunctor", route_names)
        self.assertIn("MigrationGateRehearsalFunctor", route_names)
        self.assertIn("RehearsalGateValidationFunctor", route_names)
        self.assertIn("RehearsalActivationFunctor", route_names)
        self.assertIn("GateActivationFunctor", route_names)
        self.assertIn("ActivationValidationFunctor", route_names)
        self.assertIn("RowRangeActivationFunctor", route_names)
        self.assertIn("CenterRowRangeCompatibilityFunctor", route_names)
        self.assertIn("RowRangeValidationFunctor", route_names)
        self.assertIn("ArithmeticActivationFunctor", route_names)
        self.assertIn("CenterArithmeticCompatibilityFunctor", route_names)
        self.assertIn("ArithmeticValidationFunctor", route_names)
        self.assertIn("ConsoleIOActivationFunctor", route_names)
        self.assertIn("CenterConsoleIOCompatibilityFunctor", route_names)
        self.assertIn("ConsoleIOValidationFunctor", route_names)
        self.assertIn("WordCompletionActivationFunctor", route_names)
        self.assertIn("LegacyWordCompleterCompatibilityFunctor", route_names)
        self.assertIn("WordCompletionValidationFunctor", route_names)
        naturality_names = {item["transformation"] for item in snapshot["naturality_coherence"]}
        self.assertIn("LegacyToArchitectureTransformation", naturality_names)
        self.assertIn("ContractWitnessValidationTransformation", naturality_names)
        self.assertIn("CoherenceToTraceTransformation", naturality_names)
        self.assertIn("CoherenceBoundaryValidationTransformation", naturality_names)
        self.assertIn("TraceBoundaryImpactTransformation", naturality_names)
        self.assertIn("ImpactGateValidationTransformation", naturality_names)
        self.assertIn("ImpactGateMigrationTransformation", naturality_names)
        self.assertIn("MigrationPlanCoherenceTransformation", naturality_names)
        self.assertIn("MigrationRehearsalNaturalityTransformation", naturality_names)
        self.assertIn("RehearsalReadinessValidationTransformation", naturality_names)
        self.assertIn("RehearsalActivationNaturalityTransformation", naturality_names)
        self.assertIn("ActivationRollbackValidationTransformation", naturality_names)
        self.assertIn("CenterRowRangeToArchitectureTransformation", naturality_names)
        self.assertIn("RowRangeValidationTransformation", naturality_names)
        self.assertIn("CenterArithmeticToArchitectureTransformation", naturality_names)
        self.assertIn("ArithmeticRowRangeGluingTransformation", naturality_names)
        self.assertIn("CenterConsoleIOToArchitectureTransformation", naturality_names)
        self.assertIn("ConsoleIOOutputValidationTransformation", naturality_names)
        self.assertIn("WordCompleterToArchitectureTransformation", naturality_names)
        self.assertIn("WordCompletionValidationTransformation", naturality_names)
        self.assertIn("architecture_coherence.py", snapshot["plan"]["implemented_in_stage_31"][0])
        source = (REPO_ROOT / "reta_architecture" / "architecture_coherence.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureCoherenceBundle", source)
        self.assertIn("class CapsuleCoherenceSpec", source)
        self.assertIn("bootstrap_architecture_coherence", source)


    def test_architecture_trace_layer_is_explicit(self):
        traces = self.architecture.bootstrap_architecture_traces()
        snapshot = traces.snapshot()
        self.assertIsInstance(traces, ArchitectureTraceBundle)
        self.assertEqual(snapshot["class"], "ArchitectureTraceBundle")
        self.assertEqual(snapshot["stage"], 32)
        self.assertTrue(hasattr(self.architecture, "architecture_traces"))
        self.assertIn("trace", snapshot["paradigm"])
        self.assertIn("natural_transformation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["component_traces"], 15)
        self.assertGreaterEqual(snapshot["counts"]["capsule_traces"], 11)
        self.assertEqual(snapshot["counts"]["stage_traces"], 42)
        self.assertGreaterEqual(snapshot["counts"]["route_hops"], 100)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["missing_component_traces"], [])
        self.assertEqual(snapshot["validation"]["missing_capsule_traces"], [])
        component_names = {item["legacy_owner"] for item in snapshot["component_traces"]}
        self.assertIn("reta.py", component_names)
        self.assertIn("libs/tableHandling.py", component_names)
        self.assertIn("reta_architecture/architecture_coherence.py", component_names)
        self.assertIn("reta_architecture/architecture_boundaries.py", component_names)
        self.assertIn("reta_architecture/architecture_impact.py", component_names)
        self.assertIn("reta_architecture/architecture_migration.py", component_names)
        self.assertIn("reta_architecture/architecture_rehearsal.py", component_names)
        self.assertIn("reta_architecture/architecture_activation.py", component_names)
        self.assertIn("reta_architecture/console_io.py", component_names)
        self.assertIn("libs/word_completerAlx.py", component_names)
        self.assertIn("reta_architecture/completion_word.py", component_names)
        self.assertIn("libs/nestedAlx.py", component_names)
        self.assertIn("reta_architecture/completion_nested.py", component_names)
        capsule_names = {item["capsule"] for item in snapshot["capsule_traces"]}
        self.assertIn("TableCoreCapsule", capsule_names)
        self.assertIn("CategoricalMetaCapsule", capsule_names)
        stage_numbers = {item["stage"] for item in snapshot["stage_traces"]}
        self.assertIn("Stage 27", stage_numbers)
        self.assertIn("Stage 32", stage_numbers)
        self.assertIn("Stage 33", stage_numbers)
        self.assertIn("Stage 34", stage_numbers)
        self.assertIn("Stage 35", stage_numbers)
        self.assertIn("Stage 36", stage_numbers)
        self.assertIn("Stage 37", stage_numbers)
        self.assertIn("Stage 38", stage_numbers)
        self.assertIn("Stage 40", stage_numbers)
        self.assertIn("Stage 41", stage_numbers)
        self.assertIn("ArchitectureTraceBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_traces.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureTraceBundle", source)
        self.assertIn("class TraceHopSpec", source)
        self.assertIn("bootstrap_architecture_traces", source)

    def test_architecture_boundary_layer_is_explicit(self):
        boundaries = self.architecture.bootstrap_architecture_boundaries()
        snapshot = boundaries.snapshot()
        self.assertIsInstance(boundaries, ArchitectureBoundariesBundle)
        self.assertEqual(snapshot["class"], "ArchitectureBoundariesBundle")
        self.assertEqual(snapshot["stage"], 32)
        self.assertTrue(hasattr(self.architecture, "architecture_boundaries"))
        self.assertIn("boundary", snapshot["paradigm"])
        self.assertIn("morphism", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["module_ownership"], 30)
        self.assertGreaterEqual(snapshot["counts"]["import_edges"], 40)
        self.assertGreaterEqual(snapshot["counts"]["capsule_edges"], 8)
        self.assertGreaterEqual(snapshot["counts"]["checks"], 4)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["missing_capsule_boundaries"], [])
        owned_paths = {item["path"] for item in snapshot["module_ownership"]}
        self.assertIn("reta_architecture/category_theory.py", owned_paths)
        self.assertIn("reta_architecture/architecture_traces.py", owned_paths)
        self.assertIn("reta_architecture/architecture_boundaries.py", owned_paths)
        self.assertIn("reta_architecture/architecture_impact.py", owned_paths)
        self.assertIn("reta_architecture/architecture_migration.py", owned_paths)
        self.assertIn("reta_architecture/architecture_rehearsal.py", owned_paths)
        self.assertIn("reta_architecture/architecture_activation.py", owned_paths)
        self.assertIn("reta_architecture/row_ranges.py", owned_paths)
        boundary_capsules = {item["capsule"] for item in snapshot["capsule_boundaries"]}
        self.assertIn("CategoricalMetaCapsule", boundary_capsules)
        check_names = {item["name"] for item in snapshot["validation"]["checks"]}
        self.assertIn("ModuleOwnershipCoverageCheck", check_names)
        self.assertIn("ImportEdgeClassificationCheck", check_names)
        self.assertIn("ArchitectureBoundariesBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_boundaries.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureBoundariesBundle", source)
        self.assertIn("class ModuleOwnershipSpec", source)
        self.assertIn("bootstrap_architecture_boundaries", source)


    def test_architecture_impact_layer_is_explicit(self):
        impact = self.architecture.bootstrap_architecture_impact()
        snapshot = impact.snapshot()
        self.assertIsInstance(impact, ArchitectureImpactBundle)
        self.assertEqual(snapshot["class"], "ArchitectureImpactBundle")
        self.assertEqual(snapshot["stage"], 33)
        self.assertTrue(hasattr(self.architecture, "architecture_impact"))
        self.assertIn("impact", snapshot["paradigm"])
        self.assertIn("migration_gate", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["impact_sources"], 20)
        self.assertGreaterEqual(snapshot["counts"]["impact_contracts"], 20)
        self.assertGreaterEqual(snapshot["counts"]["regression_gates"], 10)
        self.assertGreaterEqual(snapshot["counts"]["migration_candidates"], 20)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["missing_sources"], [])
        gate_names = {item["name"] for item in snapshot["regression_gates"]}
        self.assertIn("ArchitectureImpactSelfGate", gate_names)
        self.assertIn("CommandParityGate", gate_names)
        candidate_owners = {item["legacy_owner"] for item in snapshot["migration_candidates"]}
        self.assertIn("reta.py", candidate_owners)
        self.assertIn("libs/tableHandling.py", candidate_owners)
        self.assertIn("reta_architecture/architecture_impact.py", candidate_owners)
        self.assertIn("reta_architecture/architecture_migration.py", candidate_owners)
        self.assertIn("reta_architecture/architecture_rehearsal.py", candidate_owners)
        self.assertIn("reta_architecture/architecture_activation.py", candidate_owners)
        self.assertIn("reta_architecture/row_ranges.py", candidate_owners)
        source_names = {item["source"] for item in snapshot["impact_sources"]}
        self.assertIn("reta.py", source_names)
        self.assertIn("reta_architecture/architecture_impact.py", source_names)
        self.assertIn("reta_architecture/architecture_migration.py", source_names)
        self.assertIn("reta_architecture/architecture_rehearsal.py", source_names)
        self.assertIn("reta_architecture/architecture_activation.py", source_names)
        self.assertIn("reta_architecture/row_ranges.py", source_names)
        self.assertIn("ArchitectureImpactBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_impact.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureImpactBundle", source)
        self.assertIn("class MigrationCandidateSpec", source)
        self.assertIn("bootstrap_architecture_impact", source)


    def test_architecture_migration_layer_is_explicit(self):
        migration = self.architecture.bootstrap_architecture_migration()
        snapshot = migration.snapshot()
        self.assertIsInstance(migration, ArchitectureMigrationBundle)
        self.assertEqual(snapshot["class"], "ArchitectureMigrationBundle")
        self.assertEqual(snapshot["stage"], 34)
        self.assertTrue(hasattr(self.architecture, "architecture_migration"))
        self.assertIn("migration_plan", snapshot["paradigm"])
        self.assertIn("natural_transformation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["waves"], 7)
        self.assertGreaterEqual(snapshot["counts"]["steps"], 20)
        self.assertGreaterEqual(snapshot["counts"]["gate_bindings"], 20)
        self.assertGreaterEqual(snapshot["counts"]["invariants"], 7)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["missing_candidates"], [])
        self.assertEqual(snapshot["validation"]["unknown_gates"], [])
        self.assertEqual(snapshot["validation"]["unknown_diagrams"], [])
        wave_ids = {item["wave_id"] for item in snapshot["waves"]}
        for wave_id in {"M0", "M1", "M2", "M3", "M4", "M5", "M6"}:
            self.assertIn(wave_id, wave_ids)
        step_owners = {item["legacy_owner"] for item in snapshot["steps"]}
        self.assertIn("reta.py", step_owners)
        self.assertIn("libs/tableHandling.py", step_owners)
        self.assertIn("reta_architecture/architecture_migration.py", step_owners)
        self.assertIn("reta_architecture/architecture_rehearsal.py", step_owners)
        self.assertIn("reta_architecture/architecture_activation.py", step_owners)
        self.assertIn("reta_architecture/row_ranges.py", step_owners)
        gate_names = {gate for item in snapshot["gate_bindings"] for gate in item["gates"]}
        self.assertIn("ArchitectureMigrationSelfGate", gate_names)
        self.assertIn("CommandParityGate", gate_names)
        self.assertIn("ArchitectureMigrationBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_migration.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureMigrationBundle", source)
        self.assertIn("class MigrationGateBindingSpec", source)
        self.assertIn("bootstrap_architecture_migration", source)


    def test_architecture_rehearsal_layer_is_explicit(self):
        rehearsal = self.architecture.bootstrap_architecture_rehearsal()
        snapshot = rehearsal.snapshot()
        self.assertIsInstance(rehearsal, ArchitectureRehearsalBundle)
        self.assertEqual(snapshot["class"], "ArchitectureRehearsalBundle")
        self.assertEqual(snapshot["stage"], 35)
        self.assertTrue(hasattr(self.architecture, "architecture_rehearsal"))
        self.assertIn("rehearsal", snapshot["paradigm"])
        self.assertIn("natural_transformation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["open_sets"], 7)
        self.assertGreaterEqual(snapshot["counts"]["moves"], 22)
        self.assertGreaterEqual(snapshot["counts"]["gate_rehearsals"], 22)
        self.assertGreaterEqual(snapshot["counts"]["covers"], 7)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["rehearsals_without_gate_suite"], [])
        self.assertEqual(snapshot["validation"]["open_sets_without_cover"], [])
        self.assertEqual(snapshot["validation"]["unknown_diagrams"], [])
        self.assertEqual(snapshot["validation"]["unknown_laws"], [])
        self.assertEqual(snapshot["validation"]["unknown_natural_transformations"], [])
        open_set_ids = {item["open_set_id"] for item in snapshot["open_sets"]}
        for open_set_id in {"REH35-OPEN-M0", "REH35-OPEN-M1", "REH35-OPEN-M2", "REH35-OPEN-M3", "REH35-OPEN-M4", "REH35-OPEN-M5", "REH35-OPEN-M6"}:
            self.assertIn(open_set_id, open_set_ids)
        move_owners = {item["legacy_owner"] for item in snapshot["moves"]}
        self.assertIn("reta.py", move_owners)
        self.assertIn("libs/tableHandling.py", move_owners)
        self.assertIn("reta_architecture/architecture_rehearsal.py", move_owners)
        self.assertIn("reta_architecture/architecture_activation.py", move_owners)
        self.assertIn("reta_architecture/row_ranges.py", move_owners)
        gate_names = {gate for item in snapshot["gate_rehearsals"] for gate in item["gates"]}
        self.assertIn("ArchitectureRehearsalSelfGate", gate_names)
        self.assertIn("CommandParityGate", gate_names)
        self.assertIn("ArchitectureRehearsalBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_rehearsal.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureRehearsalBundle", source)
        self.assertIn("class RehearsalMoveSpec", source)
        self.assertIn("bootstrap_architecture_rehearsal", source)


    def test_architecture_activation_layer_is_explicit(self):
        activation = self.architecture.bootstrap_architecture_activation()
        snapshot = activation.snapshot()
        self.assertIsInstance(activation, ArchitectureActivationBundle)
        self.assertEqual(snapshot["class"], "ArchitectureActivationBundle")
        self.assertEqual(snapshot["stage"], 36)
        self.assertTrue(hasattr(self.architecture, "architecture_activation"))
        self.assertIn("activation", snapshot["paradigm"])
        self.assertIn("rollback_section", snapshot["paradigm"])
        self.assertIn("natural_transformation", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["windows"], 7)
        self.assertGreaterEqual(snapshot["counts"]["units"], 23)
        self.assertGreaterEqual(snapshot["counts"]["gates"], 23)
        self.assertGreaterEqual(snapshot["counts"]["rollbacks"], 23)
        self.assertGreaterEqual(snapshot["counts"]["transactions"], 7)
        self.assertEqual(snapshot["validation"]["status"], "passed")
        self.assertEqual(snapshot["validation"]["missing_rehearsal_moves"], [])
        self.assertEqual(snapshot["validation"]["activations_without_gate"], [])
        self.assertEqual(snapshot["validation"]["activations_without_rollback"], [])
        self.assertEqual(snapshot["validation"]["windows_without_transaction"], [])
        self.assertEqual(snapshot["validation"]["unknown_diagrams"], [])
        self.assertEqual(snapshot["validation"]["unknown_laws"], [])
        self.assertEqual(snapshot["validation"]["unknown_natural_transformations"], [])
        window_ids = {item["window_id"] for item in snapshot["windows"]}
        for window_id in {"ACT36-WINDOW-M0", "ACT36-WINDOW-M1", "ACT36-WINDOW-M2", "ACT36-WINDOW-M3", "ACT36-WINDOW-M4", "ACT36-WINDOW-M5", "ACT36-WINDOW-M6"}:
            self.assertIn(window_id, window_ids)
        unit_owners = {item["legacy_owner"] for item in snapshot["units"]}
        self.assertIn("reta.py", unit_owners)
        self.assertIn("libs/tableHandling.py", unit_owners)
        self.assertIn("reta_architecture/architecture_activation.py", unit_owners)
        self.assertIn("reta_architecture/row_ranges.py", unit_owners)
        gate_names = {gate for item in snapshot["gates"] for gate in item["gates"]}
        self.assertIn("ArchitectureActivationSelfGate", gate_names)
        self.assertIn("CommandParityGate", gate_names)
        transaction_statuses = {item["status"] for item in snapshot["transactions"]}
        self.assertEqual(transaction_statuses, {"transaction_ready"})
        self.assertIn("ArchitectureActivationBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_activation.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureActivationBundle", source)
        self.assertIn("class ActivationUnitSpec", source)
        self.assertIn("bootstrap_architecture_activation", source)


    def test_tag_schema_owner_is_explicit(self):
        from reta_architecture.tag_schema import ST

        tag_schema = bootstrap_tag_schema()
        snapshot = tag_schema.snapshot()
        self.assertIsInstance(tag_schema, TagSchemaBundle)
        self.assertEqual(snapshot["class"], "TagSchemaBundle")
        self.assertEqual(snapshot["stage"], 42)
        self.assertEqual(snapshot["legacy_owner"], "libs/lib4tables_Enum.py")
        self.assertGreaterEqual(snapshot["counts"]["primary_tag_groups"], 19)
        self.assertIn("sternPolygon", snapshot["tag_names"])
        reverse_tags = frozenset({ST.sternPolygon, ST.galaxie})
        explicit_group = frozenset({
            ST.universum,
            ST.keinParaOdMetaP,
            ST.sternPolygon,
            ST.gleichfoermigesPolygon,
        })
        self.assertEqual(tag_schema.tags_for_column(14), reverse_tags)
        self.assertIn(14, tag_schema.columns_for_tags(explicit_group))
        legacy_source = (REPO_ROOT / "libs" / "lib4tables_Enum.py").read_text(encoding="utf-8")
        arch_source = (REPO_ROOT / "reta_architecture" / "tag_schema.py").read_text(encoding="utf-8")
        self.assertIn("reta_architecture.tag_schema", legacy_source)
        self.assertIn("compatibility facade", legacy_source.lower())
        self.assertIn("class TagSchemaBundle", arch_source)
        self.assertIn("bootstrap_tag_schema", arch_source)


    def test_architecture_progress_layer_is_explicit(self):
        progress = self.architecture.bootstrap_architecture_progress()
        snapshot = progress.snapshot()
        self.assertIsInstance(progress, ArchitectureProgressBundle)
        self.assertEqual(snapshot["class"], "ArchitectureProgressBundle")
        self.assertEqual(snapshot["stage"], 42)
        self.assertTrue(hasattr(self.architecture, "architecture_progress"))
        self.assertIn("progress_overlay", snapshot["paradigm"])
        self.assertIn("compatibility_facade", snapshot["paradigm"])
        self.assertGreaterEqual(snapshot["counts"]["surfaces"], 25)
        self.assertGreaterEqual(snapshot["counts"]["step_progress"], 30)
        self.assertGreaterEqual(snapshot["counts"]["wave_progress"], 7)
        self.assertEqual(snapshot["validation"]["steps_without_surface"], [])
        self.assertEqual(snapshot["validation"]["inconsistent_wave_counts"], [])
        surface_owners = {item["owner"] for item in snapshot["surfaces"]}
        self.assertIn("reta.py", surface_owners)
        self.assertIn("libs/center.py", surface_owners)
        self.assertIn("libs/lib4tables_Enum.py", surface_owners)
        enum_surface = progress.surface_named("libs/lib4tables_Enum.py").snapshot()
        self.assertEqual(enum_surface["execution_status"], "extracted_to_compatibility_facade")
        self.assertEqual(snapshot["validation"]["mixed_owners"], [])
        wave_ids = {item["wave_id"] for item in snapshot["wave_progress"]}
        for wave_id in {"M0", "M1", "M2", "M3", "M4", "M5", "M6"}:
            self.assertIn(wave_id, wave_ids)
        remaining_titles = {item["title"] for item in snapshot["outstanding_work"]}
        self.assertNotIn("Extract remaining tag/data owner from lib4tables_Enum", remaining_titles)
        if (REPO_ROOT / ".git").exists() or Path("/mnt/data/reta.todel.zip").exists():
            self.assertEqual(snapshot["counts"]["outstanding_work"], 0)
            self.assertEqual(snapshot["validation"]["status"], "passed")
            self.assertEqual(remaining_titles, set())
        else:
            self.assertGreaterEqual(snapshot["counts"]["outstanding_work"], 1)
            self.assertEqual(snapshot["validation"]["status"], "attention")
            self.assertIn("Restore original reference archive for command parity", remaining_titles)
        self.assertIn("ArchitectureProgressBundle", snapshot["diagrams"]["text"])
        source = (REPO_ROOT / "reta_architecture" / "architecture_progress.py").read_text(encoding="utf-8")
        self.assertIn("class ArchitectureProgressBundle", source)
        self.assertIn("class LegacySurfaceProgressSpec", source)
        self.assertIn("bootstrap_architecture_progress", source)


    def test_known_pair_lookup_still_resolves(self):
        pair = self.architecture.sheaves.parameter_semantics.canonicalize_pair(
            "Religionen", "Hinduismus"
        )
        self.assertEqual(pair, ("Religionen", "Hinduismus"))
        self.assertEqual(
            self.architecture.sheaves.parameter_semantics.column_numbers_for_pair(
                "Religionen", "Hinduismus"
            ),
            [217],
        )


if __name__ == "__main__":
    unittest.main()
