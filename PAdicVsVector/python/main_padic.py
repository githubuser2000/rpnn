import sys
from lib.padic_utils import vector_to_padic

if len(sys.argv) != 3:
    print("Usage: python main_padic.py <p> <comma_separated_vector>")
    sys.exit(1)

p = int(sys.argv[1])
vec = list(map(int, sys.argv[2].split(',')))

padic = vector_to_padic(vec, p)
print("p-adische Zahl:", padic)
