def padic_to_vector(padic: str, p: int):
    if '.' in padic:
        int_part, frac_part = padic.split('.')
    else:
        int_part, frac_part = padic, ''
    
    vec = [int(c) for c in reversed(int_part) if c != '-']
    for c in reversed(frac_part):
        vec.insert(0, int(c))  # negative Exponenten links

    k0 = -len(frac_part)
    return vec, k0

def vector_to_padic(vec, p: int, k0=0):
    if k0 < 0:
        frac_digits = vec[: -k0]
        int_digits = vec[-k0:]
        padic = ''.join(str(d) for d in frac_digits) + '.'
        padic += ''.join(str(d) for d in int_digits)
    else:
        padic = ''.join(str(d) for d in vec)
    return padic
