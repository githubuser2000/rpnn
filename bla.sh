g++ -std=c++17 -o padic_to_vec cpp/padic_to_vec.cpp lib/padic_utils.cpp
g++ -std=c++17 -o vec_to_padic cpp/vec_to_padic.cpp lib/padic_utils.cpp

./padic_to_vec 5 132.14      # p=5, Zahl mit negativen Potenzen
./vec_to_padic 5 -2 1,3,2,1,4 # k0=-2, Vektor
