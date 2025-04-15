#pragma once

#include <cstdint>
#include <memory>

#include "VertexBuffer.hpp"
#include "IndexBuffer.hpp"
#include "VertexLayout.hpp"

class VertexArray
{
public:
    VertexArray(const std::vector<VertexAttribute>& layout);
    VertexArray(VertexBuffer&& vertexBuf, IndexBuffer&& indexBuf, const std::vector<VertexAttribute>& layout);

    ~VertexArray();

    VertexArray(const VertexArray&) = delete;
    VertexArray(VertexArray&&) = delete;

    VertexArray& operator=(const VertexArray&) = delete;
    VertexArray& operator=(VertexArray&&) = delete;

public:
    void Bind() const;
    void Unbind() const;

    uint32_t GetArray() const;

    VertexBuffer& GetVertexBuffer() const;
    IndexBuffer& GetIndexBuffer() const;

private:
    uint32_t m_array = 0;
    std::unique_ptr<VertexBuffer> m_vertexBuf;
    std::unique_ptr<IndexBuffer> m_indexBuf;
    std::vector<VertexAttribute> m_layout;
};