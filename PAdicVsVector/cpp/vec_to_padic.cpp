#include <iostream>
#include <vector>
#include <sstream>
#include "padic_utils.h"

void print_usage() {
    std::cout << "Usage: ./vec_to_padic <prime p> <k0> <comma_separated_vector>\n";
    std::cout << "Example: ./vec_to_padic 5 -2 1,4,1,3,2\n";
    std::cout << "This converts vector [1,4,1,3,2] with k0=-2 to a p-adic number.\n";
    std::cout << "k0 indicates the exponent of the first vector element.\n";
}

int main(int argc, char* argv[]) {
    if (argc != 4) {
        print_usage();
        return 1;
    }

    int p = 0, k0 = 0;
    try { p = std::stoi(argv[1]); k0 = std::stoi(argv[2]); } 
    catch (...) { std::cerr << "Error: p and k0 must be integers.\n"; return 1; }

    std::string input = argv[3];
    std::vector<int> vec;
    std::istringstream iss(input);
    std::string token;
    while (std::getline(iss, token, ',')) {
        try { vec.push_back(std::stoi(token)); }
        catch (...) { std::cerr << "Error: vector must contain integers only.\n"; return 1; }
    }

    try {
        std::string padic = vector_to_padic(vec, p, k0);
        std::cout << "Input vector: [";
        for (size_t i=0; i<vec.size(); ++i) {
            std::cout << vec[i];
            if (i+1<vec.size()) std::cout << ", ";
        }
        std::cout << "], k0=" << k0 << "\n";
        std::cout << "Converted p-adic number (base " << p << "): " << padic << "\n";
    } catch (std::exception& e) {
        std::cerr << "Conversion error: " << e.what() << "\n";
        print_usage();
        return 1;
    }
}
