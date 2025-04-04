#pragma once

namespace Math
{
    template <typename T>
    constexpr T Clamp(T value, T min, T max)
    {
        if (value < min)
            return min;
        if (value > max)
            return max;
        
        return value;
    }
};