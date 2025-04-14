#include <glad/gl.h>

#include "IndexBuffer.hpp"

IndexBuffer::IndexBuffer()
{
    glGenBuffers(1, &m_id);
}

IndexBuffer::~IndexBuffer()
{
    if (m_id != 0)
        glDeleteBuffers(1, &m_id);
}

IndexBuffer::IndexBuffer(IndexBuffer&& other) noexcept
    : m_id(other.m_id)
{
    other.m_id = 0;
}

IndexBuffer& IndexBuffer::operator=(IndexBuffer&& other) noexcept
{
    if (this != &other)
    {
        m_id = other.m_id;
        other.m_id = 0;
    }

    return *this;
}

void IndexBuffer::Data(const void* data, uint32_t size, DrawMode mode)
{
    Bind();
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, size, data, GetMode(mode));
    Unbind();
}

void IndexBuffer::Bind() const
{
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_id);
}

void IndexBuffer::Unbind() const
{
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
}

uint32_t IndexBuffer::GetBuffer() const
{
    return m_id;
}