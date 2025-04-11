#pragma once

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

namespace Math
{
    template <typename T>
    inline constexpr T Clamp(T value, T min, T max)
    {
        if (value < min)
            return min;
        if (value > max)
            return max;

        return value;
    }

    template <typename T>
    inline constexpr T Min(T x, T y)
    {
        return x < y ? x : y;
    }

    template <typename T>
    inline constexpr T Max(T x, T y)
    {
        return x > y ? x : y;
    }
};