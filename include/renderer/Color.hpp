#pragma once

#include <cstdint>

// clang-format off
namespace Color
{
    inline constexpr uint32_t New(uint8_t r, uint8_t g, uint8_t b, uint8_t a = 0xff)
    {
        return (r << 24) | (g << 16) | (b << 8) | a;
    }
    
    inline constexpr uint32_t White()
    {
        return New(0xff, 0xff, 0xff);
    }

    inline constexpr uint32_t Black()
    {
        return New(0x00, 0x00, 0x00);
    }

    inline constexpr uint32_t Red()
    {
        return New(0xff, 0x00, 0x00);
    }

    inline constexpr uint32_t Green()
    {
        return New(0x00, 0xff, 0x00);
    }

    inline constexpr uint32_t Blue()
    {
        return New(0x00, 0x00, 0xff);
    }

    inline constexpr uint32_t Yellow()
    {
        return New(0xff, 0xff, 0x00);
    }

    inline constexpr uint32_t Cyan()
    {
        return New(0x00, 0xff, 0xff);
    }

    inline constexpr uint32_t Magenta()
    {
        return New(0xff, 0x00, 0xff);
    }
};
// clang-format on