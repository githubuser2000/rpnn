#pragma once
#include <vector>
#include <string>

struct LinearProgram {
    std::vector<double> objective;                 // c_k
    std::vector<std::vector<double>> A;            // Nebenbedingungen
    std::vector<double> b;                          // rechte Seite
    std::vector<double> lower, upper;               // Box-Constraints
};
