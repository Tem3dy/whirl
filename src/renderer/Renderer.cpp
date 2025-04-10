#include <iostream>

#include <glad/gl.h>

#include "renderer/Renderer.hpp"
#include "renderer/Shader.hpp"

void Renderer::Open()
{
    if (m_isOpen)
    {
        std::cerr << "ERROR: Tried to open a renderer that's already open" << std::endl;
        return;
    }

    m_isOpen = true;

    // Initialize OpenGL resources
}

void Renderer::Close()
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to close a renderer that's already closed" << std::endl;
        return;
    }

    m_isOpen = false;

    // Delete OpenGL resources
}

void Renderer::DrawQuad(float x, float y, float w, float h)
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to draw with a closed renderer" << std::endl;
        return;
    }
}

void Renderer::DrawQuad(float x, float y, float w, float h, const Color& color)
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to draw with a closed renderer" << std::endl;
        return;
    }
}

void Renderer::Flush()
{
    if (!m_isOpen)
    {
        std::cerr << "ERROR: Tried to flush a closed renderer" << std::endl;
        return;
    }

    // Render and flush
}
