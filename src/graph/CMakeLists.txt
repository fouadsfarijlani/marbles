add_library(graph STATIC graph.cpp)
target_include_directories(graph PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_link_libraries(graph PUBLIC node)