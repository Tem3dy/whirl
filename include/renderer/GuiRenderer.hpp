#pragma once

#include <cstdint>
#include <vector>
#include <memory>

#include "Math.hpp"
#include "Color.hpp"
#include "Shader.hpp"
#include "VertexBuffer.hpp"
#include "IndexBuffer.hpp"
#include "VertexArray.hpp"

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

class GuiRenderer
{
public:
    GuiRenderer();
    ~GuiRenderer();
    
public:
    void DrawQuad(float x, float y, float w, float h, uint32_t color = Color::White());
    void DrawVLine(float x, float y, float length, float thickness, uint32_t color = Color::White());
    void DrawHLine(float x, float y, float length, float thickness, uint32_t color = Color::White());
    void Submit();

    void Adjust(int width, int height);

private:
    glm::mat4 m_projection;

    // Quads (abstract this away via a more simplified renderer later)
    std::vector<Quad> m_quadList;
    std::unique_ptr<Shader> m_quadShader;
    std::unique_ptr<VertexArray> m_quadArray;
};
