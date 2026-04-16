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

std::map<int,std::vector<long long>> generate_p_adics(const std::vector<int>& numbers) {
    std::vector<std::map<int,int>> factor_lists;
    std::map<int,int> min_exp, max_exp;

    // Primfaktorzerlegung
    for (int n : numbers) {
        factor_lists.push_back(prime_factors(n));
    }

    // Alle Primzahlen sammeln
    std::map<int,bool> all_primes;
    for (auto &f : factor_lists)
        for (auto &p : f) all_primes[p.first] = true;

    // ggT und kgV Exponenten pro Primzahl
    for (auto &p : all_primes) {
        int prime = p.first;
        int min_e = INT_MAX, max_e = 0;
        for (auto &f : factor_lists) {
            int e = f.count(prime) ? f[prime] : 0;
            min_e = std::min(min_e, e);
            max_e = std::max(max_e, e);
        }
        min_exp[prime] = min_e;
        max_exp[prime] = max_e;
    }

    // p-adische Komponenten als Vektoren der Potenzen
    std::map<int,std::vector<long long>> p_adics;
    for (auto &p : max_exp) {
        int prime = p.first;
        int max_e = p.second;
        for (int e = 0; e <= max_e; ++e)
            p_adics[prime].push_back(pow(prime,e));
    }
    return p_adics;
}

int main() {
    std::vector<int> numbers = {12, 18, 30};
    auto p_adics = generate_p_adics(numbers);

    std::cout << "p-adische Komponenten:\n";
    for (auto &p : p_adics) {
        std::cout << "Primzahl " << p.first << ": ";
        for (auto v : p.second) std::cout << v << " ";
        std::cout << "\n";
    }
}
