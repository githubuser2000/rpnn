#include <iostream>
#include <vector>
#include <map>
#include <cmath>
#include <algorithm>

std::map<int,int> prime_factors(int n) {
    std::map<int,int> factors;
    for (int p = 2; p*p <= n; ++p) {
        while (n % p == 0) {
            factors[p]++;
            n /= p;
        }
    }
    if (n > 1) factors[n] = 1;
    return factors;
}

int gcd(int a, int b) {
    while (b != 0) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

int lcm(int a, int b) {
    return a / gcd(a,b) * b;
}

std::map<int,int> min_exponents(const std::vector<std::map<int,int>>& factor_lists, const std::vector<int>& all_primes) {
    std::map<int,int> min_exp;
    for (int p : all_primes) {
        int m = INT_MAX;
        for (auto &f : factor_lists)
            m = std::min(m, f.count(p) ? f.at(p) : 0);
        min_exp[p] = m;
    }
    return min_exp;
}

std::map<int,int> max_exponents(const std::vector<std::map<int,int>>& factor_lists, const std::vector<int>& all_primes) {
    std::map<int,int> max_exp;
    for (int p : all_primes) {
        int M = 0;
        for (auto &f : factor_lists)
            M = std::max(M, f.count(p) ? f.at(p) : 0);
        max_exp[p] = M;
    }
    return max_exp;
}

int main() {
    std::vector<int> numbers = {12, 18, 30};
    std::vector<std::map<int,int>> factor_lists;
    std::map<int,bool> prime_set;

    // Primfaktorzerlegung
    for (int n : numbers) {
        auto f = prime_factors(n);
        factor_lists.push_back(f);
        for (auto &p : f) prime_set[p.first] = true;
    }

    // Alle Primzahlen
    std::vector<int> all_primes;
    for (auto &p : prime_set) all_primes.push_back(p.first);

    auto min_exp = min_exponents(factor_lists, all_primes);
    auto max_exp = max_exponents(factor_lists, all_primes);

    // ggT und kgV berechnen
    int ggT = 1, kgV = 1;
    for (int p : all_primes) {
        ggT *= pow(p, min_exp[p]);
        kgV *= pow(p, max_exp[p]);
    }

    // Perfektoide Zahl in Produktform ausgeben
    std::cout << "ggT: " << ggT << ", kgV: " << kgV << "\n";
    std::cout << "Perfektoide Zahl (Produktform):\n";
    for (int p : all_primes)
        std::cout << p << "^" << min_exp[p] << ".." << p << "^" << max_exp[p] << "\n";
}
