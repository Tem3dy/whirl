#include <iostream>

#include <glad/gl.h>

#include "renderer/Renderer.hpp"

bool Renderer::Open()
{
    if (m_isOpen)
    {
        std::cerr << "ERROR: Tried to open a renderer that's already open" << std::endl;
        return false;
    }

    if (!m_quadShader.Load("assets/shaders/quad.wsh"))
    {
        std::cerr << "ERROR: Failed to load the quad shader" << std::endl;
        return false;
    }

    m_isOpen = true;
    m_quadList.reserve(64);

    // Initialize OpenGL resources
    glGenVertexArrays(1, &m_quadArray);

    glGenBuffers(1, &m_quadVertexBuf);
    glGenBuffers(1, &m_quadIndexBuf);

    glBindVertexArray(m_quadArray);
    glBindBuffer(GL_ARRAY_BUFFER, m_quadVertexBuf);

    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, sizeof(QuadVertex), (const void*) 0);
    glEnableVertexAttribArray(0);

    glVertexAttribIPointer(1, 1, GL_UNSIGNED_INT, sizeof(QuadVertex), (const void*) (2 * sizeof(float)));
    glEnableVertexAttribArray(1);

    glBindVertexArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);

    std::cout << "INFO: Renderer opened successfully" << std::endl;
    return true;
}

bool Renderer::Close()
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to close a renderer that's already closed" << std::endl;
        return false;
    }

    m_isOpen = false;
    // Delete OpenGL resources
    glDeleteVertexArrays(1, &m_quadArray);
    glDeleteBuffers(1, &m_quadVertexBuf);
    glDeleteBuffers(1, &m_quadIndexBuf);
    m_quadList.clear();

    std::cout << "INFO: Rendered closed successfully" << std::endl;
    return true;
}

bool Renderer::Flush()
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to flush a closed renderer" << std::endl;
        return false;
    }

    // Render and flush
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

    glBindVertexArray(m_quadArray);

    glBindBuffer(GL_ARRAY_BUFFER, m_quadVertexBuf);
    glBufferData(GL_ARRAY_BUFFER, quadVertices.size() * sizeof(QuadVertex), quadVertices.data(), GL_DYNAMIC_DRAW);

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_quadIndexBuf);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, quadIndices.size() * sizeof(uint32_t), quadIndices.data(), GL_DYNAMIC_DRAW);

    m_quadShader.SetMat4("u_Projection", m_projection);
    m_quadShader.Use();
    glDrawElements(GL_TRIANGLES, quadIndices.size(), GL_UNSIGNED_INT, nullptr);

    glBindVertexArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

    m_quadList.clear();
    return true;
}

void Renderer::DrawQuad(float x, float y, float w, float h, uint32_t color)
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to draw with a closed renderer" << std::endl;
        return;
    }

    if (x < 0 || y < 0)
    {
        // This needs a logger badly
        std::cerr << "ERROR: Invalid quad coordinates ("
                  << x << ", " << y << ")" << std::endl;
        return;
    }

    if (w <= 0 || h <= 0)
    {
        // Same here
        std::cerr << "ERROR: Invalid quad dimensions ("
                  << w << ", " << h << ")" << std::endl;
        return;
    }

    if (m_quadList.size() == m_quadList.capacity())
    {
        // TODO: Should be DEBUG instead of INFO, log more details later on
        std::cout << "INFO: Reserving more memory for rendering quads" << std::endl;
        m_quadList.reserve(m_quadList.capacity() * 2);
    }

    m_quadList.emplace_back(x, y, w, h, color);
}

void Renderer::Adjust(int width, int height)
{
    if (width <= 0 || height <= 0)
    {
        std::cerr << "ERROR: Invalid viewport data (" << width << ", " << height << ")" << std::endl;
    }

    std::cout << "INFO: Adjusting renderer: (" << width << ", " << height << ")" << std::endl;
    m_projection = glm::ortho(
        0.0f,
        static_cast<float>(width),
        static_cast<float>(height),
        0.0f,
       -1.0f,
        1.0f
    );
}