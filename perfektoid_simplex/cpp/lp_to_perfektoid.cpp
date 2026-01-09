#include "perfektoid.h"
#include <iostream>

extern PerfektoidApprox lp_to_perfektoid(
    const std::vector<double>&, int, int);

int main(int argc, char** argv) {
    if (argc < 4) {
        std::cout << "Usage: lp_to_perfektoid p k0 x0 x1 x2 ...\n";
        return 1;
    }

    int p = std::stoi(argv[1]);
    int k0 = std::stoi(argv[2]);

    std::vector<double> sol;
    for (int i = 3; i < argc; ++i)
        sol.push_back(std::stod(argv[i]));

    auto x = lp_to_perfektoid(sol, p, k0);

    std::cout << "Perfektoid approx:\n";
    for (int a : x.a)
        std::cout << a << " ";
    std::cout << "\n";
}
