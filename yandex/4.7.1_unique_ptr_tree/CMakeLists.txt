project (4.7.1_share_pointer)

cmake_minimum_required(VERSION 3.6)
# # Add a library with the above sources
# add_library(${PROJECT_NAME} include/finalValueAfterOperations.c)
# # add_library(sub::lib1 ALIAS ${PROJECT_NAME})
# add_library(sub::2011_finalValueAfterOperations ALIAS ${PROJECT_NAME})


add_executable(${PROJECT_NAME} sources/tree.cpp)

target_include_directories( ${PROJECT_NAME}
    PUBLIC ${PROJECT_SOURCE_DIR}/sources
)
