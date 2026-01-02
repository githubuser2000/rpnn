#include <iostream>
#include <vector>
#include <map>
#include <cmath>

void generate_numbers(std::map<int,std::pair<int,int>>& perfektoid,
                      std::vector<int>& primes,
                      std::vector<int>& exps,
                      int index,
                      std::vector<long long>& results) {
    if(index == primes.size()) {
        long long n = 1;
        for(int i = 0; i < primes.size(); ++i)
            n *= pow(primes[i], exps[i]);
        results.push_back(n);
        return;
    }
    int p = primes[index];
    for(int e = perfektoid[p].first; e <= perfektoid[p].second; ++e) {
        exps[index] = e;
        generate_numbers(perfektoid, primes, exps, index+1, results);
    }
}

int main() {
    // perfektoid: p -> (min_exp, max_exp)
    std::map<int,std::pair<int,int>> perfektoid;
    perfektoid[2] = {1,2};
    perfektoid[3] = {1,2};
    perfektoid[5] = {0,1};

    std::vector<int> primes;
    for(auto &p : perfektoid) primes.push_back(p.first);

    std::vector<int> exps(primes.size(),0);
    std::vector<long long> results;
    generate_numbers(perfektoid, primes, exps, 0, results);

    std::cout << "Mögliche Zahlen:\n";
    for(auto n : results) std::cout << n << " ";
    std::cout << std::endl;
}
