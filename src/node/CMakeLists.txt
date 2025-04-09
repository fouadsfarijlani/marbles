add_library(node STATIC node.cpp)
target_include_directories(node PUBLIC ${CMAKE_SOURCE_DIR}/include)
