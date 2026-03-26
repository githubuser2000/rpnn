from math import gcd
from functools import reduce
import sympy

def prime_factors(n):
    """Primfaktorzerlegung als Dictionary {p: exponent}"""
    return sympy.factorint(n)

def lcm(a, b):
    return abs(a*b) // gcd(a, b)

def gcd_list(nums):
    return reduce(gcd, nums)

def lcm_list(nums):
    return reduce(lcm, nums)

def perfeqize_product(numbers):
    """Erzeugt eine 'perfektoide Zahl' aus ganzen Zahlen"""
    factor_lists = [prime_factors(n) for n in numbers]

    # Alle Primzahlen sammeln
    all_primes = set()
    for f in factor_lists:
        all_primes.update(f.keys())

    # ggT und kgV Exponenten pro Primzahl
    min_exp = {}
    max_exp = {}
    for p in all_primes:
        exps = [f.get(p, 0) for f in factor_lists]
        min_exp[p] = min(exps)
        max_exp[p] = max(exps)

    # Fertige p-adische/perfektoide Zahl als Produktform
    # Beispiel: 2^min 3^min ... bis 2^max 3^max ...
    # Wir erzeugen ein Dictionary: {"ggT": ..., "kgV": ...}
    ggT = 1
    kgV = 1
    for p in all_primes:
        ggT *= p ** min_exp[p]
        kgV *= p ** max_exp[p]

    # Perfektoide Zahl symbolisch als Projektivlimit
    perfektoid = {p: f"{p}^{min_exp[p]}..{p}^{max_exp[p]}" for p in all_primes}

    return {
        "numbers": numbers,
        "ggT": ggT,
        "kgV": kgV,
        "perfektoid_product_form": perfektoid
    }

# Beispiel
numbers = [12, 18, 30]
perfektoid = perfeqize_product(numbers)
from pprint import pprint
pprint(perfektoid)
