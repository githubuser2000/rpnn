from __future__ import annotations

from dataclasses import dataclass, field
from itertools import zip_longest
from typing import Any, Callable, Dict, Iterable, List, Mapping, MutableMapping, Optional, Sequence, Tuple

try:
    from orderedset import OrderedSet
except Exception:  # pragma: no cover - reta already tolerates this fallback
    OrderedSet = set  # type: ignore[assignment]

from .schema import RetaContextSchema
from .universal import merge_parameter_dicts


@dataclass
class ParameterSemanticsBuildResult:
    para_main_dict: Dict[str, tuple]
    para_dict: Dict[Tuple[str, str], object]
    data_dict: List[dict]
    para_n_data_matrix: List[tuple]
    kombi_para_n_data_matrix: object
    kombi_para_n_data_matrix2: object
    kombi_reverse_dict: Dict[str, int]
    kombi_reverse_dict2: Dict[str, int]
    all_simple_command_columns: set
    all_values: List[set] = field(default_factory=list)


class ParameterSemanticsBuilder:
    """Build reta's canonical parameter/data dictionaries from the schema layer.

    This extracts the heavy parameter-globalisation logic from `reta.py` while
    preserving the legacy semantics.
    """

    def __init__(
        self,
        schema: RetaContextSchema,
        *,
        gebrochen_spalten_maximum_plus1: int,
        invert_alles: bool,
        initial_data_dict: Optional[Sequence[dict]] = None,
        prim_number_predicate: Callable[[int], int],
        alles_parameter_names: Sequence[str],
    ) -> None:
        self.schema = schema
        self.gebrochen_spalten_maximum_plus1 = int(gebrochen_spalten_maximum_plus1)
        self.invert_alles = bool(invert_alles)
        self.initial_data_dict = [dict(d) for d in (initial_data_dict or ())]
        self.prim_number_predicate = prim_number_predicate
        self.alles_parameter_names = tuple(str(v) for v in alles_parameter_names)

    def allowed_prim_numbers_for_command(self) -> Tuple[str, ...]:
        return tuple(
            str(num)
            for num in tuple(
                OrderedSet(
                    (
                        num if self.prim_number_predicate(num) == 1 else None
                        for num in range(2, 32)
                    )
                )
                - {None}
            )
        )

    def build_reverse_lookup(self, mapping) -> Dict[str, int]:
        reverse: Dict[str, int] = {}
        if mapping is None:
            return reverse
        for key, value in mapping.items():
            for values_in_values in value:
                reverse[values_in_values] = key
        return reverse

    def collect_all_values(self, para_n_data_matrix: Sequence[tuple]) -> List[set]:
        all_values = [
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
        ]
        for possible_commands in para_n_data_matrix:
            for command_value, a_all_value in zip(possible_commands[2:], all_values):
                try:
                    a_all_value |= command_value
                except TypeError:
                    raise ValueError(command_value)

        all_simple_command_columns = set(all_values[0])
        if self.invert_alles and all_values[0]:
            all_values[0] = (
                set(range(max(all_values[0])))
                - set(all_values[0])
                - {a[0] for a in all_values[1]}
                - {a[1] for a in all_values[1]}
            )

        all_values[2] = set((int(p_num) for p_num in self.allowed_prim_numbers_for_command()))
        all_values[3] = set((self.schema.kombi_para_n_data_matrix or {}).keys())
        all_values[5] = set(range(2, self.gebrochen_spalten_maximum_plus1))
        all_values[6] = set(range(2, self.gebrochen_spalten_maximum_plus1))
        all_values[8] = set((self.schema.kombi_para_n_data_matrix2 or {}).keys())
        all_values[9] = set(range(2, self.gebrochen_spalten_maximum_plus1))
        all_values[10] = set(range(2, self.gebrochen_spalten_maximum_plus1))

        if self.invert_alles:
            for zahl in range(1, 11):
                all_values[zahl] = set()
        return [set(v) for v in all_values], all_simple_command_columns

    def into_parameter_datatype(
        self,
        parameter_main_names: Sequence[str],
        parameter_names: Sequence[str],
        datas: Sequence[Iterable[Any]],
    ) -> Tuple[dict, dict, tuple]:
        para_main_dict = {}
        for name in parameter_main_names:
            para_main_dict[name] = tuple(parameter_names)
        para_dict = {}
        for name1 in parameter_main_names:
            for name2 in parameter_names:
                para_dict[(name1, name2)] = datas
            if len(parameter_names) == 0:
                para_dict[(name1, "")] = datas
        data_dicts: tuple = ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})

        for i, d in enumerate(datas):
            for spalten_nummer_oder_etc in d:
                into = []
                parameter_main_name_per_loop = []
                case: Optional[int] = None

                for parameter_main_name in parameter_main_names:
                    for parameter_name in (parameter_names if len(parameter_names) > 0 else ("",)):
                        if i == 4 and (
                            type(spalten_nummer_oder_etc) is bool
                            or (
                                type(spalten_nummer_oder_etc) in [tuple, list]
                                and len(spalten_nummer_oder_etc) > 0
                                and type(spalten_nummer_oder_etc[0]) is bool
                            )
                        ):
                            case = 1
                            into += [(parameter_main_name, parameter_name)]
                        elif i in (5, 6, 9, 10):
                            case = 2
                            into += [[(parameter_main_name, parameter_name)]]
                            parameter_main_name_per_loop += [parameter_name]
                        elif i == 2 and callable(spalten_nummer_oder_etc):
                            case = 2
                            parameter_main_name_per_loop += [parameter_name]
                            into += [[(parameter_main_name, parameter_name)]]
                        elif i == 4 and (type(spalten_nummer_oder_etc) in (list, tuple)):
                            case = 4
                            into += [(parameter_main_name, parameter_name)]
                        elif i == 4 and (type(spalten_nummer_oder_etc) in (set,)):
                            case = 4
                            into += [(parameter_main_name, parameter_name)]
                            # Do not mutate the schema/all-values set while building a
                            # local data-dictionary entry.  The old ``pop`` made repeated
                            # semantic builds order-dependent after the prompt runtime had
                            # already touched the same schema object.
                            spalten_nummer_oder_etc = next(iter(spalten_nummer_oder_etc))
                        else:
                            case = 3
                            try:
                                into += [(parameter_main_name, parameter_name)]
                            except KeyError:
                                into = [(parameter_main_name, parameter_name)]

                index1 = i if case != 1 else 3
                index2a = (
                    spalten_nummer_oder_etc
                    if case == 3
                    else (
                        spalten_nummer_oder_etc
                        if case == 4
                        else ("bool", 0)
                        if case == 1
                        else tuple(
                            (
                                int(para)
                                if para.isdecimal()
                                else para
                                if len(parameter_names) > 0
                                else None
                                for para in parameter_main_name_per_loop
                            )
                        )
                        if case == 2
                        else None
                    )
                )
                into_a = into if case == 2 else (into,)
                for index2, into2 in zip_longest(
                    index2a if case == 2 else (index2a,), into_a, fillvalue=into
                ):
                    try:
                        data_dicts[index1][index2] += (
                            (into2,) if into2 not in data_dicts[index1][index2] else ()
                        )
                    except KeyError:
                        data_dicts[index1][index2] = (into2,)
        return para_main_dict, para_dict, data_dicts

    def build(self) -> ParameterSemanticsBuildResult:
        para_n_data_matrix = list(self.schema.para_n_data_matrix)
        kombi_para_n_data_matrix = self.schema.kombi_para_n_data_matrix
        kombi_para_n_data_matrix2 = self.schema.kombi_para_n_data_matrix2
        kombi_reverse_dict = self.build_reverse_lookup(kombi_para_n_data_matrix)
        kombi_reverse_dict2 = self.build_reverse_lookup(kombi_para_n_data_matrix2)
        all_values, all_simple_command_columns = self.collect_all_values(para_n_data_matrix)

        para_n_data_matrix = para_n_data_matrix + [
            (
                self.alles_parameter_names,
                (),
                *all_values,
            )
        ]

        para_main_dict: Dict[str, tuple] = {}
        para_dict: Dict[Tuple[str, str], object] = {}
        data_dict: List[dict] = [dict(d) for d in self.initial_data_dict]
        if not data_dict:
            data_dict = [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}]

        for parameter_entry in para_n_data_matrix:
            into = self.into_parameter_datatype(
                parameter_entry[0],
                parameter_entry[1],
                tuple(parameter_entry_element for parameter_entry_element in parameter_entry[2:]),
            )
            para_main_dict.update(into[0])
            para_dict, data_dict = merge_parameter_dicts(
                para_main_dict,
                para_dict,
                data_dict,
                *into,
            )

        data_dict[3] = kombi_para_n_data_matrix
        data_dict[8] = kombi_para_n_data_matrix2

        return ParameterSemanticsBuildResult(
            para_main_dict=para_main_dict,
            para_dict=para_dict,
            data_dict=data_dict,
            para_n_data_matrix=para_n_data_matrix,
            kombi_para_n_data_matrix=kombi_para_n_data_matrix,
            kombi_para_n_data_matrix2=kombi_para_n_data_matrix2,
            kombi_reverse_dict=kombi_reverse_dict,
            kombi_reverse_dict2=kombi_reverse_dict2,
            all_simple_command_columns=all_simple_command_columns,
            all_values=all_values,
        )
