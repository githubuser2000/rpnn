# --- Importiere viele Pip-Pakete ---
import numpy as np
import pandas as pd
import sympy as sp
import itertools
import math
import functools
import operator
import networkx as nx
import matplotlib.pyplot as plt
import seaborn as sns
from tqdm import tqdm
from rich import print
from more_itertools import distinct_permutations, powerset
from typing import List

# --- Funktion: Alle Partitionen einer Zahl ---
def all_partitions(n: int) -> List[List[int]]:
    """Alle möglichen Partitionen einer Zahl n in positive ganze Zahlen"""
    if n == 0:
        return [[0]]
    result = []
    # Rekursion über erste Zahl
    for i in range(1, n + 1):
        for tail in all_partitions(n - i):
            if tail == [0]:
                result.append([i])
            else:
                result.append([i] + tail)
    return result

# --- Perfektoide Zerlegung einer 3-adischen Zahl ---
def perfektoid_arrays_3adic(digits: List[int]) -> List[List[int]]:
    """
    digits: Ziffern einer 3-adischen Zahl von rechts nach links
    Rückgabe: Liste aller perfektoiden Arrays
    """
    result = [[0]]  # Start mit Null
    for i, d in enumerate(digits):
        power = 3**i
        new_result = []
        for partition in all_partitions(d):
            scaled = [x * power for x in partition]
            for r in result:
                new_result.append(r + scaled)
        result = new_result
    return result

# --- Beispiel: 3-adische Zahl …210 ---
digits = [0, 1, 2]  # rechts nach links
arrays = perfektoid_arrays_3adic(digits)

# --- Ausgabe mit rich für bessere Formatierung ---
print("[bold green]Perfektoide Arrays für 3-adische Zahl …210:[/bold green]")
for arr in arrays[:20]:  # nur die ersten 20 zur Übersicht
    print(arr)

# Optional: Pandas DataFrame für bessere Übersicht
df = pd.DataFrame(arrays, columns=[f'Teil{i+1}' for i in range(max(len(a) for a in arrays))])
df.fillna(0, inplace=True)
df.to_csv("perfektoide_arrays.csv", index=False)
print("[bold blue]CSV-Datei 'perfektoide_arrays.csv' gespeichert![/bold blue]")
