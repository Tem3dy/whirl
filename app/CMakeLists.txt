# Create an executable
add_executable(whirl_app WhirlApplication.cpp)

# Link against whirl
target_link_libraries(whirl_app PRIVATE whirl)