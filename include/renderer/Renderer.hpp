#pragma once

#include <cstdint>
#include <vector>

#include "Math.hpp"
#include "Color.hpp"
#include "Shader.hpp"

struct Quad
{
    float x, y;
    float w, h;
    uint32_t color;
};

struct QuadVertex
{
    float x, y;
    uint32_t color;
};

class Renderer
{
public:
    bool Open();
    bool Close();
    bool Flush();

    void DrawQuad(float x, float y, float w, float h, uint32_t color = Color::White());

    void Adjust(int width, int height);

private:
    bool m_isOpen = false;
    glm::mat4 m_projection;

    Shader m_quadShader;
    std::vector<Quad> m_quadList;
    unsigned int m_quadVertexBuf;
    unsigned int m_quadIndexBuf;
    unsigned int m_quadArray;
};
