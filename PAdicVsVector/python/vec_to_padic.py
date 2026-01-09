import sys
from python.padic_utils import vector_to_padic

def print_usage():
    print("Usage: python -m python.vec_to_padic <prime p> <k0> <comma_separated_vector>")
    print("Example: python -m python.vec_to_padic 5 -2 1,4,1,3,2")
    print("Converts the vector into a p-adic number.")
    print("k0 indicates the exponent of the first element.")

if len(sys.argv) != 4:
    print_usage()
    sys.exit(1)

try:
    p = int(sys.argv[1])
    k0 = int(sys.argv[2])
except:
    print("Error: p and k0 must be integers.")
    print_usage()
    sys.exit(1)

try:
    vec = list(map(int, sys.argv[3].split(',')))
except:
    print("Error: vector must contain integers separated by commas.")
    print_usage()
    sys.exit(1)

try:
    padic = vector_to_padic(vec, p, k0)
    print(f"Input vector: {vec}, k0={k0}")
    print(f"Converted p-adic number (base {p}): {padic}")
except Exception as e:
    print("Conversion error:", e)
    print_usage()
    sys.exit(1)
