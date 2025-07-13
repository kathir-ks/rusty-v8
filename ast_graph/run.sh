#!/bin/bash

# Install Clang/LLVM and development headers
echo "Installing LLVM/Clang development packages..."

# For Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
    clang \
    libclang-dev \
    libclang-cpp-dev \
    llvm-dev \
    libboost-graph-dev \
    libboost-system-dev \
    cmake \
    pkg-config

# Alternative: Install specific LLVM version (recommended)
# Add LLVM repository for latest stable version
wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-15 main" | sudo tee /etc/apt/sources.list.d/llvm.list
sudo apt-get update
sudo apt-get install -y \
    clang-15 \
    libclang-15-dev \
    libclang-cpp15-dev \
    llvm-15-dev

# Check installation
echo "Checking LLVM/Clang installation..."
llvm-config --version
clang --version

# Get compiler flags
echo "LLVM compile flags:"
llvm-config --cxxflags --ldflags --system-libs --libs core support

# Create CMakeLists.txt for easier building
cat > CMakeLists.txt << 'EOF'
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
EOF

# Build with CMake
echo "Building with CMake..."
mkdir -p build
cd build
cmake ..
make -j$(nproc)

# Alternative: Direct compilation command
echo "Alternative direct compilation command:"
echo "clang++ -std=c++17 ast_graph_builder.cpp \\"
echo "  \$(llvm-config --cxxflags --ldflags --system-libs --libs core support) \\"
echo "  -lclangTooling -lclangFrontend -lclangDriver -lclangSerialization \\"
echo "  -lclangParse -lclangSema -lclangAnalysis -lclangAST -lclangBasic \\"
echo "  -lclangEdit -lclangLex -lclangRewrite \\"
echo "  -lboost_graph -lboost_system \\"
echo "  -o code_graph"
