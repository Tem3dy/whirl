#pragma once

#include <cstdint>
#include <vector>

#include "math/Math.hpp"
#include "renderer/Color.hpp"

struct Quad
{
    float x, y;
    float w, h;
    uint32_t color;
};

class Renderer
{
public:
    bool Open();
    bool Close();
    bool Flush();

    void DrawQuad(float x, float y, float w, float h, uint32_t color = Color::White());

private:
    bool m_isOpen = false;
    std::vector<Quad> m_quadList;

    unsigned int m_quadVertexBuf;
    unsigned int m_quadIndexBuf;
    unsigned int m_quadArray;
};
