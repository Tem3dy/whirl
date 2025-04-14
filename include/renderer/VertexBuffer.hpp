#pragma once

#include <cstdint>

#include "DrawMode.hpp"

class VertexBuffer
{
public:
    VertexBuffer();
    ~VertexBuffer();

    // Move
    VertexBuffer(VertexBuffer&& other) noexcept;
    VertexBuffer& operator=(VertexBuffer&& other) noexcept;

    // Copy
    VertexBuffer(const VertexBuffer&) = delete;
    VertexBuffer& operator=(const VertexBuffer&) = delete;

public:
    void Data(const void* data, uint32_t size, DrawMode mode);
    void Bind() const;
    void Unbind() const;

    uint32_t GetBuffer() const;

private:
    uint32_t m_buffer = 0;
};