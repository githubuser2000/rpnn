from sympy import factorint
from itertools import product
from pyadic import PAdic
from fractions import Fraction

# ---- 1) Richtung A: Zahlen → perfektoid

def compute_perfektoid(numbers):
    # Zerlegung
    factor_lists = [factorint(n) for n in numbers]
    # alle Primzahlen
    primes = sorted(set().union(*[set(f.keys()) for f in factor_lists]))
    
    min_exp = {}
    max_exp = {}
    for p in primes:
        exps = [f.get(p,0) for f in factor_lists]
        min_exp[p] = min(exps)
        max_exp[p] = max(exps)
    
    # Produktform (symbolisch)
    product_form = {p:(min_exp[p], max_exp[p]) for p in primes}
    
    return {
        "primes": primes,
        "min_exp": min_exp,
        "max_exp": max_exp,
        "product_form": product_form
    }

# ---- 2) Richtung B: perfektoid → mögliche ganze Zahlen

def numbers_from_product_form(product_form):
    primes = sorted(product_form.keys())
    ranges = [range(product_form[p][0], product_form[p][1]+1) for p in primes]
    
    nums = []
    for exps in product(*ranges):
        n = 1
        for p,e in zip(primes, exps):
            n *= p**e
        nums.append(n)
    return sorted(nums)

# ---- 3) p-adische Darstellung mit pyadic

def padic_representations(numbers, precision=10):
    padics = {}
    for n in numbers:
        padics[n] = {}
        f = factorint(n)
        for p,e in f.items():
            # In Q_p mit N-stelligen Präzision
            padics[n][p] = PAdic(Fraction(n,1), p, precision)
    return padics

# ---- Beispiel

nums = [12, 18, 30]
perf = compute_perfektoid(nums)

print("Perfektoide Produktform:", perf["product_form"])

candidates = numbers_from_product_form(perf["product_form"])
print("Alle möglichen Zahlen:", candidates)

padic_forms = padic_representations(candidates)
print("p‑adische Repräsentationen (kurz):")
for n, reps in padic_forms.items():
    for p,val in reps.items():
        print(f"{n} in Q_{p} :", val)
