project (2011_finalValueAfterOperations)

cmake_minimum_required(VERSION 3.6)
# Add a library with the above sources
add_library(${PROJECT_NAME} include/finalValueAfterOperations.c)
# add_library(sub::lib1 ALIAS ${PROJECT_NAME})
add_library(sub::2011_finalValueAfterOperations ALIAS ${PROJECT_NAME})


target_include_directories( ${PROJECT_NAME}
    PUBLIC ${PROJECT_SOURCE_DIR}/include
)
