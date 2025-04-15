#include <cstdint>
#include <vector>
#include <stdexcept>

#include <glad/gl.h>

#include "Math.hpp"
#include "GuiRenderer.hpp"
#include "Logger.hpp"
#include "WhirlError.hpp"

GuiRenderer::GuiRenderer()
{
    try
    {
        // clang-format off
        m_quadRenderer = std::make_unique<QuadRenderer>("assets/shaders/quad.wsh", VertexLayout::New({
            {
             .size = 2,
             .format = VertexFormat::FLOAT,
            },
            {
             .size = 1,
             .format = VertexFormat::UINT,
            }
        }));
        // clang-format on
    }
    catch (WhirlError& error)
    {
        error.Context("Failed to create quad renderer");
        throw;
    }

    WHIRL_TRACE("Renderer opened successfully");
}

GuiRenderer::~GuiRenderer()
{
    WHIRL_TRACE("Renderer closed successfully");
}

void GuiRenderer::Submit()
{
    m_quadRenderer->Submit(m_projection);
}

void GuiRenderer::DrawQuad(float x, float y, float w, float h, uint32_t color)
{
    m_quadRenderer->Draw({x, y, w, h, color});
}

void GuiRenderer::DrawVLine(float x, float y, float length, float thickness, uint32_t color)
{
    DrawQuad(x, y, thickness, length, color);
}

void GuiRenderer::DrawHLine(float x, float y, float length, float thickness, uint32_t color)
{
    DrawQuad(x, y, length, thickness, color);
}

void GuiRenderer::Adjust(int width, int height)
{
    // Shouldn't happen, but just in case
    if (width <= 0 || height <= 0)
    {
        WHIRL_WARN("Invalid viewport data: ({}, {})", width, height);
        return;
    }

    WHIRL_DEBUG("Adjusting renderer: ({}, {})", width, height);
    // clang-format off
    m_projection = glm::ortho(
        0.0f,
        static_cast<float>(width),
        static_cast<float>(height),
        0.0f,
       -1.0f,
        1.0f
    );
    // clang-format on
}