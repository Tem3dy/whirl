#pragma once

#include <cstdint>

// Could only define the necessary symbols without including gl.h
#include <glad/gl.h>

enum class DrawMode
{
    STATIC,
    DYNAMIC,
    STREAM,
};

constexpr inline uint32_t GetMode(DrawMode value) noexcept
{
    switch (value)
    {
        case DrawMode::STATIC:
            return GL_STATIC_DRAW;
        case DrawMode::DYNAMIC:
            return GL_DYNAMIC_DRAW;
        case DrawMode::STREAM:
            return GL_STREAM_DRAW;

        default:
            return GL_STATIC_DRAW;
    }
}