def lp_to_perfektoid(solution, p, k0):
    a = [int(round(x)) for x in solution]
    return {
        "p": p,
        "k0": k0,
        "digits": a
    }
