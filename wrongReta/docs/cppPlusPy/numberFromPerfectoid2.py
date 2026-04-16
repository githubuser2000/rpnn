from itertools import product

def numbers_from_perfektoid(perfektoid):
    """
    Generiert alle möglichen Zahlen, die aus der perfektoiden Produktform
    resultieren könnten.
    
    perfektoid: Dict {p: "p^min..p^max"}
    """
    # p: (min_exp, max_exp)
    prime_ranges = {}
    for p, s in perfektoid.items():
        parts = s.split('..')
        min_e = int(parts[0].split('^')[1])
        max_e = int(parts[1].split('^')[1])
        prime_ranges[p] = range(min_e, max_e + 1)
    
    # Alle Kombinationen der Exponenten bilden
    primes = list(prime_ranges.keys())
    exponent_lists = [prime_ranges[p] for p in primes]
    numbers = []
    for exps in product(*exponent_lists):
        n = 1
        for i, e in enumerate(exps):
            n *= primes[i] ** e
        numbers.append(n)
    
    return numbers

# Beispiel
perfektoid = {2: "2^1..2^2", 3: "3^1..3^2", 5: "5^0..5^1"}
possible_numbers = numbers_from_perfektoid(perfektoid)
print(possible_numbers)
