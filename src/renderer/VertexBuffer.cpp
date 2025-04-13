#include <glad/gl.h>

#include "VertexBuffer.hpp"

VertexBuffer::VertexBuffer()
{
    glGenBuffers(1, &m_id);
}

VertexBuffer::~VertexBuffer()
{
    glDeleteBuffers(1, &m_id);
}

void VertexBuffer::Data(const void* data, uint32_t size, DrawMode mode)
{
    uint32_t usage;
    switch (mode)
    {
        case DrawMode::STATIC:
        {
            usage = GL_STATIC_DRAW;
            break;
        }

        case DrawMode::DYNAMIC:
        {
            usage = GL_DYNAMIC_DRAW;
            break;
        }

        case DrawMode::STREAM:
        {
            usage = GL_STREAM_DRAW;
            break;
        }

        default:
        {
            usage = GL_STATIC_DRAW;
        }
    }

    Bind();
    glBufferData(GL_ARRAY_BUFFER, size, data, usage);
    Unbind();
}

void VertexBuffer::Bind()
{
    glBindBuffer(GL_ARRAY_BUFFER, m_id);
}

void VertexBuffer::Unbind()
{
    glBindBuffer(GL_ARRAY_BUFFER, 0);
}