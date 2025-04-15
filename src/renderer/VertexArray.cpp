#include <glad/gl.h>

#include "VertexArray.hpp"
#include "Logger.hpp"

VertexArray::VertexArray(VertexBuffer&& vertexBuf, IndexBuffer&& indexBuf, const std::vector<VertexAttribute>& layout)
    : m_vertexBuf(std::make_unique<VertexBuffer>(std::move(vertexBuf))),
      m_indexBuf(std::make_unique<IndexBuffer>(std::move(indexBuf))),
      m_layout(layout)
{
    glGenVertexArrays(1, &m_array);
    WHIRL_DEBUG("Creating vertex array: {}", m_array);

    // Configure
    WHIRL_TRACE("Configuring vertex array attributes...");
    glBindVertexArray(m_array);
    m_vertexBuf->Bind();
    m_indexBuf->Bind();

    for (int i = 0; i < layout.size(); i++)
    {
        const auto& attribute = layout[i];
        if (attribute.format == VertexFormat::FLOAT)
        {
            // clang-format off
            glVertexAttribPointer(
                i,
                attribute.size,
                VertexLayout::GetType(attribute.format),
                GL_FALSE,
                VertexLayout::GetStride(layout),
                (const void*) VertexLayout::GetOffset(layout, i)
            );
        }
        else
        {
            glVertexAttribIPointer(
                i,
                attribute.size,
                VertexLayout::GetType(attribute.format),
                VertexLayout::GetStride(layout),
                (const void*) VertexLayout::GetOffset(layout, i)
            );
            // clang-format on
        }

        glEnableVertexAttribArray(i);
    }
    
    glBindVertexArray(0);
    m_vertexBuf->Unbind();
    m_indexBuf->Unbind();
    WHIRL_TRACE("Configured vertex array attributes");
}

VertexArray::~VertexArray()
{
    if (m_array != 0)
    {
        WHIRL_DEBUG("Deleting vertex array: {}", m_array);
        glDeleteVertexArrays(1, &m_array);
    }
}

void VertexArray::Bind() const
{
    glBindVertexArray(m_array);
}

void VertexArray::Unbind() const
{
    glBindVertexArray(0);
}

uint32_t VertexArray::GetArray() const
{
    return m_array;
}

VertexBuffer& VertexArray::GetVertexBuffer() const
{
    return *m_vertexBuf;
}

IndexBuffer& VertexArray::GetIndexBuffer() const
{
    return *m_indexBuf;
}