cmake_minimum_required(VERSION 3.15)
project(PadicVectorProject)

# ----------------------------
# C++ Standard & Output
# ----------------------------
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)

# ----------------------------
# Library aus lib/
# ----------------------------
include_directories(${CMAKE_SOURCE_DIR}/lib)
add_library(padic_utils STATIC
    lib/padic_utils.cpp
    lib/padic_utils.h
)

# ----------------------------
# Executables
# ----------------------------
add_executable(padic_to_vec cpp/padic_to_vec.cpp)
target_link_libraries(padic_to_vec padic_utils)

add_executable(vec_to_padic cpp/vec_to_padic.cpp)
target_link_libraries(vec_to_padic padic_utils)

# ----------------------------
# Python Integration
# ----------------------------
find_package(Python3 COMPONENTS Interpreter Development REQUIRED)

# Installiere Python-Skripte in bin/
install(PROGRAMS
    python/padic_to_vec.py
    python/vec_to_padic.py
    DESTINATION ${CMAKE_SOURCE_DIR}/bin
)

# Optional: Vorbereitung für pybind11 (C++ als Python-Modul)
# find_package(pybind11 REQUIRED)
# pybind11_add_module(padic_utils_cpp lib/padic_utils.cpp)
# set_target_properties(padic_utils_cpp PROPERTIES
#     PREFIX ""
# )
