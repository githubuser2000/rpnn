#include "perfektoid.h"
#include "lp_model.h"
#include <cmath>

LinearProgram perfektoid_to_lp(const PerfektoidApprox& x) {
    int n = x.a.size();
    LinearProgram lp;

    // Zielfunktion: p-adische Tiefe minimieren
    lp.objective.resize(n);
    for (int i = 0; i < n; ++i)
        lp.objective[i] = x.k0 + i;

    // Box-Constraints: 0 ≤ a_k ≤ p-1
    lp.lower.assign(n, 0.0);
    lp.upper.assign(n, x.p - 1);

    // Frobenius-Kohärenz: a_{k+p} - a_k = 0
    for (int i = 0; i + x.p < n; ++i) {
        std::vector<double> row(n, 0.0);
        row[i] = -1.0;
        row[i + x.p] = 1.0;
        lp.A.push_back(row);
        lp.b.push_back(0.0);
    }

    return lp;
}
