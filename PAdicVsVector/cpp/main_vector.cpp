#include <iostream>
#include <string>
#include "padic_utils.h"

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: ./main_vector <p> <padic_number>\n";
        return 1;
    }
    int p = std::stoi(argv[1]);
    std::string padic = argv[2];

    try {
        auto vec = padic_to_vector(padic, p);
        std::cout << "Vektor: [";
        for (size_t i = 0; i < vec.size(); ++i) {
            std::cout << vec[i];
            if (i + 1 < vec.size()) std::cout << ", ";
        }
        std::cout << "]\n";
    } catch (std::exception& e) {
        std::cerr << e.what() << "\n";
        return 1;
    }
}
