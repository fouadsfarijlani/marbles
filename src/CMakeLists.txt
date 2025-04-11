cmake_minimum_required(VERSION 3.30)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(nodzy VERSION 1.0)

add_subdirectory(node)
add_subdirectory(graph)
add_subdirectory(graph_service)

add_executable(nodzy main.cpp)

target_link_libraries(nodzy PRIVATE graph_service)
