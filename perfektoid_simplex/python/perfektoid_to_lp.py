from typing import List

def perfektoid_to_lp(p: int, k0: int, a: List[int]):
    n = len(a)

    objective = [k0 + i for i in range(n)]
    bounds = [(0, p-1) for _ in range(n)]

    constraints = []
    for i in range(n - p):
        row = [0.0] * n
        row[i] = -1.0
        row[i + p] = 1.0
        constraints.append((row, 0.0))

    return objective, constraints, bounds
