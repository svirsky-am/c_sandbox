project(main_into)


set(SOURCES
    src/finalValueAfterOperations.c
    src/main.c
)

# Create the executable
add_executable(${PROJECT_NAME} src/main.c)

# Link the static library from subproject1 using its alias sub::lib1
# Link the header only library from subproject2 using its alias sub::lib2
# This will cause the include directories for that target to be added to this project
target_link_libraries(${PROJECT_NAME}
    sublibrary1
    sub::2011_finalValueAfterOperations
)
