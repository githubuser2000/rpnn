#include <iostream>
#include <vector>
#include <sstream>
#include "padic_utils.h"

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: ./main_padic <p> <comma_separated_vector>\n";
        return 1;
    }
    int p = std::stoi(argv[1]);
    std::string input = argv[2];
    std::vector<int> vec;
    std::istringstream iss(input);
    std::string token;
    while (std::getline(iss, token, ',')) {
        vec.push_back(std::stoi(token));
    }

    try {
        std::string padic = vector_to_padic(vec, p);
        std::cout << "p-adische Zahl: " << padic << "\n";
    } catch (std::exception& e) {
        std::cerr << e.what() << "\n";
        return 1;
    }
}
