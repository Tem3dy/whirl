#pragma once

#include <cstdint>

#include "DrawMode.hpp"

class IndexBuffer
{
public:
    IndexBuffer();
    ~IndexBuffer();

    IndexBuffer(const IndexBuffer&) = delete;
    IndexBuffer(IndexBuffer&&) = delete;

    IndexBuffer& operator=(const IndexBuffer&) = delete;
    IndexBuffer& operator=(IndexBuffer&&) = delete;

public:
    void Data(const void* data, uint32_t size, DrawMode mode);
    void Bind() const;
    void Unbind() const;

private:
    uint32_t m_id = 0;
};