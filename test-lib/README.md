# test-lib

A C++ project built with CMake

## Features

- **C++ Standard**: C++17
- **Build System**: CMake
- **Project Type**: library
- **Testing**: Enabled with Catch2
- **License**: MIT

## Prerequisites

- CMake 3.15 or higher
- A C++17 (or higher) compatible compiler

## Build Steps

1. Configure the project:
```bash
cmake -B build -DCMAKE_BUILD_TYPE=Release
```

2. Build the project:
```bash
cmake --build build --config Release
```

3. Use the library:
The library will be built in `build/lib/` (or `build/` on Windows) and can be linked against other projects.

### Build Types

Available build types:
- `Debug`: Debug build with no optimizations
- `Release`: Optimized release build
- `RelWithDebInfo`: Release build with debug information

## Testing


This project uses [Catch2](https://github.com/catchorg/Catch2) for testing.

### Running Tests

Build and run tests:
```bash
cmake --build build --target test
ctest --test-dir build --output-on-failure
```

Or run the test executable directly:
```bash
./build/tests
```

### Test Structure

- Test files are located in the `tests/` directory
- The main test executable is `tests/test_main.cpp`
- Tests use Catch2's BDD-style macros for readable test cases



## Usage


### Integration with CMake

Add this project to your CMake file:

```cmake
find_package(test-lib REQUIRED)
target_link_libraries(your_target test-lib::test-lib)
```

### API Reference

```cpp
#include "test-lib/test-lib.hpp"

// Basic usage
int result = test-lib::add(2, 3);
```



## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Project Structure

```
test-lib/
├── CMakeLists.txt          # Main CMake configuration
├── README.md               # This file
├── .gitignore              # Git ignore patterns
├── src/                    # Source files
│   └── test-lib.cpp
├── include/                # Header files
│   └── test-lib/
│       └── test-lib.hpp
├── tests/                   # Test files
│   └── test_main.cpp


└── build/                  # Build directory (generated)
```

## License

This project is licensed under the MIT license.

## Generated with Devora

This project was generated using [Devora](https://github.com/your-username/devora), the universal project scaffolding framework.

Generated on: 2025-10-30
Author: Nathan