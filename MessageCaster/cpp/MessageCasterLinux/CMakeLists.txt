cmake_minimum_required(VERSION 3.0)
project(MessageCasterLinux)

set(CMAKE_CXX_STANDARD 11)

add_executable(MessageCasterLinux src/main.cpp)

# Find required libraries
find_package(Threads REQUIRED)

# Link required libraries
target_link_libraries(MessageCasterLinux Threads::Threads)