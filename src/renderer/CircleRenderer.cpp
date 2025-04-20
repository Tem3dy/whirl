#include <cstdint>

#include "CircleRenderer.hpp"
#include "Color.hpp"
#include "Logger.hpp"

// clang-format off
CircleRenderer::CircleRenderer()
    : Renderer<Circle>("assets/shaders/circle.wsh", VertexLayout::New({
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
    m_circles.resize(BASE_SIZE);
    m_vertices.reserve(BASE_SIZE * 4);
    m_indices.reserve(BASE_SIZE * 6);
    WHIRL_DEBUG("Creating circle renderer");
}
// clang-format on

CircleRenderer::~CircleRenderer()
{
    WHIRL_DEBUG("Deleting circle renderer");
}

void CircleRenderer::Submit(const Circle& circle)
{
    if (circle.radius <= 0)
    {
        WHIRL_WARN("Invalid circle radius: ({})", circle.radius);
        return;
    }

    m_circles.push_back(circle);
}

void CircleRenderer::Configure()
{
    m_vertices.clear();
    m_indices.clear();
    for (int i = 0; i < m_circles.size(); i++)
    {
        const auto& circle = m_circles[i];
        const float size = circle.radius * 2;
        const float centerX = circle.radius;
        const float centerY = circle.radius;

        // TODO: Ditch this approach and use instancing to not duplicate data
        m_vertices.emplace_back(
            circle.x - circle.radius, circle.y - circle.radius,
            centerX, centerY,
            0.0f, size,
            circle.radius,
            circle.color
        );

        m_vertices.emplace_back(
            circle.x - circle.radius, circle.y + circle.radius,
            centerX, centerY,
            0.0f, 0.0f,
            circle.radius,
            circle.color
        );

        m_vertices.emplace_back(
            circle.x + circle.radius, circle.y + circle.radius,
            centerX, centerY,
            size, 0.0f,
            circle.radius,
            circle.color
        );

        m_vertices.emplace_back(
            circle.x + circle.radius, circle.y - circle.radius,
            centerX, centerY,
            size, size,
            circle.radius,
            circle.color
        );

        const size_t base = i * 4;
        m_indices.push_back(base + 0);
        m_indices.push_back(base + 1);
        m_indices.push_back(base + 2);

        m_indices.push_back(base + 2);
        m_indices.push_back(base + 3);
        m_indices.push_back(base + 0);

        auto& vertexBuf = m_array->GetVertexBuffer();
        vertexBuf.Bind();
        vertexBuf.Data(m_vertices.data(), m_vertices.size() * sizeof(CircleVertex), DrawMode::DYNAMIC);

        auto& indexBuf = m_array->GetIndexBuffer();
        indexBuf.Bind();
        indexBuf.Data(m_indices.data(), m_indices.size() * sizeof(uint32_t), DrawMode::DYNAMIC);

        m_count = m_indices.size();
    }
}

bool CircleRenderer::CanRender()
{
    return !m_circles.empty();
}

void CircleRenderer::Reset()
{
    m_circles.clear();
}