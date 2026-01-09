import sys
from python.padic_utils import padic_to_vector

def print_usage():
    print("Usage: python -m python.padic_to_vec <prime p> <p-adic number>")
    print("Example: python -m python.padic_to_vec 5 132.14")
    print("Converts the p-adic number (possibly with fractional part) into a vector.")
    print("Negative exponents after '.' are handled automatically.")

if len(sys.argv) != 3:
    print_usage()
    sys.exit(1)

try:
    p = int(sys.argv[1])
except:
    print("Error: p must be an integer prime.")
    print_usage()
    sys.exit(1)

padic = sys.argv[2]

try:
    vec, k0 = padic_to_vector(padic, p)
    print(f"Input p-adic number: {padic} (base {p})")
    print(f"Converted vector (k0={k0}): {vec}")
    print(f"Interpretation: first element corresponds to exponent {k0}")
except Exception as e:
    print("Conversion error:", e)
    print_usage()
    sys.exit(1)
