#include "padic_utils.h"
#include <sstream>
#include <stdexcept>

std::pair<std::vector<int>, int> padic_to_vector(const std::string& padic, int p) {
    // Format: optional '-' für negatives Vorzeichen, optional '.' für Dezimalpunkt (negative Potenzen)
    int k0 = 0;  // minimaler Exponent
    std::vector<int> vec;

    size_t dot = padic.find('.');
    std::string int_part = padic.substr(0, dot);
    std::string frac_part = (dot != std::string::npos) ? padic.substr(dot + 1) : "";

    // Ganze Zahl (positiv oder negativ)
    for (auto it = int_part.rbegin(); it != int_part.rend(); ++it) {
        if (*it == '-') continue; // Vorzeichen ignorieren
        int digit = *it - '0';
        if (digit < 0 || digit >= p) throw std::runtime_error("Ungültige Ziffer in integer part");
        vec.push_back(digit);
    }

    // Fraktion
    for (char c : frac_part) {
        int digit = c - '0';
        if (digit < 0 || digit >= p) throw std::runtime_error("Ungültige Ziffer in fractional part");
        vec.insert(vec.begin(), digit); // negative Exponenten links
    }

    k0 = frac_part.length() * -1;
    return {vec, k0};
}

std::string vector_to_padic(const std::vector<int>& vec, int p, int k0) {
    if (vec.empty()) return "0";

    std::ostringstream oss;

    if (k0 < 0) {
        // negative Exponenten zuerst
        for (int i = 0; i < -k0; ++i) {
            if (i < vec.size())
                oss << vec[i];
            else
                oss << '0';
        }
        oss << '.';
    }

    for (size_t i = std::max(0, -k0); i < vec.size(); ++i) {
        if (vec[i] < 0 || vec[i] >= p) throw std::runtime_error("Ungültige Ziffer");
        oss << vec[i];
    }

    return oss.str();
}
