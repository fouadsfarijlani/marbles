add_library(graph_service STATIC graph_service.cpp)
target_include_directories(graph_service PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_link_libraries(graph_service PUBLIC graph)