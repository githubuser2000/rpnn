#include "lp_model.h"
#include "perfektoid.h"

PerfektoidApprox lp_to_perfektoid(
    const std::vector<double>& solution,
    int p,
    int k0
) {
    PerfektoidApprox x;
    x.p = p;
    x.k0 = k0;

    for (double v : solution)
        x.a.push_back(static_cast<int>(v + 0.5));

    return x;
}
