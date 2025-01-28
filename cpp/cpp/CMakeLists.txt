cmake_minimum_required(VERSION 3.0)
project(qsm_gcc)

set(CMAKE_CXX_STANDARD 11)

# Include directories
include_directories(src/QuickShotMessage)

add_executable(qsm_gcc src/main.cpp)

# Find required libraries
find_package(Threads REQUIRED)

# Link required libraries
target_link_libraries(qsm_gcc Threads::Threads)