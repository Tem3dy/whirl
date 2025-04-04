#include <iostream>

#include <GL/glew.h>

#include "renderer/Renderer.hpp"
#include "renderer/Shader.hpp"

void Renderer::Open()
{
    m_Open = true;

    // Initialize OpenGL resources
}

void Renderer::Close()
{
    m_Open = false;

    // Delete OpenGL resources
}

void Renderer::DrawQuad(float x, float y, float w, float h)
{
    if (!m_Open)
    {
        std::cerr << "Tried to draw with a closed renderer" << std::endl;
        return;
    }
}

void Renderer::DrawQuad(float x, float y, float w, float h, const Color& color)
{
    if (!m_Open)
    {
        std::cerr << "Tried to draw with a closed renderer" << std::endl;
        return;
    }
}

void Renderer::Flush()
{
    // Render and flush
}
