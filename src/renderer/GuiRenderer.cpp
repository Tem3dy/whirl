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
        m_quadShader = std::make_unique<Shader>("assets/shaders/quad.wsh");
    }
    catch (WhirlError& error)
    {
        error.Context("Failed to load renderer shaders");
        throw;
    }

    m_quadList.reserve(64);
    // clang-format off
    m_quadArray = std::make_unique<VertexArray>(VertexLayout::New({
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
    WHIRL_TRACE("Renderer opened successfully");
}

GuiRenderer::~GuiRenderer()
{
    WHIRL_DEBUG("Clearing quad list");
    m_quadList.clear();
    WHIRL_TRACE("Renderer closed successfully");
}

void GuiRenderer::Submit()
{
    // TODO: Abstract all this stuff away into a proper renderer abstraction
    std::vector<QuadVertex> quadVertices;
    quadVertices.reserve(4 * m_quadList.size());
    std::vector<uint32_t> quadIndices;
    quadIndices.reserve(6 * m_quadList.size());

    for (int i = 0; i < m_quadList.size(); i++)
    {
        // Vertices
        const auto& quad = m_quadList[i];
        quadVertices.emplace_back(quad.x, quad.y, quad.color);
        quadVertices.emplace_back(quad.x, quad.y + quad.h, quad.color);
        quadVertices.emplace_back(quad.x + quad.w, quad.y + quad.h, quad.color);
        quadVertices.emplace_back(quad.x + quad.w, quad.y, quad.color);

        // Indices
        const uint32_t base = i * 4;
        quadIndices.push_back(base + 0);
        quadIndices.push_back(base + 1);
        quadIndices.push_back(base + 2);

        quadIndices.push_back(base + 2);
        quadIndices.push_back(base + 3);
        quadIndices.push_back(base + 0);
    }

    m_quadArray->Bind();
    auto& vertexBuf = m_quadArray->GetVertexBuffer();
    vertexBuf.Bind();
    vertexBuf.Data(quadVertices.data(), quadVertices.size() * sizeof(QuadVertex), DrawMode::DYNAMIC);

    auto& indexBuf = m_quadArray->GetIndexBuffer();
    indexBuf.Bind();
    indexBuf.Data(quadIndices.data(), quadIndices.size() * sizeof(uint32_t), DrawMode::DYNAMIC);

    m_quadShader->SetMat4("u_Projection", m_projection);
    m_quadShader->Use();
    glDrawElements(GL_TRIANGLES, quadIndices.size(), GL_UNSIGNED_INT, nullptr);
    m_quadArray->Unbind();
    vertexBuf.Unbind();
    indexBuf.Unbind();

    // Clear on submit
    m_quadList.clear();
}

void GuiRenderer::DrawQuad(float x, float y, float w, float h, uint32_t color)
{
    if (x < 0 || y < 0)
    {
        WHIRL_WARN("Invalid quad coordinates: ({}, {})", x, y);
        return;
    }

    if (w <= 0 || h <= 0)
    {
        WHIRL_WARN("Invalid quad dimensions: ({}, {})", w, h);
        return;
    }

    if (m_quadList.size() == m_quadList.capacity())
    {
        const size_t newCapacity = static_cast<size_t>(m_quadList.capacity() * 1.5);
        WHIRL_DEBUG("Reserving more memory for rendering quads: ({} -> {})", m_quadList.capacity(), newCapacity);
        // TODO: Potentially check for overflows (shouldn't happen though)
        m_quadList.reserve(newCapacity);
    }

    m_quadList.emplace_back(x, y, w, h, color);
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