#pragma once

#include <cstdint>

#include "DrawMode.hpp"

class VertexBuffer
{
public:
    VertexBuffer();
    ~VertexBuffer();

public:
    void Data(const void* data, uint32_t size, DrawMode mode);
    void Bind();
    void Unbind();

private:
    uint32_t m_id = 0;
};