cmake_minimum_required(VERSION 3.10)
project(PadicVectorProject)

# C++ Standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Bin-Ordner für Executables
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)

# Include Verzeichnisse
include_directories(${CMAKE_SOURCE_DIR}/lib)

# Library aus lib erstellen
add_library(padic_utils STATIC
    lib/padic_utils.cpp
    lib/padic_utils.h
)

# Executables erstellen
add_executable(padic_to_vec cpp/padic_to_vec.cpp)
target_link_libraries(padic_to_vec padic_utils)

add_executable(vec_to_padic cpp/vec_to_padic.cpp)
target_link_libraries(vec_to_padic padic_utils)
