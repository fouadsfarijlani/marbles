add_library(parser STATIC parser.cpp)
target_include_directories(parser PUBLIC ${CMAKE_SOURCE_DIR}/include)
