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
    Bind();
    glBufferData(GL_ARRAY_BUFFER, size, data, GetMode(mode));
    Unbind();
}

void VertexBuffer::Bind() const
{
    glBindBuffer(GL_ARRAY_BUFFER, m_id);
}

void VertexBuffer::Unbind() const
{
    glBindBuffer(GL_ARRAY_BUFFER, 0);
}

uint32_t VertexBuffer::GetBuffer() const
{
    return m_id;
}