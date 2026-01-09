#ifndef PADIC_UTILS_H
#define PADIC_UTILS_H

#include <vector>
#include <string>
#include <utility>

// Rückgabe: (vektor, minimaler Exponent k0)
std::pair<std::vector<int>, int> padic_to_vector(const std::string& padic, int p);
std::string vector_to_padic(const std::vector<int>& vec, int p, int k0 = 0);

#endif
