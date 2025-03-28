# Minimum CMake version required to build the library
cmake_minimum_required(VERSION 3.30)

# Project information
project(whirl VERSION 0.0.0)

# Ensure that GLFW, GLEW and OpenGL are installed
find_package(glfw3 REQUIRED)
find_package(GLEW REQUIRED)
find_package(OpenGL REQUIRED)

# Set C++ standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the library target
add_library(${PROJECT_NAME} STATIC
    src/Whirl.cpp
    src/GuiApplication.cpp
)

# Add the include directory to the include path
target_include_directories(${PROJECT_NAME} PUBLIC include)

# Link GLFW, GLEW and OpenGL
target_link_libraries(${PROJECT_NAME} PUBLIC glfw GLEW GL)

# Add the demo app as a subdirectory
add_subdirectory(app/)