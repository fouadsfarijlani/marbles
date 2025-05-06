cmake_minimum_required(VERSION 3.30)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g")

project(nodzy VERSION 1.0)

add_subdirectory(node)
add_subdirectory(graph)
add_subdirectory(graph_service)

add_executable(marbles main.cpp)

target_link_libraries(marbles PRIVATE graph_service)
