from math import gcd
from functools import reduce
from collections import defaultdict
import sympy

def prime_factors(n):
    """Primfaktorzerlegung von n als Dictionary {p: exponent}"""
    factors = sympy.factorint(n)
    return factors

def lcm(a, b):
    return abs(a*b) // gcd(a, b)

def gcd_list(nums):
    return reduce(gcd, nums)

def lcm_list(nums):
    return reduce(lcm, nums)

def perfeqize(numbers):
    """Erzeugt die 'perfektoide' Darstellung einer Menge ganzer Zahlen"""
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
    
    # p-adische Komponenten simuliert als Listen von Potenzen
    p_adic_components = {}
    for p in all_primes:
        p_adic_components[p] = [p**e for e in range(max_exp[p]+1)]
    
    return {
        "numbers": numbers,
        "prime_factors": factor_lists,
        "ggT_exponents": min_exp,
        "kgV_exponents": max_exp,
        "p_adics": p_adic_components
    }

# Beispiel
numbers = [12, 18, 30]
perfektoid = perfeqize(numbers)
from pprint import pprint
pprint(perfektoid)
