
  
# Set the minimum version of CMake that can be used
# To find the cmake version run
# $ cmake --version
set(CMAKE_BUILD_TYPE Debug)
cmake_minimum_required(VERSION 3.5)

# Set the project name
project (2_twoSun)

# Create a sources variable with a link to all cpp files to compile
set(SOURCES
    src/solution.c
)

# Add an executable with the above sources
add_executable(2_twoSun ${SOURCES})
include_directories(./src)

# Set the directories that should be included in the build command for this target
# when running g++ these will be included as -I/directory/path/
# target_include_directories(hello_headers
#     PRIVATE 
#         ${PROJECT_SOURCE_DIR}/include
# )