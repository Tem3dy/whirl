# Minimum CMake version required to build the library
cmake_minimum_required(VERSION 3.30)

# Project information
project(whirl VERSION 0.3.0)

# Ensure OpenGL is installed
find_package(OpenGL REQUIRED)

# Enable fetch content
include(FetchContent)

# Fetch GLFW
FetchContent_Declare(
    glfw
    GIT_REPOSITORY https://github.com/glfw/glfw
    GIT_TAG 3.4
)

# Force static build
set(GLFW_BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)

# Disable examples
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)

# Disable tests
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)

# Make GLFW available
FetchContent_MakeAvailable(glfw)

# Fetch GLM
FetchContent_Declare(
    glm
    GIT_REPOSITORY https://github.com/g-truc/glm
    GIT_TAG 1.0.1
)

# Make GLM available
FetchContent_MakeAvailable(glm)

# Fetch FMT
FetchContent_Declare(
    fmt
    GIT_REPOSITORY https://github.com/fmtlib/fmt.git
    GIT_TAG 11.1.4
)

# Make FMT available
FetchContent_MakeAvailable(fmt)

# Compile GLAD as a static library
add_library(glad STATIC
    lib/glad/src/gl.c
)

# Add GLAD's include path to its target
target_include_directories(glad PUBLIC
    lib/glad/include
)

# Set C++ standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the library target
add_library(${PROJECT_NAME} STATIC
    # Interface
    src/Whirl.cpp
    src/GuiApplication.cpp
    
    # Renderer
    src/renderer/Shader.cpp
    src/renderer/VertexBuffer.cpp
    src/renderer/IndexBuffer.cpp
    src/renderer/VertexArray.cpp
    src/renderer/GuiRenderer.cpp
    src/renderer/QuadRenderer.cpp
    src/renderer/RoundedQuadRenderer.cpp
    src/renderer/CircleRenderer.cpp
)

# Compile with C++ 20
target_compile_features(${PROJECT_NAME} PUBLIC
    cxx_std_20
)

# Add the include directory to the include path
target_include_directories(${PROJECT_NAME} PUBLIC
    # Whirl include path
    include/
    include/math/
    include/renderer/
    include/logger/
    include/error/
    
    # GLM include path
    ${glm_SOURCE_DIR}
)

# Link GLFW, GLAD and OpenGL
target_link_libraries(${PROJECT_NAME} PUBLIC
    glfw
    glad
    fmt
    OpenGL::GL
)

# Add the demo app as a subdirectory
add_subdirectory(app/)