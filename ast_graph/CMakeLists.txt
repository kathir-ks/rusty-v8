cmake_minimum_required(VERSION 3.10)
project(CodeGraphBuilder)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find LLVM/Clang
find_package(LLVM REQUIRED CONFIG)
find_package(Clang REQUIRED CONFIG)

# Add LLVM definitions and include directories
add_definitions(${LLVM_DEFINITIONS})
include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${CLANG_INCLUDE_DIRS})

# Find Boost
find_package(Boost REQUIRED COMPONENTS graph system)

# Add executable
add_executable(code_graph ast_graph_builder.cpp)

# Link libraries
target_link_libraries(code_graph
    clangTooling
    clangFrontend
    clangDriver
    clangSerialization
    clangParse
    clangSema
    clangAnalysis
    clangAST
    clangBasic
    clangEdit
    clangLex
    clangRewrite
    ${Boost_LIBRARIES}
)

# Link LLVM libraries
llvm_map_components_to_libnames(llvm_libs support core irreader)
target_link_libraries(code_graph ${llvm_libs})
