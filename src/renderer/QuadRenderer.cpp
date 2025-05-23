#include <cstdint>

#include "QuadRenderer.hpp"
#include "Logger.hpp"

// clang-format off
QuadRenderer::QuadRenderer()
    : Renderer<Quad>("assets/shaders/quad.wsh", VertexLayout::New({
        {
            .size = 2,
            .format = VertexFormat::FLOAT,
        },
        {
            .size = 1,
            .format = VertexFormat::UINT,
        }
    }))
{
    constexpr size_t BASE_SIZE = 16;
    m_quads.reserve(BASE_SIZE);
    m_vertices.reserve(BASE_SIZE * 4);
    m_indices.reserve(BASE_SIZE * 6);
    WHIRL_DEBUG("Creating quad renderer");
}
// clang-format on

QuadRenderer::~QuadRenderer()
{
    WHIRL_DEBUG("Deleting quad renderer");
}

void QuadRenderer::Submit(const Quad& quad)
{
    if (quad.x < 0 || quad.y < 0)
    {
        WHIRL_WARN("Invalid quad coordinates: ({}, {})", quad.x, quad.y);
        return;
    }

    if (quad.w <= 0 || quad.h <= 0)
    {
        WHIRL_WARN("Invalid quad dimensions: ({}, {})", quad.w, quad.h);
        return;
    }

    m_quads.push_back(quad);
}

void QuadRenderer::Configure()
{
    m_vertices.clear();
    m_indices.clear();
    for (int i = 0; i < m_quads.size(); i++)
    {
        const auto& quad = m_quads[i];

        // TODO: Ditch this approach and use instancing to not duplicate data
        m_vertices.emplace_back(quad.x, quad.y, quad.color);
        m_vertices.emplace_back(quad.x, quad.y + quad.h, quad.color);
        m_vertices.emplace_back(quad.x + quad.w, quad.y + quad.h, quad.color);
        m_vertices.emplace_back(quad.x + quad.w, quad.y, quad.color);

        const uint32_t base = i * 4;
        m_indices.push_back(base + 0);
        m_indices.push_back(base + 1);
        m_indices.push_back(base + 2);

        m_indices.push_back(base + 2);
        m_indices.push_back(base + 3);
        m_indices.push_back(base + 0);
    }

    auto& vertexBuf = m_array->GetVertexBuffer();
    vertexBuf.Bind();
    vertexBuf.Data(m_vertices.data(), m_vertices.size() * sizeof(QuadVertex), DrawMode::DYNAMIC);

    auto& indexBuf = m_array->GetIndexBuffer();
    indexBuf.Bind();
    indexBuf.Data(m_indices.data(), m_indices.size() * sizeof(uint32_t), DrawMode::DYNAMIC);

    m_count = m_indices.size();
}

bool QuadRenderer::CanRender()
{
    return !m_quads.empty();
}

void QuadRenderer::Reset()
{
    m_quads.clear();
}