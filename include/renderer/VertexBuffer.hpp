#pragma once

#include <cstdint>

#include "DrawMode.hpp"

class VertexBuffer
{
public:
    VertexBuffer();
    ~VertexBuffer();

    VertexBuffer(const VertexBuffer&) = delete;
    VertexBuffer(VertexBuffer&&) = delete;
    
    VertexBuffer& operator=(const VertexBuffer&) = delete;
    VertexBuffer& operator=(VertexBuffer&&) = delete;

public:
    void Data(const void* data, uint32_t size, DrawMode mode);
    void Bind() const;
    void Unbind() const;

private:
    uint32_t m_id = 0;
};