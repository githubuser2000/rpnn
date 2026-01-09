def padic_to_vector(padic: str, p: int):
    vec = [int(c) for c in reversed(padic)]
    if any(d >= p or d < 0 for d in vec):
        raise ValueError("Ungültige Ziffer")
    return vec

def vector_to_padic(vec, p: int):
    if any(d >= p or d < 0 for d in vec):
        raise ValueError("Ungültige Ziffer")
    return ''.join(str(d) for d in reversed(vec))
