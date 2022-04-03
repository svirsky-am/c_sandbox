cmake_minimum_required(VERSION 3.5)

project (hello_cmake)

macro(print_all_variables)
    message(STATUS "print_all_variables------------------------------------------{")
    get_cmake_property(_variableNames VARIABLES)
    foreach (_variableName ${_variableNames})
        message(STATUS "${_variableName}=${${_variableName}}")
    endforeach()
    message(STATUS "print_all_variables------------------------------------------}")
endmacro()


message(STATUS "${CMAKE_C_COMPILER}=${${CMAKE_C_COMPILER}}")

message(STATUS "${CMAKE_CPP_COMPILER}=${${CMAKE_CPP_COMPILER}}")

message("TEST")
# print_all_variables()
add_executable(${PROJECT_NAME} main.cpp)

