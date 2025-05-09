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
        m_quadRenderer = std::make_unique<QuadRenderer>();
        m_roundedQuadRenderer = std::make_unique<RoundedQuadRenderer>();
        m_circleRenderer = std::make_unique<CircleRenderer>();
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

void GuiRenderer::DrawQuad(float x, float y, float w, float h, uint32_t color)
{
    m_quadRenderer->Submit({x, y, w, h, color});
}

void GuiRenderer::DrawVLine(float x, float y, float length, float thickness, uint32_t color)
{
    DrawQuad(x, y, thickness, length, color);
}

void GuiRenderer::DrawHLine(float x, float y, float length, float thickness, uint32_t color)
{
    DrawQuad(x, y, length, thickness, color);
}

void GuiRenderer::DrawRoundedQuad(float x, float y, float w, float h, float radius, uint32_t color)
{
    m_roundedQuadRenderer->Submit({x, y, w, h, radius, color});
}

void GuiRenderer::DrawCircle(float x, float y, float radius, uint32_t color)
{
    m_circleRenderer->Submit({x, y, radius, color});
}

void GuiRenderer::Submit()
{
    m_quadRenderer->Draw(m_projection);
    m_roundedQuadRenderer->Draw(m_projection);
    m_circleRenderer->Draw(m_projection);
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