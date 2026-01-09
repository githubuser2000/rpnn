import sys
from lib.padic_utils import vector_to_padic

if len(sys.argv) != 4:
    print("Usage: python vec_to_padic.py <p> <k0> <comma_separated_vector>")
    sys.exit(1)

p = int(sys.argv[1])
k0 = int(sys.argv[2])
vec = list(map(int, sys.argv[3].split(',')))

padic = vector_to_padic(vec, p, k0)
print("p-adische Zahl:", padic)
