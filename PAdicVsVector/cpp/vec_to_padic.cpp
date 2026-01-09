#include <iostream>
#include <vector>
#include <sstream>
#include "padic_utils.h"

int main(int argc, char* argv[]) {
    if (argc != 4) {
        std::cerr << "Usage: ./vec_to_padic <p> <k0> <comma_separated_vector>\n";
        return 1;
    }

    int p = std::stoi(argv[1]);
    int k0 = std::stoi(argv[2]);

    std::string input = argv[3];
    std::vector<int> vec;
    std::istringstream iss(input);
    std::string token;
    while (std::getline(iss, token, ',')) {
        vec.push_back(std::stoi(token));
    }

    try {
        std::string padic = vector_to_padic(vec, p, k0);
        std::cout << "p-adische Zahl: " << padic << "\n";
    } catch (std::exception& e) {
        std::cerr << e.what() << "\n";
        return 1;
    }
}
