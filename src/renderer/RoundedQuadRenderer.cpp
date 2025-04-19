#include <cstdint>

#include "RoundedQuadRenderer.hpp"
#include "Color.hpp"
#include "Logger.hpp"

// clang-format off
RoundedQuadRenderer::RoundedQuadRenderer()
    : Renderer<RoundedQuad>("assets/shaders/rquad.wsh", VertexLayout::New({
        {
            .size = 2,
            .format = VertexFormat::FLOAT,
        },
        {
            .size = 2,
            .format = VertexFormat::FLOAT,
        },
        {
            .size = 2,
            .format = VertexFormat::FLOAT,
        },
        {
            .size = 1,
            .format = VertexFormat::FLOAT,
        },
        {
            .size = 1,
            .format = VertexFormat::UINT,
        }
    }))
{
    constexpr size_t BASE_SIZE = 16;
    m_quads.resize(BASE_SIZE);
    m_vertices.reserve(BASE_SIZE * 4);
    m_indices.reserve(BASE_SIZE * 6);
    WHIRL_DEBUG("Creating rounded quad renderer");
}
// clang-format on

RoundedQuadRenderer::~RoundedQuadRenderer()
{
    WHIRL_DEBUG("Deleting rounded quad renderer");
}

void RoundedQuadRenderer::Submit(const RoundedQuad& quad)
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

    if (quad.radius <= 0)
    {
        WHIRL_WARN("Invalid quad radius: ({})", quad.radius);
    }

    if (quad.w < (quad.radius * 2) || quad.h < (quad.radius * 2))
    {
        WHIRL_WARN("Radius is ({}) way too large for quad: ({}, {})", quad.radius, quad.w, quad.h);
        return;
    }

    m_quads.push_back(quad);
}

void RoundedQuadRenderer::Configure()
{
    m_vertices.clear();
    m_indices.clear();
    for (int i = 0; i < m_quads.size(); i++)
    {
        const auto& quad = m_quads[i];
        m_vertices.emplace_back(
            quad.x, quad.y,
            quad.w, quad.h,
            0.0f, quad.h,
            quad.radius, quad.color
        );
        m_vertices.emplace_back(
            quad.x, quad.y + quad.h,
            quad.w, quad.h,
            0.0f, 0.0f,
            quad.radius, quad.color
        );
        m_vertices.emplace_back(
            quad.x + quad.w, quad.y + quad.h,
            quad.w, quad.h,
            quad.w, 0.0f,
            quad.radius, quad.color
        );
        m_vertices.emplace_back(
            quad.x + quad.w, quad.y,
            quad.w, quad.h,
            quad.w, quad.h,
            quad.radius, quad.color
        );

        const size_t base = i * 4;
        m_indices.push_back(base + 0);
        m_indices.push_back(base + 1);
        m_indices.push_back(base + 2);

        m_indices.push_back(base + 2);
        m_indices.push_back(base + 3);
        m_indices.push_back(base + 0);
    }

    auto& vertexBuf = m_array->GetVertexBuffer();
    vertexBuf.Bind();
    vertexBuf.Data(m_vertices.data(), m_vertices.size() * sizeof(RoundedQuadVertex), DrawMode::DYNAMIC);

    auto& indexBuf = m_array->GetIndexBuffer();
    indexBuf.Bind();
    indexBuf.Data(m_indices.data(), m_indices.size() * sizeof(uint32_t), DrawMode::DYNAMIC);

    m_count = m_indices.size();
}

bool RoundedQuadRenderer::CanRender()
{
    return !m_quads.empty();
}

void RoundedQuadRenderer::Reset()
{
    return m_quads.clear();
}