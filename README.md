# whirl 🌀
A C++ GUI library

# Building

Instructions on how to build whirl

## Requirements
- Git
- CMake **3.30+**
- C++ **20** compatible compiler (GCC, MSVC, clang)
- GLFW3, GLEW and OpenGL installed on your system

## Build process
1. Clone the project:
```bash
git clone https://github.com/Tem3dy/whirl.git
```

2. Generate build files for your preferred build system (`Makefile`, `Ninja`, `Solution`), for example:
```bash
cmake . -B build -G "Ninja" 
```

3. Build the project:
```bash
cmake --build build
```

This will build the library under `build/` as well as an executable under `build/app/` that you can run to see a basic example of an application that uses Whirl.