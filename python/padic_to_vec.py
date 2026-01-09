import sys
from lib.padic_utils import padic_to_vector

if len(sys.argv) != 3:
    print("Usage: python padic_to_vec.py <p> <padic_number>")
    sys.exit(1)

p = int(sys.argv[1])
padic = sys.argv[2]

vec, k0 = padic_to_vector(padic, p)
print(f"Vektor (k0={k0}): {vec}")
