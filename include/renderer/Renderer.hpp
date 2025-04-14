#pragma once

#include <cstdint>
#include <vector>
#include <memory>

#include "Math.hpp"
#include "Color.hpp"
#include "Shader.hpp"
#include "VertexBuffer.hpp"
#include "IndexBuffer.hpp"

// Ensure no padding
#pragma pack(1)
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
#pragma pack()
// Reset padding

class Renderer
{
public:
    bool Open();
    bool Close();
    bool Flush();

    void DrawQuad(float x, float y, float w, float h, uint32_t color = Color::White());
    void DrawVLine(float x, float y, float length, float thickness, uint32_t color = Color::White());
    void DrawHLine(float x, float y, float length, float thickness, uint32_t color = Color::White());
    
    void Adjust(int width, int height);

private:
    bool m_isOpen = false;
    glm::mat4 m_projection;

    // Quads
    Shader m_quadShader;
    std::vector<Quad> m_quadList;
    std::unique_ptr<VertexBuffer> m_quadVertexBuf;
    std::unique_ptr<IndexBuffer> m_quadIndexBuf;
    // Make this a std::unique_ptr<VertexArray> later on
    unsigned int m_quadArray;
};
