cmake_minimum_required(VERSION 3.10)
project(PerfektoidSimplex)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)

include_directories(lib)

add_library(core
    lib/perfektoid.cpp
    lib/lp_model.cpp
    lib/cli_utils.cpp
)

add_executable(perfektoid_to_lp cpp/perfektoid_to_lp.cpp)
target_link_libraries(perfektoid_to_lp core)

add_executable(lp_to_perfektoid cpp/lp_to_perfektoid.cpp)
target_link_libraries(lp_to_perfektoid core)
