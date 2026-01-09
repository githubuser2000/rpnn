#include <iostream>
#include <vector>
#include "padic_utils.h"

void print_usage() {
    std::cout << "Usage: ./padic_to_vec <prime p> <p-adic number>\n";
    std::cout << "Example: ./padic_to_vec 5 132.14\n";
    std::cout << "This will convert p-adic number 132.14 (base 5) to a vector.\n";
    std::cout << "Negative exponents (after '.') are handled automatically.\n";
}

int main(int argc, char* argv[]) {
    if (argc != 3) {
        print_usage();
        return 1;
    }

    int p = 0;
    try { p = std::stoi(argv[1]); } 
    catch (...) { std::cerr << "Error: p must be a prime number.\n"; return 1; }

    std::string padic = argv[2];

    try {
        auto [vec, k0] = padic_to_vector(padic, p);
        std::cout << "Input p-adic number: " << padic << " (base " << p << ")\n";
        std::cout << "Converted vector (k0 = " << k0 << "): [";
        for (size_t i = 0; i < vec.size(); ++i) {
            std::cout << vec[i];
            if (i + 1 < vec.size()) std::cout << ", ";
        }
        std::cout << "]\n";
        std::cout << "Interpretation: first element corresponds to exponent " << k0 << "\n";
    } catch (std::exception& e) {
        std::cerr << "Conversion error: " << e.what() << "\n";
        print_usage();
        return 1;
    }
}
