#pragma once
#include <vector>
#include <string>

struct PerfektoidApprox {
    int p;                 // Primzahl
    int k0;                // Startbewertung
    std::vector<int> a;    // Ziffern a_k
};
