cmake_minimum_required(VERSION 3.15)

project(test-lib
    VERSION 1.0.0
    DESCRIPTION "A C++ project"
    LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Set build type
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Compiler-specific options
if(MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()


# Library target
add_library(test-lib)
target_sources(test-lib
    PRIVATE
        src/test-lib.cpp
    PUBLIC
        include/test-lib/test-lib.hpp
)

# Include directories
target_include_directories(test-lib
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
    PRIVATE
        src
)

# Set library properties
set_target_properties(test-lib PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION ${PROJECT_VERSION_MAJOR}
)




# Enable testing
enable_testing()

# Find Catch2 (or add as subdirectory)
find_package(Catch2 QUIET)
if(NOT Catch2_FOUND)
    # Add Catch2 as a subdirectory if not found
    include(FetchContent)
    FetchContent_Declare(
        Catch2
        GIT_REPOSITORY https://github.com/catchorg/Catch2.git
        GIT_TAG        v3.4.0
    )
    FetchContent_MakeAvailable(Catch2)
endif()

# Test executable
add_executable(tests)
target_sources(tests
    PRIVATE
        tests/test_main.cpp
)

target_link_libraries(tests
    PRIVATE
        Catch2::Catch2WithMain
        test-lib
)

# Include directories for tests
target_include_directories(tests
    PRIVATE
        include
        src
)

# Add tests
add_test(NAME AllTests COMMAND tests)



# Installation

install(TARGETS test-lib
    EXPORT test-libTargets
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(DIRECTORY include/
    DESTINATION include
)

install(EXPORT test-libTargets
    FILE test-libTargets.cmake
    NAMESPACE test-lib::
    DESTINATION lib/cmake/test-lib
)
