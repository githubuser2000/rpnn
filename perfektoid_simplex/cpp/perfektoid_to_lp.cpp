#include <iostream>
#include <string>

#include "perfektoid.h"
#include "lp_model.h"
#include "cli_utils.h"

extern LinearProgram perfektoid_to_lp(const PerfektoidApprox&);

void print_help() {
    std::cout <<
    "Usage:\n"
    "  perfektoid_to_lp [OPTIONS]\n\n"
    "Required parameters:\n"
    "  --p <prime>           Prime p (e.g. 2, 3, 5)\n"
    "  --k0 <integer>        Lowest valuation exponent\n"
    "  --digits <a0,a1,...>  p-adic digits (comma separated)\n\n"
    "Optional:\n"
    "  --objective valuation   Minimize p-adic valuation (default)\n"
    "  --help                  Show this help\n\n"
    "Example:\n"
    "  perfektoid_to_lp --p 5 --k0 -2 --digits 1,3,2,4\n";
}

int main(int argc, char** argv) {
    if (argc == 1) {
        print_help();
        return 0;
    }

    PerfektoidApprox x;
    std::string digits_raw;

    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];

        if (arg == "--help") {
            print_help();
            return 0;
        }
        else if (arg == "--p" && i + 1 < argc) {
            x.p = std::stoi(argv[++i]);
        }
        else if (arg == "--k0" && i + 1 < argc) {
            x.k0 = std::stoi(argv[++i]);
        }
        else if (arg == "--digits" && i + 1 < argc) {
            digits_raw = argv[++i];
        }
        else {
            print_error("Unknown or incomplete argument: " + arg);
            return 1;
        }
    }

    if (x.p <= 1)
        return print_error("p must be a prime ≥ 2"), 1;

    if (digits_raw.empty())
        return print_error("Missing --digits argument"), 1;

    x.a = parse_int_list(digits_raw);

    print_header("Perfektoid → Linear Program");

    LinearProgram lp = perfektoid_to_lp(x);

    std::cout << "Objective (minimize):\n";
    for (double c : lp.objective)
        std::cout << "  " << c;
    std::cout << "\n\nConstraints (A x = b):\n";

    for (size_t i = 0; i < lp.A.size(); ++i) {
        std::cout << "  ";
        for (double v : lp.A[i])
            std::cout << v << " ";
        std::cout << "= " << lp.b[i] << "\n";
    }

    std::cout << "\nBounds:\n";
    for (size_t i = 0; i < lp.lower.size(); ++i)
        std::cout << "  0 ≤ a_" << i << " ≤ " << lp.upper[i] << "\n";
}
