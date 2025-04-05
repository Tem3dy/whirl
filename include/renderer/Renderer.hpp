#pragma once

#include <cstdint>
#include <vector>

#include "math/Math.hpp"

// clang-format off
struct Color
{
    static constexpr uint32_t Encode(float r, float g, float b, float a)
    {
        r = Math::Clamp(r, 0.0f, 1.0f);
        g = Math::Clamp(g, 0.0f, 1.0f);
        b = Math::Clamp(b, 0.0f, 1.0f);
        a = Math::Clamp(a, 0.0f, 1.0f);

        uint32_t ri = static_cast<uint32_t>(r * 255);
        uint32_t gi = static_cast<uint32_t>(g * 255);
        uint32_t bi = static_cast<uint32_t>(b * 255);
        uint32_t ai = static_cast<uint32_t>(a * 255);

        return (ri << 24) | (gi << 16) | (bi << 8) | ai;
    }
    
    uint32_t value;

    constexpr Color(float r, float g, float b) : Color(r, g, b, 1.0f) {}
    constexpr Color(float r, float g, float b, float a) : value(Encode(r, g, b, a)) {}

    static constexpr Color White()
    {
        return Color(1.0f, 1.0f, 1.0f);
    }

    static constexpr Color Black()
    {
        return Color(0.0f, 0.0f, 0.0f);
    }

    static constexpr Color Red()
    {
        return Color(1.0f, 0.0f, 0.0f);
    }

    static constexpr Color Green()
    {
        return Color(0.0f, 1.0f, 0.0f);
    }

    static constexpr Color Blue()
    {
        return Color(0.0f, 0.0f, 1.0f);
    }
};
// clang-format on

struct Quad
{
    float x, y;
    float w, h;
    Color color;
};

class Renderer
{
public:
    void Open();
    void Close();

    void DrawQuad(float x, float y, float w, float h);
    void DrawQuad(float x, float y, float w, float h, const Color& color);

    void Flush();

private:
    bool m_isOpen = false;
    std::vector<Quad> m_quadList;

    unsigned int m_quadVertexBuf;
    unsigned int m_quadIndexBuf;
    unsigned int m_quadArray;
};
