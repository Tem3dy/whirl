#pragma once

#include <cstdint>
#include <vector>
#include <memory>

#include "Math.hpp"
#include "Color.hpp"
#include "QuadRenderer.hpp"

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

    // Renderers
    std::unique_ptr<QuadRenderer> m_quadRenderer;
};
