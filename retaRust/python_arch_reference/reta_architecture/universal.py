from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
from typing import Dict, Iterable, List, Mapping, MutableMapping, Optional, Sequence, Tuple


def merge_parameter_dicts(
    para_main_dict1: Mapping,
    para_dict1: Mapping,
    data_dicts1: Sequence[dict],
    para_main_dict2: Mapping,
    para_dict2: Mapping,
    data_dicts2: Sequence[dict],
) -> Tuple[dict, list]:
    """Canonical pushout-like merge for reta parameter semantics."""
    merged_para_dict = {**dict(para_dict1), **dict(para_dict2)}
    # Keep the accumulated dictionaries structurally shared during the builder
    # fold, but deepcopy every newly merged value.  The old implementation
    # deep-copied the whole accumulated dictionary graph on every parameter-family
    # merge, which made bootstrap unnecessarily slow; copying only incoming values
    # preserves non-aliasing from the current local section without repeatedly
    # cloning the complete history.
    merged_data_dicts = [dict(d) for d in data_dicts1]
    max_len = max(len(data_dicts1), len(data_dicts2))
    while len(merged_data_dicts) < max_len:
        merged_data_dicts.append({})
    for index in range(max_len):
        dict1 = dict(data_dicts1[index]) if index < len(data_dicts1) else None
        dict2 = dict(data_dicts2[index]) if index < len(data_dicts2) else None
        if isinstance(dict1, dict) and isinstance(dict2, dict):
            if len(merged_data_dicts[index].keys()) == 0:
                merged_data_dicts[index] = {key: deepcopy(value) for key, value in dict2.items()}
            else:
                for key2, value2 in dict2.items():
                    value2_copy = deepcopy(value2)
                    if key2 in merged_data_dicts[index]:
                        try:
                            merged_data_dicts[index][key2] += value2_copy
                        except TypeError:
                            if merged_data_dicts[index][key2] != value2_copy:
                                merged_data_dicts[index][key2] = value2_copy
                    else:
                        merged_data_dicts[index][key2] = value2_copy
        elif isinstance(dict1, dict) and dict2 is None:
            merged_data_dicts[index] = dict1
        elif dict1 is None and isinstance(dict2, dict):
            merged_data_dicts[index] = {key: deepcopy(value) for key, value in dict2.items()}
    return merged_para_dict, merged_data_dicts


def normalize_column_buckets(spalten_arten: Mapping[Tuple[int, int], set]) -> Dict[Tuple[int, int], set]:
    """Normalise positive/negative column buckets.

    This is the old `spalten_removeDoublesNthenRemoveOneFromAnother`, made into a
    named universal construction.  Large bucket payloads may be normalised in
    process chunks on PyPy3, but the returned mapping is assembled serially here
    so the universal construction remains deterministic.
    """
    try:
        from .parallel_execution import normalize_column_buckets_in_processes

        parallel_result = normalize_column_buckets_in_processes(spalten_arten)
        if parallel_result is not None:
            return parallel_result.values
    except Exception:
        pass

    buckets = {key: set(value) for key, value in spalten_arten.items()}
    max_type = int(len(buckets) / 2)
    for bucket_type in range(max_type):
        positive_key = (0, bucket_type)
        negative_key = (1, bucket_type)
        if positive_key in buckets and negative_key in buckets:
            buckets[positive_key] -= buckets[positive_key] & buckets[negative_key]
    for bucket_type in range(max_type):
        positive_key = (0, bucket_type)
        negative_key = (1, bucket_type)
        if positive_key in buckets and negative_key in buckets:
            buckets[positive_key] -= buckets[negative_key]
            buckets.pop(negative_key, None)
    return buckets


def sync_generated_columns_from_tables(tables, generated_columns_sheaf) -> None:
    generated_columns_sheaf.sync_from_tables(tables)


def sync_output_section_from_tables(
    tables,
    table_output_sheaf,
    output_mode: str,
    finally_display_lines=None,
    rows_range=None,
) -> None:
    table_output_sheaf.sync_from_tables(
        tables,
        output_mode=output_mode,
        finally_display_lines=finally_display_lines,
        rows_range=rows_range,
    )


@dataclass
class UniversalBundle:
    sheaves: object

    def merge_parameter_dicts(self, *args, **kwargs):
        return merge_parameter_dicts(*args, **kwargs)

    def normalize_column_buckets(self, spalten_arten):
        return normalize_column_buckets(spalten_arten)

    def sync_tables(self, tables, output_mode: Optional[str] = None, finally_display_lines=None, rows_range=None) -> None:
        sync_generated_columns_from_tables(tables, self.sheaves.generated_columns)
        if output_mode is not None:
            sync_output_section_from_tables(
                tables,
                self.sheaves.table_output,
                output_mode=output_mode,
                finally_display_lines=finally_display_lines,
                rows_range=rows_range,
            )

    def snapshot(self):
        return {
            "available": [
                "merge_parameter_dicts",
                "normalize_column_buckets",
                "sync_tables",
            ]
        }
