#include "cli_utils.h"
#include <iostream>
#include <sstream>

std::vector<int> parse_int_list(const std::string& s) {
    std::vector<int> result;
    std::stringstream ss(s);
    std::string item;

    while (std::getline(ss, item, ',')) {
        result.push_back(std::stoi(item));
    }
    return result;
}

void print_header(const std::string& title) {
    std::cout << "\n=== " << title << " ===\n\n";
}

void print_error(const std::string& msg) {
    std::cerr << "Error: " << msg << "\n";
}
