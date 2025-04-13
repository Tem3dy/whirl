#pragma once

#include <cstdint>

enum class DrawMode
{
    STATIC,
    DYNAMIC,
    STREAM,
};

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
    uint32_t m_id;
};