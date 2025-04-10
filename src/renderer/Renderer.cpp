#include <iostream>

#include <glad/gl.h>

#include "renderer/Renderer.hpp"
#include "renderer/Shader.hpp"

bool Renderer::Open()
{
    if (m_isOpen)
    {
        std::cerr << "ERROR: Tried to open a renderer that's already open" << std::endl;
        return false;
    }

    m_isOpen = true;
    // Initialize OpenGL resources
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
    return true;
}

void Renderer::DrawQuad(float x, float y, float w, float h, uint32_t color)
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to draw with a closed renderer" << std::endl;
        return;
    }
}
