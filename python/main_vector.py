import sys
from lib.padic_utils import padic_to_vector

if len(sys.argv) != 3:
    print("Usage: python main_vector.py <p> <padic_number>")
    sys.exit(1)

p = int(sys.argv[1])
padic = sys.argv[2]

vec = padic_to_vector(padic, p)
print("Vektor:", vec)
